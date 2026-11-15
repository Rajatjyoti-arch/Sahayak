"""
Offline Scheme Reconciliation Engine (V0 Reference Prototype)
Innovathon 2.0 | Central University of Jammu & University of Jammu (SIIEDC)

Operational Motto:
"AI suggests. Deterministic code calculates. The officer decides. Every material action is traceable."

Core Guarantees:
1. Strict integer paise arithmetic (zero IEEE 754 float drift)
2. Mutually exclusive row partitioning: N_A = M + U_A + D_A + I_A + P_A
3. Decoupling of record linkage from financial variance (amount mismatch doesn't disqualify match)
4. Multi-signal candidate generation (masked account suffix is supporting evidence, not unique key)
5. Tamper-evident SHA-256 hash-chained audit manifest
6. Structured multi-tab Excel workbook export
"""

import os
import sys
import csv
import json
import re
import math
import hashlib
from datetime import datetime, timezone
from decimal import Decimal
import difflib

# -------------------------------------------------------------------------
# 1. Strict Integer Paise Currency Parser
# -------------------------------------------------------------------------
# Using [0-9] and [.] completely eliminates backslash-escaping ambiguities across
# markdown renderers, LaTeX listings, shell scripts, and Python runtimes.
_AMOUNT_RE = re.compile(
    r"^(?:[0-9]+|[0-9]{1,3}(?:,[0-9]{3})+|[0-9]{1,2}(?:,[0-9]{2})*,[0-9]{3})"
    r"(?:[.][0-9]{1,2})?$"
)
_CURRENCY_PREFIX_RE = re.compile(
    r"^(?:₹|INR|Rs[.]?)\s*",
    re.IGNORECASE,
)

def parse_to_paise(amount_text: str) -> int:
    """Parse a non-negative currency string into integer paise.
    Accepted: '5000', '5000.50', '₹ 5,000.50', '1,00,000.00', 'Rs. 5000.50'.
    Rejects negatives, malformed comma groupings, multiple dots, and excess decimal precision.
    """
    if not isinstance(amount_text, str):
        raise TypeError("Expected currency string; handle numeric spreadsheet cells explicitly")

    cleaned = amount_text.strip()
    cleaned = _CURRENCY_PREFIX_RE.sub("", cleaned)
    cleaned = re.sub(r"\s+", "", cleaned)

    if not cleaned or not _AMOUNT_RE.fullmatch(cleaned):
        raise ValueError(f"Invalid currency amount: {amount_text!r}")

    normalized = cleaned.replace(",", "")
    rupees, dot, fractional = normalized.partition(".")
    paise_text = (fractional + "00")[:2] if dot else "00"
    return int(rupees) * 100 + int(paise_text)

def parse_cell_to_paise(val) -> int:
    """Parse spreadsheet cell value (str, int, float, Decimal) into integer paise safely.
    Preserves true source precision; rejects negatives, blanks, NaNs, infinities,
    and precision beyond 2 decimal places without silent rounding or truncation.
    """
    if val is None or val == "":
        raise ValueError("Empty or missing currency cell")
    if isinstance(val, bool):
        raise TypeError("Boolean values cannot be parsed as currency")
    if isinstance(val, int):
        if val < 0:
            raise ValueError(f"Negative currency amount not permitted: {val}")
        return val * 100
    if isinstance(val, str):
        return parse_to_paise(val)
    if isinstance(val, float):
        if math.isnan(val) or math.isinf(val):
            raise ValueError(f"Invalid numeric cell value (NaN/Inf): {val}")
        if val < 0:
            raise ValueError(f"Negative currency amount not permitted: {val}")
        # Convert float via exact string representation without artificial rounding
        d = Decimal(str(val))
    elif isinstance(val, Decimal):
        if val < 0:
            raise ValueError(f"Negative currency amount not permitted: {val}")
        d = val
    else:
        raise TypeError(f"Unsupported cell type: {type(val)}")

    # Reject excess decimal precision without silent truncation (e.g. 1.0000004 or 5000.999)
    if d.as_tuple().exponent < -2:
        raise ValueError(f"Excess precision beyond 2 decimal places: {val}")
    return int((d * 100).to_integral_value())

def format_paise(paise: int) -> str:
    """Format integer paise into Indian currency format (e.g. ₹ 5,000.50)."""
    sign = "-" if paise < 0 else ""
    abs_paise = abs(paise)
    rupees = abs_paise // 100
    rem_paise = abs_paise % 100
    
    # Format Indian comma grouping for rupees
    s = str(rupees)
    if len(s) > 3:
        last3 = s[-3:]
        other = s[:-3]
        groups = []
        while len(other) > 2:
            groups.insert(0, other[-2:])
            other = other[:-2]
        if other:
            groups.insert(0, other)
        groups.append(last3)
        formatted_rupees = ",".join(groups)
    else:
        formatted_rupees = s
    return f"{sign}₹ {formatted_rupees}.{rem_paise:02d}"

