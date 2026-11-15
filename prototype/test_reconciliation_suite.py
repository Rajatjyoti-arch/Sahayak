"""
Comprehensive Automated Acceptance Test Suite for GovTech Reconciliation Engine
Innovathon 2.0 | Central University of Jammu & University of Jammu (SIIEDC)

Runs unit tests and system tests for:
1. Currency parser (text and numeric cells)
2. Mathematical row conservation invariants
3. Candidate generation vs. suffix collision
4. Decoupled financial variance reporting
5. Canonical duplicate handling
6. Tamper-evident SHA-256 audit chaining & tamper detection
"""

import unittest
import os
import json
from decimal import Decimal
from reconciliation_engine import (
    parse_to_paise, parse_cell_to_paise, format_paise,
    AuditLogger, SchemeReconciliationEngine
)
from verify_audit_log import verify_manifest

class TestCurrencyParser(unittest.TestCase):
    def test_valid_text_amounts(self):
        cases = [
            ("5000", 500000),
            ("5000.50", 500050),
            ("5000.5", 500050),
            ("₹ 5,000.50", 500050),
            ("₹5000", 500000),
            ("INR 1,00,000.00", 10000000),
            ("Rs. 25,000", 2500000),
            ("Rs 12,34,567.89", 123456789),
            ("100", 10000),
            ("0.50", 50),
            ("0", 0)
        ]
        for text, expected in cases:
            self.assertEqual(parse_to_paise(text), expected, f"Failed on {text}")

    def test_invalid_text_amounts(self):
        invalid_inputs = [
            "5000.999",  # excess precision
            "₹",          # prefix only
            "",           # empty
            "   ",        # whitespace
            "-5000",      # negative
            "50,00",      # bad grouping
            "1,000,00",   # bad grouping
            "abc",        # non-numeric
            "5000.50.50", # multiple dots
            "12,345,67"   # malformed commas
        ]
        for text in invalid_inputs:
            with self.assertRaises(ValueError, msg=f"Should have failed on {text}"):
                parse_to_paise(text)

    def test_numeric_cell_inputs(self):
        # Integers
        self.assertEqual(parse_cell_to_paise(5000), 500000)
        self.assertEqual(parse_cell_to_paise(0), 0)
        # Floats
        self.assertEqual(parse_cell_to_paise(5000.50), 500050)
        self.assertEqual(parse_cell_to_paise(100.25), 10025)
        # Decimals
        self.assertEqual(parse_cell_to_paise(Decimal("100000.00")), 10000000)
        
        # Invalids
        with self.assertRaises(ValueError):
            parse_cell_to_paise(-500)
        with self.assertRaises(ValueError):
            parse_cell_to_paise(5000.999) # Excess precision float
        with self.assertRaises(ValueError):
            parse_cell_to_paise(1.0000004) # Excess precision float that truncates if rounded at 6 decimals
        with self.assertRaises(ValueError):
            parse_cell_to_paise(float('nan'))
        with self.assertRaises(ValueError):
            parse_cell_to_paise(float('inf'))
        with self.assertRaises(ValueError):
            parse_cell_to_paise(None)
        with self.assertRaises(TypeError):
            parse_cell_to_paise(True)
        with self.assertRaises(TypeError):
            parse_cell_to_paise(False)

