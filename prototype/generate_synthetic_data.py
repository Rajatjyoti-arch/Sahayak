"""
Generate deterministic, labeled synthetic datasets for Innovathon 2.0 GovTech Scheme Reconciliation.
Zero live PII. Completely synthetic names, account suffixes, and amounts designed to test:
1. Clean 1-to-1 exact matches
2. Transliteration / phonetic name variants with corroborating evidence
3. Amount mismatch on valid candidate link (separation of linkage from reconciliation)
4. Suffix collision across different people (suffix is supporting, not unique key)
5. Duplicate detection in sanction list and disbursement log
6. Unmatched sanction rows and unmatched disbursement rows
7. Invalid / malformed currency strings and excess precision
"""

import os
import csv

def generate_datasets(output_dir="data"):
    os.makedirs(output_dir, exist_ok=True)
    
    sanction_file = os.path.join(output_dir, "synthetic_sanction_list.csv")
    disbursement_file = os.path.join(output_dir, "synthetic_disbursement_log.csv")
    ground_truth_file = os.path.join(output_dir, "ground_truth_labels.json")

    # File A: Departmental Sanction List (Old Age Pension Scheme)
    # Headers: source_row_id, application_no, beneficiary_name, parentage, masked_account_ref, bank_ifsc, sanctioned_amount, tehsil
    sanction_rows = [
        # Clean exact matches
        ["SAN-001", "SCH-2026-001", "Ramesh Sharma", "S/o Som Nath", "XXXX-1024", "SBIN0001234", "5000.00", "Samba"],
        ["SAN-002", "SCH-2026-002", "Sunita Devi", "W/o Vijay Kumar", "XXXX-5521", "JAKA0SAMBA1", "₹ 5,000.00", "Samba"],
        ["SAN-003", "SCH-2026-003", "Ghulam Hassan Mir", "S/o Wali Mohd", "XXXX-9012", "JAKA0BDO001", "4000.00", "Vijaypur"],
        ["SAN-004", "SCH-2026-004", "Pooja Verma", "D/o Satish Verma", "XXXX-3341", "PUNB0123400", "3000.50", "Ghagwal"],
        ["SAN-005", "SCH-2026-005", "Kartar Singh", "S/o Amar Singh", "XXXX-7719", "SBIN0004567", "10,000.00", "Ramgarh"],
        
        # Name spelling / transliteration variation (Corroborating parentage + suffix)
        ["SAN-006", "SCH-2026-006", "Mohd Iqbal", "S/o Abdul Rashid", "XXXX-4812", "SBIN0001234", "4000.00", "Samba"],
        ["SAN-007", "SCH-2026-007", "Tariq Ahmed Rather", "S/o Ghulam Qadir", "XXXX-6623", "JAKA0SAMBA1", "5000.00", "Vijaypur"],
        ["SAN-008", "SCH-2026-008", "Meenakshi Kumari", "D/o Kuldeep Raj", "XXXX-8104", "SBIN0001234", "₹ 5,000.00", "Ghagwal"],
        
        # Amount mismatch on valid link (Sanctioned 5000 vs Disbursed 3500)
        ["SAN-009", "SCH-2026-009", "Parveen Akhtar", "W/o Bashir Ahmed", "XXXX-9128", "JAKA0BDO001", "5000.00", "Ramgarh"],
        
        # Suffix collision: Ramesh Gupta has XXXX-8821
        ["SAN-010", "SCH-2026-010", "Surinder Kumar", "S/o Hans Raj", "XXXX-8821", "SBIN0001234", "5000.00", "Samba"],
        
        # Duplicate row in Sanction List (duplicate of SAN-001)
        ["SAN-011", "SCH-2026-001", "Ramesh Sharma", "S/o Som Nath", "XXXX-1024", "SBIN0001234", "5000.00", "Samba"],
        
        # Unmatched in Sanction List (No disbursement occurred)
        ["SAN-012", "SCH-2026-012", "Bishan Dass", "S/o Mool Raj", "XXXX-4411", "SBIN0001234", "5000.00", "Samba"],
        
        # Invalid format: Excess precision 5000.999
        ["SAN-013", "SCH-2026-013", "Neelam Sharma", "W/o Ashok Kumar", "XXXX-3912", "SBIN0001234", "5000.999", "Vijaypur"]
    ]

    # File B: Bank / PFMS Disbursement Log
    # Headers: source_row_id, application_no, beneficiary_name, parentage, masked_account_ref, bank_ifsc, disbursed_amount, transaction_ref
    disbursement_rows = [
        # Clean exact matches
        ["DISB-001", "SCH-2026-001", "Ramesh Sharma", "S/o Som Nath", "XXXX-1024", "SBIN0001234", "5000.00", "UTR-SYN-001"],
        ["DISB-002", "SCH-2026-002", "Sunita Devi", "W/o Vijay Kumar", "XXXX-5521", "JAKA0SAMBA1", "5000.00", "UTR-SYN-002"],
        ["DISB-003", "SCH-2026-003", "Ghulam Hassan Mir", "S/o Wali Mohd", "XXXX-9012", "JAKA0BDO001", "4000.00", "UTR-SYN-003"],
        ["DISB-004", "SCH-2026-004", "Pooja Verma", "D/o Satish Verma", "XXXX-3341", "PUNB0123400", "3000.50", "UTR-SYN-004"],
        ["DISB-005", "SCH-2026-005", "Kartar Singh", "S/o Amar Singh", "XXXX-7719", "SBIN0004567", "10,000.00", "UTR-SYN-005"],
        
        # Transliteration variations in Bank Record (App No empty or bank-internal ref)
        ["DISB-006", "", "Mohammad Iqbal", "S/o Ab. Rashid", "XXXX-4812", "SBIN0001234", "4000.00", "UTR-SYN-006"],
        ["DISB-007", "", "Tariq Ahmad Rather", "S/o Gh Qadir", "XXXX-6623", "JAKA0SAMBA1", "5000.00", "UTR-SYN-007"],
        ["DISB-008", "", "Minakshi Kumari", "D/o Kuldeep Raj", "XXXX-8104", "SBIN0001234", "5000.00", "UTR-SYN-008"],
        
        # Amount mismatch on valid link (Disbursed 3500 vs Sanctioned 5000)
        ["DISB-009", "", "Parveen Akhter", "W/o Bashir Ahmad", "XXXX-9128", "JAKA0BDO001", "3500.00", "UTR-SYN-009"],
        
        # Suffix collision: Abdul Majeed also has XXXX-8821, completely different name
        ["DISB-010", "", "Abdul Majeed", "S/o Farooq Ahmed", "XXXX-8821", "SBIN0001234", "5000.00", "UTR-SYN-010"],
        
        # Duplicate row in Disbursement Log (duplicate credit of DISB-002)
        ["DISB-011", "SCH-2026-002", "Sunita Devi", "W/o Vijay Kumar", "XXXX-5521", "JAKA0SAMBA1", "5000.00", "UTR-SYN-002-DUP"],
        
        # Unmatched in Disbursement Log (bank credit without sanction order)
        ["DISB-012", "SCH-2026-099", "Vikram Singh", "S/o Karan Singh", "XXXX-7102", "JAKA0SAMBA1", "5000.00", "UTR-SYN-099"],
        
        # Invalid format: Negative amount -2500
        ["DISB-013", "SCH-2026-017", "Manzoor Ahmed", "S/o Ali Mohd", "XXXX-1100", "JAKA0BDO001", "-2500.00", "UTR-SYN-017"]
    ]

    with open(sanction_file, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["source_row_id", "application_no", "beneficiary_name", "parentage", "masked_account_ref", "bank_ifsc", "sanctioned_amount", "tehsil"])
        writer.writerows(sanction_rows)

    with open(disbursement_file, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["source_row_id", "application_no", "beneficiary_name", "parentage", "masked_account_ref", "bank_ifsc", "disbursed_amount", "transaction_ref"])
        writer.writerows(disbursement_rows)

    print(f"Generated {len(sanction_rows)} sanction rows in {sanction_file}")
    print(f"Generated {len(disbursement_rows)} disbursement rows in {disbursement_file}")

if __name__ == "__main__":
    generate_datasets()