# -------------------------------------------------------------------------
# 2. Tamper-Evident SHA-256 Audit Logger
# -------------------------------------------------------------------------
class AuditLogger:
    def __init__(self, run_id: str, operator_label: str = "clerk_station_01"):
        self.run_id = run_id
        self.operator_label = operator_label
        self.events = []
        self.sequence_no = 0
        self.current_hash = "0" * 64  # Initial Genesis Hash

    def log_event(self, event_type: str, details: dict):
        self.sequence_no += 1
        timestamp = datetime.now(timezone.utc).isoformat()
        
        event_body = {
            "sequence_no": self.sequence_no,
            "run_id": self.run_id,
            "timestamp": timestamp,
            "operator_label": self.operator_label,
            "event_type": event_type,
            "details": details,
            "prev_hash": self.current_hash
        }
        
        canonical_json = json.dumps(event_body, sort_keys=True, separators=(',', ':'))
        payload = f"{self.current_hash}{canonical_json}".encode("utf-8")
        event_hash = hashlib.sha256(payload).hexdigest()
        
        event_record = {
            **event_body,
            "event_hash": event_hash
        }
        self.events.append(event_record)
        self.current_hash = event_hash
        return event_record

    def export_manifest(self, filepath: str):
        manifest = {
            "run_id": self.run_id,
            "operator_label": self.operator_label,
            "terminal_hash": self.current_hash,
            "total_events": len(self.events),
            "created_at": datetime.now(timezone.utc).isoformat(),
            "events": self.events
        }
        with open(filepath, "w", encoding="utf-8") as f:
            json.dump(manifest, f, indent=2)
        return manifest

# -------------------------------------------------------------------------
# 3. String & Evidence Matching Utilities
# -------------------------------------------------------------------------
def normalize_text(text: str) -> str:
    if not text:
        return ""
    t = text.lower().strip()
    t = re.sub(r"[^\w\s]", "", t)
    return re.sub(r"\s+", " ", t)

def name_similarity(n1: str, n2: str) -> float:
    s1, s2 = normalize_text(n1), normalize_text(n2)
    if not s1 or not s2:
        return 0.0
    if s1 == s2:
        return 1.0
    # Sequence matcher ratio
    return difflib.SequenceMatcher(None, s1, s2).ratio()

def parentage_agrees(p1: str, p2: str) -> tuple[bool, str]:
    norm1, norm2 = normalize_text(p1), normalize_text(p2)
    if not norm1 or not norm2:
        return (False, "MISSING_PARENTAGE")
    # Clean relationship prefixes
    clean1 = re.sub(r"^(s\/o|w\/o|d\/o|so|wo|do)\s*", "", norm1).strip()
    clean2 = re.sub(r"^(s\/o|w\/o|d\/o|so|wo|do)\s*", "", norm2).strip()
    
    # Common abbreviations: ab -> abdul, gh -> ghulam, mohd -> mohammad
    subs = {"ab": "abdul", "gh": "ghulam", "mohd": "mohammad", "md": "mohammad"}
    words1 = [subs.get(w, w) for w in clean1.split()]
    words2 = [subs.get(w, w) for w in clean2.split()]
    
    if words1 == words2:
        return (True, "EXACT_AGREEMENT")
    ratio = difflib.SequenceMatcher(None, " ".join(words1), " ".join(words2)).ratio()
    if ratio >= 0.80:
        return (True, f"FUZZY_AGREEMENT_{ratio:.2f}")
    return (False, f"CONFLICTING_PARENTAGE_{ratio:.2f}")