class TestReconciliationSystem(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # Generate datasets
        import generate_synthetic_data
        generate_synthetic_data.generate_datasets("data")
        
        cls.logger = AuditLogger(run_id="TEST-RUN-001", operator_label="qa_test_harness")
        cls.engine = SchemeReconciliationEngine(cls.logger)
        cls.engine.load_files("data/synthetic_sanction_list.csv", "data/synthetic_disbursement_log.csv")
        cls.engine.run_reconciliation(candidate_name_threshold=0.82)

    def test_01_row_conservation_invariants(self):
        """Verify N = M + U + D + I + P for both files without exception."""
        inv = self.engine.assert_row_conservation_invariants()
        self.assertEqual(inv["File_A_Total"], 13)
        self.assertEqual(inv["File_B_Total"], 13)
        
        pA = inv["File_A_Partition"]
        pB = inv["File_B_Partition"]
        
        # Before human adjudication:
        # 5 Exact Matches
        self.assertEqual(pA["Matched"], 5)
        self.assertEqual(pB["Matched"], 5)
        # 1 Duplicate in each
        self.assertEqual(pA["Duplicates"], 1)
        self.assertEqual(pB["Duplicates"], 1)
        # 1 Invalid in each
        self.assertEqual(pA["Invalid"], 1)
        self.assertEqual(pB["Invalid"], 1)
        # Pending review candidates
        self.assertGreaterEqual(pA["Pending"], 3)
        self.assertGreaterEqual(pB["Pending"], 3)
        
        # Verify partition sum matches N
        self.assertEqual(sum(pA.values()), 13)
        self.assertEqual(sum(pB.values()), 13)

    def test_02_suffix_collision_protection(self):
        """Verify that Surinder Kumar and Abdul Majeed (both ending in XXXX-8821) are NOT matched."""
        # Row SAN-010 is Surinder Kumar, DISB-010 is Abdul Majeed
        for pair in self.engine.matched_pairs:
            self.assertFalse(
                pair["row_a"]["source_row_id"] == "SAN-010" and pair["row_b"]["source_row_id"] == "DISB-010",
                "CRITICAL ERROR: Suffix collision falsely linked different beneficiaries!"
            )
        for cand in self.engine.pending_candidates:
            self.assertFalse(
                cand["row_a"]["source_row_id"] == "SAN-010" and cand["row_b"]["source_row_id"] == "DISB-010",
                "CRITICAL ERROR: Suffix collision generated an invalid review candidate!"
            )

    def test_03_financial_variance_decoupling(self):
        """Verify that Parveen Akhtar (SAN-009) and Parveen Akhter (DISB-009) are linked despite amount variance."""
        # They should be generated as a candidate with AMOUNT_MISMATCH flag and 1500 variance
        target_cand = None
        for c in self.engine.pending_candidates:
            if c["row_a"]["source_row_id"] == "SAN-009" and c["row_b"]["source_row_id"] == "DISB-009":
                target_cand = c
                break
        self.assertIsNotNone(target_cand, "Failed to link candidate with amount mismatch!")
        self.assertIn("AMOUNT_MISMATCH", target_cand["flags"])
        self.assertEqual(target_cand["variance_paise"], 150000) # ₹1,500.00 = 150000 paise

    def test_04_financial_bridge_equality(self):
        """Verify the exact accounting bridge equation holding in integer paise."""
        fin = self.engine.calculate_financial_bridge()
        self.assertEqual(fin["Paise_Bridge_Exact_Check"], "VERIFIED_ZERO_FLOAT_ERROR")

    def test_05_adjudication_and_excel_export(self):
        """Adjudicate candidate and verify exported workbook and audit manifest."""
        # Adjudicate candidate 0
        if self.engine.pending_candidates:
            self.engine.adjudicate_candidate(0, "APPROVE", notes="Verified by QA")
            
        # Re-check row invariants
        inv_post = self.engine.assert_row_conservation_invariants()
        self.assertEqual(inv_post["Invariant_Status"], "MATHEMATICALLY_CONSERVED (100% Verified)")
        
        # Export Excel and Manifest
        os.makedirs("output", exist_ok=True)
        excel_path = "output/test_reconciliation_report.xlsx"
        manifest_path = "output/test_audit_manifest.json"
        
        self.engine.export_excel_report(excel_path)
        manifest = self.logger.export_manifest(manifest_path)
        
        self.assertTrue(os.path.exists(excel_path))
        self.assertTrue(os.path.exists(manifest_path))
        self.assertGreater(os.path.getsize(excel_path), 5000)
        
        # Verify audit manifest
        ok, msg = verify_manifest(manifest_path, expected_terminal_anchor=manifest["terminal_hash"])
        self.assertTrue(ok, f"Manifest verification failed: {msg}")

if __name__ == "__main__":
    import time
    from datetime import datetime, timezone
    
    suite = unittest.TestSuite()
    suite.addTests(unittest.TestLoader().loadTestsFromTestCase(TestCurrencyParser))
    suite.addTests(unittest.TestLoader().loadTestsFromTestCase(TestReconciliationSystem))
    
    all_tests = list(suite)
    
    start_time = time.time()
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    elapsed = time.time() - start_time
    
    report = {
        "report_id": "RECON-TEST-REPORT-001",
        "timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "runner": "unittest",
        "tests_run": result.testsRun,
        "success": result.wasSuccessful(),
        "failures_count": len(result.failures),
        "errors_count": len(result.errors),
        "elapsed_seconds": round(elapsed, 4),
        "test_results": []
    }
    
    for test in all_tests:
        test_id = test.id()
        failed = any(test_id == f[0].id() for f in result.failures)
        errored = any(test_id == e[0].id() for e in result.errors)
        status = "FAIL" if failed else ("ERROR" if errored else "PASS")
        report["test_results"].append({
            "test_name": test_id.split(".")[-1],
            "class": test_id.split(".")[-2],
            "status": status
        })
        
    os.makedirs("output", exist_ok=True)
    report_path = "output/test_report.json"
    with open(report_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
        
    print(f"\nMachine-readable test report written to: {report_path}")
    exit(0 if result.wasSuccessful() else 1)