# -------------------------------------------------------------------------
# 4. Reconciliation Core Engine
# -------------------------------------------------------------------------
class SchemeReconciliationEngine:
    def __init__(self, audit_logger: AuditLogger):
        self.audit = audit_logger
        self.rows_a = []  # File A: Sanction List
        self.rows_b = []  # File B: Disbursement Log
        
        # Partitions for File A
        self.M_A = []   # Matched rows
        self.U_A = []   # Unmatched rows
        self.D_A = []   # Duplicate-excluded rows
        self.I_A = []   # Invalid rows
        self.P_A = []   # Pending-review rows
        
        # Partitions for File B
        self.M_B = []   # Matched rows
        self.U_B = []   # Unmatched rows
        self.D_B = []   # Duplicate-excluded rows
        self.I_B = []   # Invalid rows
        self.P_B = []   # Pending-review rows
        
        self.matched_pairs = []       # Confirmed pairs (Row A, Row B, variance_paise, flags)
        self.pending_candidates = []   # Unresolved pairs (Row A, Row B, score, rationale, flags)

    def load_files(self, path_a: str, path_b: str):
        # Hash source files
        with open(path_a, "rb") as f:
            hash_a = hashlib.sha256(f.read()).hexdigest()
        with open(path_b, "rb") as f:
            hash_b = hashlib.sha256(f.read()).hexdigest()
            
        self.audit.log_event("SOURCE_FILES_LOADED", {
            "file_a_path": path_a, "file_a_sha256": hash_a,
            "file_b_path": path_b, "file_b_sha256": hash_b
        })
        
        # Read File A (Sanction List)
        with open(path_a, newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            for idx, r in enumerate(reader):
                r["_idx"] = idx
                r["_source"] = "SANCTION_LIST"
                self.rows_a.append(r)
                
        # Read File B (Disbursement Log)
        with open(path_b, newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            for idx, r in enumerate(reader):
                r["_idx"] = idx
                r["_source"] = "DISBURSEMENT_LOG"
                self.rows_b.append(r)

        self.audit.log_event("INGESTION_COMPLETED", {
            "total_sanction_rows": len(self.rows_a),
            "total_disbursement_rows": len(self.rows_b)
        })

    def run_reconciliation(self, candidate_name_threshold: float = 0.82):
        """Execute deterministic matching, candidate generation, and row partitioning."""
        
        # Step 1: Validate amounts & syntax -> Partition INVALID_FORMAT
        valid_a, valid_b = [], []
        
        for r in self.rows_a:
            try:
                paise = parse_cell_to_paise(r.get("sanctioned_amount"))
                r["_amount_paise"] = paise
                valid_a.append(r)
            except Exception as e:
                r["_error"] = str(e)
                r["primary_status"] = "INVALID_FORMAT"
                self.I_A.append(r)
                self.audit.log_event("ROW_INVALID_FORMAT", {"row_id": r.get("source_row_id"), "file": "A", "error": str(e)})

        for r in self.rows_b:
            try:
                paise = parse_cell_to_paise(r.get("disbursed_amount"))
                r["_amount_paise"] = paise
                valid_b.append(r)
            except Exception as e:
                r["_error"] = str(e)
                r["primary_status"] = "INVALID_FORMAT"
                self.I_B.append(r)
                self.audit.log_event("ROW_INVALID_FORMAT", {"row_id": r.get("source_row_id"), "file": "B", "error": str(e)})

        # Step 2: Canonical Duplicate Detection -> Partition DUPLICATE_EXCLUDED
        seen_keys_a = {}
        non_dup_a = []
        for r in valid_a:
            app_no = r.get("application_no", "").strip()
            name_norm = normalize_text(r.get("beneficiary_name", ""))
            suffix = r.get("masked_account_ref", "").strip()
            dup_key = (app_no, name_norm, suffix) if app_no else (name_norm, suffix)
            if dup_key in seen_keys_a:
                r["primary_status"] = "DUPLICATE_EXCLUDED"
                r["_duplicate_of"] = seen_keys_a[dup_key]["source_row_id"]
                self.D_A.append(r)
                self.audit.log_event("ROW_DUPLICATE_EXCLUDED", {"row_id": r.get("source_row_id"), "file": "A", "first_seen": r["_duplicate_of"]})
            else:
                seen_keys_a[dup_key] = r
                non_dup_a.append(r)

        seen_keys_b = {}
        non_dup_b = []
        for r in valid_b:
            app_no = r.get("application_no", "").strip()
            name_norm = normalize_text(r.get("beneficiary_name", ""))
            suffix = r.get("masked_account_ref", "").strip()
            dup_key = (app_no, name_norm, suffix) if app_no else (name_norm, suffix)
            if dup_key in seen_keys_b:
                r["primary_status"] = "DUPLICATE_EXCLUDED"
                r["_duplicate_of"] = seen_keys_b[dup_key]["source_row_id"]
                self.D_B.append(r)
                self.audit.log_event("ROW_DUPLICATE_EXCLUDED", {"row_id": r.get("source_row_id"), "file": "B", "first_seen": r["_duplicate_of"]})
            else:
                seen_keys_b[dup_key] = r
                non_dup_b.append(r)

        # Step 3: Exact Matching on Validated Strong Keys
        # Strong key: Application Number present and matching in both
        unmatched_a = []
        matched_b_ids = set()

        for ra in non_dup_a:
            app_a = ra.get("application_no", "").strip()
            matched_rb = None
            if app_a:
                # Find in non_dup_b
                for rb in non_dup_b:
                    if rb["source_row_id"] in matched_b_ids:
                        continue
                    if rb.get("application_no", "").strip() == app_a:
                        matched_rb = rb
                        break
            
            if matched_rb:
                # Confirmed exact match
                ra["primary_status"] = "MATCHED"
                matched_rb["primary_status"] = "MATCHED"
                matched_b_ids.add(matched_rb["source_row_id"])
                
                variance = ra["_amount_paise"] - matched_rb["_amount_paise"]
                flags = []
                if variance != 0:
                    flags.append("AMOUNT_MISMATCH")
                
                pair = {
                    "row_a": ra, "row_b": matched_rb,
                    "match_type": "EXACT_APPLICATION_NO",
                    "variance_paise": variance,
                    "flags": flags
                }
                self.M_A.append(ra)
                self.M_B.append(matched_rb)
                self.matched_pairs.append(pair)
                self.audit.log_event("EXACT_MATCH_CONFIRMED", {
                    "row_a_id": ra["source_row_id"],
                    "row_b_id": matched_rb["source_row_id"],
                    "variance_paise": variance
                })
            else:
                unmatched_a.append(ra)

        remaining_b = [rb for rb in non_dup_b if rb["source_row_id"] not in matched_b_ids]

        # Step 4: Multi-Signal Candidate Generation (with experimental threshold)
        # CRITICAL RULES:
        # 1. Masked account suffix is SUPPORTING signal, NEVER sufficient alone.
        # 2. Missing parentage != agreement. Conflicting parentage surfaces.
        # 3. Amount variance does NOT disqualify candidate.
        
        pending_a_ids = set()
        pending_b_ids = set()

        for ra in unmatched_a:
            best_candidate = None
            best_score = 0.0
            best_rationale = ""
            best_flags = []

            for rb in remaining_b:
                if rb["source_row_id"] in pending_b_ids:
                    continue

                name_score = name_similarity(ra.get("beneficiary_name", ""), rb.get("beneficiary_name", ""))
                p_agrees, p_reason = parentage_agrees(ra.get("parentage", ""), rb.get("parentage", ""))
                
                suffix_a = ra.get("masked_account_ref", "").strip()
                suffix_b = rb.get("masked_account_ref", "").strip()
                suffix_match = (suffix_a and suffix_b and suffix_a == suffix_b)

                # Check for Suffix Collision between different people
                # If suffix matches but name similarity is low (< 0.5) and parentage conflicts, REJECT
                if suffix_match and name_score < 0.50 and not p_agrees:
                    continue

                # Qualifying candidate criteria:
                # High name similarity (>= threshold) AND corroborating signal (parentage OR suffix)
                if name_score >= candidate_name_threshold:
                    corroborating = 0
                    rationale_parts = [f"Name similarity: {name_score:.2f}"]
                    
                    if p_agrees:
                        corroborating += 1
                        rationale_parts.append(f"Parentage ({p_reason})")
                    if suffix_match:
                        corroborating += 1
                        rationale_parts.append(f"Account suffix ({suffix_a})")

                    if corroborating >= 1:
                        score = name_score + (0.15 * corroborating)
                        if score > best_score:
                            best_score = score
                            best_candidate = rb
                            
                            flags = []
                            # Decouple linkage from financial reconciliation!
                            variance = ra["_amount_paise"] - rb["_amount_paise"]
                            if variance != 0:
                                flags.append("AMOUNT_MISMATCH")
                            if "CONFLICTING" in p_reason:
                                flags.append("PARENTAGE_CONFLICT")
                            
                            best_flags = flags
                            best_rationale = " + ".join(rationale_parts)

            if best_candidate and best_score >= candidate_name_threshold:
                ra["primary_status"] = "PENDING_REVIEW"
                best_candidate["primary_status"] = "PENDING_REVIEW"
                pending_a_ids.add(ra["source_row_id"])
                pending_b_ids.add(best_candidate["source_row_id"])
                
                variance = ra["_amount_paise"] - best_candidate["_amount_paise"]
                cand_record = {
                    "row_a": ra, "row_b": best_candidate,
                    "score": round(best_score, 3),
                    "rationale": best_rationale,
                    "variance_paise": variance,
                    "flags": best_flags,
                    "status": "PENDING_REVIEW"
                }
                self.P_A.append(ra)
                self.P_B.append(best_candidate)
                self.pending_candidates.append(cand_record)
                self.audit.log_event("CANDIDATE_GENERATED", {
                    "row_a_id": ra["source_row_id"],
                    "row_b_id": best_candidate["source_row_id"],
                    "score": cand_record["score"],
                    "rationale": best_rationale,
                    "flags": best_flags
                })

        # Step 5: Remaining rows -> Partition UNMATCHED
        for ra in unmatched_a:
            if ra["source_row_id"] not in pending_a_ids:
                ra["primary_status"] = "UNMATCHED"
                self.U_A.append(ra)
                self.audit.log_event("ROW_UNMATCHED", {"row_id": ra["source_row_id"], "file": "A"})

        for rb in remaining_b:
            if rb["source_row_id"] not in pending_b_ids:
                rb["primary_status"] = "UNMATCHED"
                self.U_B.append(rb)
                self.audit.log_event("ROW_UNMATCHED", {"row_id": rb["source_row_id"], "file": "B"})

    def adjudicate_candidate(self, candidate_idx: int, decision: str, notes: str = ""):
        """Adjudicate candidate in the human review queue.
        Decisions: 'APPROVE', 'REJECT', 'FLAG_INQUIRY'
        """
        cand = self.pending_candidates[candidate_idx]
        ra, rb = cand["row_a"], cand["row_b"]
        
        if decision == "APPROVE":
            # Move from PENDING to MATCHED
            cand["status"] = "APPROVED_BY_OFFICER"
            ra["primary_status"] = "MATCHED"
            rb["primary_status"] = "MATCHED"
            self.P_A.remove(ra)
            self.P_B.remove(rb)
            self.M_A.append(ra)
            self.M_B.append(rb)
            
            pair = {
                "row_a": ra, "row_b": rb,
                "match_type": "FUZZY_OFFICER_APPROVED",
                "variance_paise": cand["variance_paise"],
                "flags": cand["flags"] + ["OFFICER_APPROVED"]
            }
            self.matched_pairs.append(pair)
            self.audit.log_event("OFFICER_ADJUDICATION_APPROVED", {
                "row_a_id": ra["source_row_id"],
                "row_b_id": rb["source_row_id"],
                "operator": self.audit.operator_label,
                "notes": notes
            })
        elif decision == "REJECT":
            # Keep separate -> Move from PENDING to UNMATCHED
            cand["status"] = "REJECTED_BY_OFFICER"
            ra["primary_status"] = "UNMATCHED"
            rb["primary_status"] = "UNMATCHED"
            self.P_A.remove(ra)
            self.P_B.remove(rb)
            self.U_A.append(ra)
            self.U_B.append(rb)
            self.audit.log_event("OFFICER_ADJUDICATION_REJECTED", {
                "row_a_id": ra["source_row_id"],
                "row_b_id": rb["source_row_id"],
                "operator": self.audit.operator_label,
                "notes": notes
            })
        elif decision == "FLAG_INQUIRY":
            cand["status"] = "FLAGGED_FOR_FIELD_INQUIRY"
            self.audit.log_event("OFFICER_ADJUDICATION_FLAGGED", {
                "row_a_id": ra["source_row_id"],
                "row_b_id": rb["source_row_id"],
                "operator": self.audit.operator_label,
                "notes": notes
            })

    def assert_row_conservation_invariants(self) -> dict:
        """Validate mathematical row conservation:
        N_A = M + U_A + D_A + I_A + P_A
        N_B = M + U_B + D_B + I_B + P_B
        """
        N_A = len(self.rows_a)
        N_B = len(self.rows_b)
        
        M_count_A = len(self.M_A)
        M_count_B = len(self.M_B)
        assert M_count_A == M_count_B == len(self.matched_pairs), "Matched pair counts must be identical!"
        
        sum_A = len(self.M_A) + len(self.U_A) + len(self.D_A) + len(self.I_A) + len(self.P_A)
        sum_B = len(self.M_B) + len(self.U_B) + len(self.D_B) + len(self.I_B) + len(self.P_B)
        
        assert sum_A == N_A, f"File A Row Invariant Violated! {sum_A} != {N_A}"
        assert sum_B == N_B, f"File B Row Invariant Violated! {sum_B} != {N_B}"
        
        res = {
            "File_A_Total": N_A,
            "File_A_Partition": {
                "Matched": len(self.M_A), "Unmatched": len(self.U_A),
                "Duplicates": len(self.D_A), "Invalid": len(self.I_A), "Pending": len(self.P_A)
            },
            "File_B_Total": N_B,
            "File_B_Partition": {
                "Matched": len(self.M_B), "Unmatched": len(self.U_B),
                "Duplicates": len(self.D_B), "Invalid": len(self.I_B), "Pending": len(self.P_B)
            },
            "Invariant_Status": "MATHEMATICALLY_CONSERVED (100% Verified)"
        }
        self.audit.log_event("ROW_CONSERVATION_VERIFIED", res)
        return res

    def calculate_financial_bridge(self) -> dict:
        """Calculate deterministic financial totals across accounting buckets in integer paise."""
        # Sum valid canonical records (excluding invalid & duplicates)
        canonical_valid_a = self.M_A + self.U_A + self.P_A
        canonical_valid_b = self.M_B + self.U_B + self.P_B
        
        total_sanctioned_paise = sum(r["_amount_paise"] for r in canonical_valid_a)
        total_disbursed_paise = sum(r["_amount_paise"] for r in canonical_valid_b)
        
        matched_variance_paise = sum(p["variance_paise"] for p in self.matched_pairs)
        unmatched_sanction_paise = sum(r["_amount_paise"] for r in self.U_A)
        unmatched_disbursed_paise = sum(r["_amount_paise"] for r in self.U_B)
        pending_sanction_paise = sum(r["_amount_paise"] for r in self.P_A)
        pending_disbursed_paise = sum(r["_amount_paise"] for r in self.P_B)
        
        # Financial Accounting Bridge Equation:
        # Sum(S) - Sum(D) = Matched_Variance + S_unmatched + S_pending - D_unmatched - D_pending
        lhs = total_sanctioned_paise - total_disbursed_paise
        rhs = (matched_variance_paise + unmatched_sanction_paise + pending_sanction_paise 
               - unmatched_disbursed_paise - pending_disbursed_paise)
        
        assert lhs == rhs, f"Financial Bridge Discrepancy! {lhs} != {rhs}"
        
        bridge = {
            "Total_Sanctioned": format_paise(total_sanctioned_paise),
            "Total_Disbursed": format_paise(total_disbursed_paise),
            "Net_Scheme_Difference": format_paise(lhs),
            "Breakdown": {
                "Matched_Pair_Net_Discrepancy": format_paise(matched_variance_paise),
                "Unmatched_Sanction_Total": format_paise(unmatched_sanction_paise),
                "Unmatched_Disbursed_Total": format_paise(unmatched_disbursed_paise),
                "Pending_Review_Sanction_Total": format_paise(pending_sanction_paise),
                "Pending_Review_Disbursed_Total": format_paise(pending_disbursed_paise)
            },
            "Paise_Bridge_Exact_Check": "VERIFIED_ZERO_FLOAT_ERROR"
        }
        self.audit.log_event("FINANCIAL_BRIDGE_CALCULATED", bridge)
        return bridge

    def export_excel_report(self, output_path: str):
        """Export comprehensive multi-tab Excel reconciliation report using openpyxl."""
        import openpyxl
        from openpyxl.styles import Font, PatternFill, Alignment, Border, Side
        from openpyxl.utils import get_column_letter

        wb = openpyxl.Workbook()
        
        # Styling palettes: Clean, professional GovTech aesthetic
        header_fill = PatternFill(start_color="1F4E79", end_color="1F4E79", fill_type="solid")
        header_font = Font(name="Calibri", size=11, bold=True, color="FFFFFF")
        sub_header_fill = PatternFill(start_color="D9E1F2", end_color="D9E1F2", fill_type="solid")
        sub_header_font = Font(name="Calibri", size=11, bold=True, color="000000")
        accent_fill = PatternFill(start_color="FFF2CC", end_color="FFF2CC", fill_type="solid") # Warning / Mismatch
        border_thin = Border(
            left=Side(style='thin', color='D9D9D9'), right=Side(style='thin', color='D9D9D9'),
            top=Side(style='thin', color='D9D9D9'), bottom=Side(style='thin', color='D9D9D9')
        )

        # ----------------------------------------------------
        # Tab 1: Executive Summary
        # ----------------------------------------------------
        ws1 = wb.active
        ws1.title = "Executive Summary"
        ws1.views.sheetView[0].showGridLines = True
        
        inv = self.assert_row_conservation_invariants()
        fin = self.calculate_financial_bridge()
        
        ws1.append(["OFFLINE GOVERNANCE ASSISTANT -- SCHEME RECONCILIATION REPORT"])
        ws1.append(["Generated by:", "Deterministic Core Engine v1.1"])
        ws1.append(["Terminal Audit Hash:", self.audit.current_hash])
        ws1.append(["Operator Label:", self.audit.operator_label])
        ws1.append(["Timestamp (UTC):", datetime.now(timezone.utc).isoformat()])
        ws1.append([])
        
        ws1.append(["1. ROW CONSERVATION & PARTITION INVARIANTS"])
        ws1.append(["Partition Bucket", "File A (Sanction List)", "File B (Disbursement Log)", "Verification Status"])
        for cell in ws1[ws1.max_row]:
            cell.fill = header_fill
            cell.font = header_font
            
        pA, pB = inv["File_A_Partition"], inv["File_B_Partition"]
        rows_inv = [
            ("Confirmed Matched Rows (M)", pA["Matched"], pB["Matched"], "1-to-1 Balanced"),
            ("Unmatched Rows (U)", pA["Unmatched"], pB["Unmatched"], "Verified Isolated"),
            ("Excluded Duplicates (D)", pA["Duplicates"], pB["Duplicates"], "Canonical Policy Applied"),
            ("Invalid Format Rows (I)", pA["Invalid"], pB["Invalid"], "Flagged for Correction"),
            ("Pending Review Queue (P)", pA["Pending"], pB["Pending"], "Awaiting Officer Adjudication"),
            ("TOTAL ROW COUNT (N)", inv["File_A_Total"], inv["File_B_Total"], "100% Conserved (N = M+U+D+I+P)")
        ]
        for r in rows_inv:
            ws1.append(list(r))
            
        ws1.append([])
        ws1.append(["2. FINANCIAL RECONCILIATION BRIDGE (INTEGER PAISE ACCURACY)"])
        ws1.append(["Metric / Accounting Head", "Amount (INR)", "Operational Significance"])
        for cell in ws1[ws1.max_row]:
            cell.fill = header_fill
            cell.font = header_font
            
        ws1.append(["Total Sanctioned (Canonical Valid)", fin["Total_Sanctioned"], "Net entitlement budget"])
        ws1.append(["Total Disbursed (Canonical Valid)", fin["Total_Disbursed"], "Actual bank credits disbursed"])
        ws1.append(["Net Scheme Variance (Total S - Total D)", fin["Net_Scheme_Difference"], "Total ledger divergence"])
        ws1.append(["  ↳ Matched-Pair Net Discrepancy", fin["Breakdown"]["Matched_Pair_Net_Discrepancy"], "Amount variances on linked beneficiaries"])
        ws1.append(["  ↳ Unmatched Sanction Total", fin["Breakdown"]["Unmatched_Sanction_Total"], "Sanctioned benefits not yet credited"])
        ws1.append(["  ↳ Unmatched Disbursed Total", fin["Breakdown"]["Unmatched_Disbursed_Total"], "Bank credits without matching sanction"])
        ws1.append(["  ↳ Pending Review Sanctions", fin["Breakdown"]["Pending_Review_Sanction_Total"], "Ambiguous candidates pending review"])
        ws1.append(["  ↳ Pending Review Disbursements", fin["Breakdown"]["Pending_Review_Disbursed_Total"], "Ambiguous bank credits pending review"])

        # ----------------------------------------------------
        # Tab 2: Confirmed Matched Pairs
        # ----------------------------------------------------
        ws2 = wb.create_sheet(title="Confirmed Matches")
        ws2.views.sheetView[0].showGridLines = True
        headers_m = ["Pair ID", "Match Type", "Sanction App No", "Sanction Beneficiary", "Bank Beneficiary", "Parentage (Sanction)", "Account Suffix", "Sanctioned (INR)", "Disbursed (INR)", "Variance (INR)", "Flags / Audit Note"]
        ws2.append(headers_m)
        for cell in ws2[1]:
            cell.fill = header_fill
            cell.font = header_font
            
        for idx, p in enumerate(self.matched_pairs, 1):
            ra, rb = p["row_a"], p["row_b"]
            row_data = [
                f"PAIR-{idx:03d}",
                p["match_type"],
                ra.get("application_no", ""),
                ra.get("beneficiary_name", ""),
                rb.get("beneficiary_name", ""),
                ra.get("parentage", ""),
                ra.get("masked_account_ref", ""),
                format_paise(ra["_amount_paise"]),
                format_paise(rb["_amount_paise"]),
                format_paise(p["variance_paise"]),
                ", ".join(p["flags"]) if p["flags"] else "CLEAN_MATCH"
            ]
            ws2.append(row_data)
            if p["variance_paise"] != 0:
                # Highlight variance row
                ws2.cell(row=ws2.max_row, column=10).fill = accent_fill

        # ----------------------------------------------------
        # Tab 3: Pending Review Queue
        # ----------------------------------------------------
        ws3 = wb.create_sheet(title="Pending Review Queue")
        ws3.views.sheetView[0].showGridLines = True
        headers_p = ["Candidate ID", "Match Score", "Matching Rationale", "Sanction Row ID", "Sanction Name", "Bank Row ID", "Bank Name", "Sanction Amount", "Disbursement Amount", "Variance", "Reviewer Status"]
        ws3.append(headers_p)
        for cell in ws3[1]:
            cell.fill = header_fill
            cell.font = header_font
            
        for idx, c in enumerate(self.pending_candidates, 1):
            ra, rb = c["row_a"], c["row_b"]
            ws3.append([
                f"CAND-{idx:03d}",
                c["score"],
                c["rationale"],
                ra.get("source_row_id", ""),
                ra.get("beneficiary_name", ""),
                rb.get("source_row_id", ""),
                rb.get("beneficiary_name", ""),
                format_paise(ra["_amount_paise"]),
                format_paise(rb["_amount_paise"]),
                format_paise(c["variance_paise"]),
                c["status"]
            ])

        # ----------------------------------------------------
        # Tab 4: Unmatched Sanction Records
        # ----------------------------------------------------
        ws4 = wb.create_sheet(title="Unmatched Sanctions")
        ws4.views.sheetView[0].showGridLines = True
        headers_uA = ["Source Row ID", "Application No", "Beneficiary Name", "Parentage", "Masked Account", "IFSC", "Sanctioned Amount", "Tehsil"]
        ws4.append(headers_uA)
        for cell in ws4[1]:
            cell.fill = header_fill
            cell.font = header_font
        for ra in self.U_A:
            ws4.append([
                ra.get("source_row_id", ""), ra.get("application_no", ""), ra.get("beneficiary_name", ""),
                ra.get("parentage", ""), ra.get("masked_account_ref", ""), ra.get("bank_ifsc", ""),
                format_paise(ra["_amount_paise"]), ra.get("tehsil", "")
            ])

        # ----------------------------------------------------
        # Tab 5: Unmatched Disbursements
        # ----------------------------------------------------
        ws5 = wb.create_sheet(title="Unmatched Disbursements")
        ws5.views.sheetView[0].showGridLines = True
        headers_uB = ["Source Row ID", "Application No", "Beneficiary Name", "Parentage", "Masked Account", "IFSC", "Disbursed Amount", "Transaction Ref"]
        ws5.append(headers_uB)
        for cell in ws5[1]:
            cell.fill = header_fill
            cell.font = header_font
        for rb in self.U_B:
            ws5.append([
                rb.get("source_row_id", ""), rb.get("application_no", ""), rb.get("beneficiary_name", ""),
                rb.get("parentage", ""), rb.get("masked_account_ref", ""), rb.get("bank_ifsc", ""),
                format_paise(rb["_amount_paise"]), rb.get("transaction_ref", "")
            ])

        # ----------------------------------------------------
        # Tab 6: Excluded Duplicates
        # ----------------------------------------------------
        ws6 = wb.create_sheet(title="Excluded Duplicates")
        ws6.views.sheetView[0].showGridLines = True
        headers_d = ["Source File", "Source Row ID", "Beneficiary Name", "Application No", "Account Suffix", "Amount", "Duplicate Of Row ID"]
        ws6.append(headers_d)
        for cell in ws6[1]:
            cell.fill = header_fill
            cell.font = header_font
        for ra in self.D_A:
            ws6.append(["Sanction List (File A)", ra.get("source_row_id", ""), ra.get("beneficiary_name", ""), ra.get("application_no", ""), ra.get("masked_account_ref", ""), format_paise(ra["_amount_paise"]), ra.get("_duplicate_of", "")])
        for rb in self.D_B:
            ws6.append(["Disbursement Log (File B)", rb.get("source_row_id", ""), rb.get("beneficiary_name", ""), rb.get("application_no", ""), rb.get("masked_account_ref", ""), format_paise(rb["_amount_paise"]), rb.get("_duplicate_of", "")])

        # ----------------------------------------------------
        # Tab 7: Invalid Format Records
        # ----------------------------------------------------
        ws7 = wb.create_sheet(title="Invalid Format Rows")
        ws7.views.sheetView[0].showGridLines = True
        headers_i = ["Source File", "Source Row ID", "Beneficiary Name", "Raw Amount Text", "Parser Error Reason"]
        ws7.append(headers_i)
        for cell in ws7[1]:
            cell.fill = header_fill
            cell.font = header_font
        for ra in self.I_A:
            ws7.append(["Sanction List (File A)", ra.get("source_row_id", ""), ra.get("beneficiary_name", ""), str(ra.get("sanctioned_amount")), ra.get("_error", "")])
        for rb in self.I_B:
            ws7.append(["Disbursement Log (File B)", rb.get("source_row_id", ""), rb.get("beneficiary_name", ""), str(rb.get("disbursed_amount")), rb.get("_error", "")])

        # Auto-adjust column widths for all worksheets
        for sheet in wb.worksheets:
            for col in sheet.columns:
                max_len = max(len(str(cell.value or '')) for cell in col)
                col_letter = get_column_letter(col[0].column)
                sheet.column_dimensions[col_letter].width = max(max_len + 3, 12)

        wb.save(output_path)
        self.audit.log_event("EXCEL_REPORT_EXPORTED", {"output_filepath": output_path})
        print(f"Reconciliation workbook successfully exported to {output_path}")

# -------------------------------------------------------------------------
# 5. Standalone Execution Demonstration
# -------------------------------------------------------------------------
if __name__ == "__main__":
    run_id = f"RUN-{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}"
    logger = AuditLogger(run_id=run_id, operator_label="clerk_station_01")
    
    engine = SchemeReconciliationEngine(logger)
    engine.load_files("data/synthetic_sanction_list.csv", "data/synthetic_disbursement_log.csv")
    engine.run_reconciliation(candidate_name_threshold=0.82)
    
    print("\n--- INITIAL RECONCILIATION SUMMARY ---")
    invariants = engine.assert_row_conservation_invariants()
    print(json.dumps(invariants, indent=2))
    
    print(f"\nFound {len(engine.pending_candidates)} pending review candidates.")
    # Demonstrate Human Adjudication on candidate 0 (Mohd Iqbal vs Mohammad Iqbal)
    if engine.pending_candidates:
        cand0 = engine.pending_candidates[0]
        print(f"Adjudicating Candidate 0: {cand0['row_a']['beneficiary_name']} <-> {cand0['row_b']['beneficiary_name']}")
        print(f"Rationale: {cand0['rationale']}")
        engine.adjudicate_candidate(0, "APPROVE", notes="Clerk verified transliteration and matching parentage/suffix")
    
    print("\n--- POST-ADJUDICATION ROW INVARIANTS ---")
    invariants_post = engine.assert_row_conservation_invariants()
    print(json.dumps(invariants_post, indent=2))
    
    print("\n--- DETERMINISTIC FINANCIAL BRIDGE ---")
    fin_bridge = engine.calculate_financial_bridge()
    print(json.dumps(fin_bridge, indent=2))
    
    # Export Excel and Manifest
    os.makedirs("output", exist_ok=True)
    engine.export_excel_report("output/reconciliation_audit_report.xlsx")
    manifest = logger.export_manifest("output/audit_manifest.json")
    print(f"\nTerminal Audit Hash: {manifest['terminal_hash']}")
