# COMPREHENSIVE MASTER BUSINESS PLAN & TECHNICAL DOSSIER

## Offline Governance Assistant
### First Product Module: Scheme Reconciliation Assistant
**Sub-Track:** AI for Intelligent Governance, Security & Public Services (Track 1)  
**Hackathon Platform:** Innovathon 2.0 | University of Jammu | Skill Incubation, Innovation and Entrepreneurship Development Centre (SIIEDC)  
**Institutional Alignment:** Central University of Jammu (CUJ) & University of Jammu (JU)  
**National Initiatives:** In support of grassroots digital public infrastructure under Viksit Bharat @2047 & Mission YUVA  
**Document Classification:** Confidential Startup Business Plan, Systems Architecture & Financial Projections  
**Status:** Version 1.1 (Rigorous Baseline) — Aligned with Master Source of Truth Brief  

---

```
========================================================================================
                                 DOCUMENT STRUCTURE
========================================================================================
SECTION 1: EXECUTIVE SUMMARY & THREE LEVELS OF SCOPE (VISION, PLATFORM, V0 MVP)
SECTION 2: PROBLEM HYPOTHESIS & COMPETITIVE DIFFERENTIATION (VS. EXCEL & SCRIPTS)
SECTION 3: LOCKED V0 PROTOTYPE SCOPE (THE RECONCILIATION ENGINE)
SECTION 4: DATA CONTRACT & MATCHING POLICY (SEPARATING LINKAGE FROM RECONCILIATION)
SECTION 5: MATHEMATICAL INVARIANTS & STRICT INTEGER PAISE PARSER
SECTION 6: TECHNICAL ARCHITECTURE & LOCAL INFERENCE CANDIDATE (QWEN2.5-1.5B)
SECTION 7: CYBERSECURITY, AUDIT TRAILS & ON-DEVICE DATA MINIMIZATION (DPDP CONTEXT)
SECTION 8: GROUNDED COMMERCIAL MODEL (OFFICE STARTER PACK & GFR 149 CAUTION)
SECTION 9: ITEMIZED PROTOTYPE VALIDATION BUDGET (THE REALISTIC ₹50,000 ASK)
SECTION 10: RECONCILED 3-YEAR OPERATING SCENARIO & BOTTOM-UP MARKET SIZING
SECTION 11: INSTITUTIONAL PILOT PLAYBOOK (CUJ + JU SIIEDC ACADEMIC BRIDGE)
SECTION 12: TECHNICAL ACCEPTANCE CRITERIA, KNOWN LIMITATIONS & RISK REGISTER
========================================================================================
```

---

## 1. Executive Summary & Three Levels of Scope

### 1.1 The Core Proposition
Public administrative offices (Tehsils, Block Development Offices / BDOs, and Municipal Wards) handle citizen welfare disbursements and records under tight operational deadlines, legacy computer hardware, and strict data confidentiality obligations. 

Rather than promising an unconstrained general artificial intelligence that replaces human officers, the venture builds an **offline-first governance software platform** around a focused, demonstrable first product: the **Scheme Reconciliation Assistant**.

The central operational principle governing the entire product is:

> **AI suggests. Deterministic code calculates. The officer decides. Every material action is traceable.**

---

### 1.2 Clear Separation of Scope Across Three Horizons

To maintain complete credibility before government evaluators and hackathon juries, our capabilities are strictly partitioned into three distinct horizons:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                 THREE LEVELS OF SCOPE                                  │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ LEVEL 1: STRATEGIC VISION (3 to 5 Years)                                               │
│ • A local-first, privacy-conscious AI assistant for public administration.             │
│ • Long-term ambition: Assist authorized staff in processing fragmented records into    │
│   structured, auditable work products across multiple administrative workflows.       │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ LEVEL 2: REUSABLE PRODUCT PLATFORM (6 to 24 Months)                                   │
│ • Reusable desktop runtime: local inference adapter, deterministic execution tools,   │
│   human review/adjudication UI, and tamper-evident audit-chain infrastructure.         │
│ • Future modules (added only after independent testing): Document extraction,         │
│   circular search, archival retrieval, and administrative drafting.                    │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ LEVEL 3: LOCKED V0 PROTOTYPE / FIRST MVP (Innovathon 2.0 Focus)                        │
│ • Offline Scheme Reconciliation Assistant: Ingest two spreadsheets (sanction list vs  │
│   bank disbursement report), perform deterministic matching, flag exceptions, present  │
│   ambiguous candidates in a human review queue, and export an audited Excel workbook.  │
│ • The MVP is designed for local execution without cloud inference. Compatibility with │
│   8 GB RAM workstations, end-to-end performance, and network-egress behaviour will be │
│   validated through documented tests before deployment claims are made.                │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Problem Hypothesis & Competitive Differentiation

### 2.1 Problem Hypothesis to Validate with Users
In grassroots offices across Jammu & Kashmir and India, staff routinely compare records received from different sources:
1. **Departmental Sanction Lists:** Generated by administrative departments (e.g., Social Welfare, Rural Development), listing sanctioned beneficiaries, parentage, bank references, and sanction amounts.
2. **Bank Disbursement / PFMS Reports:** Generated by commercial banks or the Public Financial Management System (PFMS), listing actual bank credits, failure codes, and disbursement amounts.

**The Working Hypothesis:** Manual row-by-row reconciliation across large datasets creates a recurring clerical burden, increases the risk of undetected disbursement errors, and complicates audit defense when records do not reconcile cleanly. 

*Field Discovery Milestone:* The team will validate the actual files involved, typical record volumes, baseline task times, and recurring error categories through structured interviews with administrative staff during the pilot discovery phase.

---

### 2.2 Why Not Just Use Microsoft Excel or a Python Script?
Judges and technical officers will rightfully ask: *"Why build a packaged application when an experienced user can use VLOOKUP, Power Query, or a custom script?"*

Our competitive advantage is grounded in operational usability and governance:

| Solution | What It Does Well | Operational Bottlenecks in Administration | Packaged Appliance Advantage |
| :--- | :--- | :--- | :--- |
| **Excel / Power Query** | Flexible, ubiquitous, and familiar to office staff. | High maintenance burden; formulas can be accidentally edited or dragged incorrectly; silent truncation of leading zeroes in account numbers (`0045` $\rightarrow$ `45`); lacks tamper-evident audit logs. | **Packaged Workflow:** Rigid execution rules; raw source files remain read-only; zero formula maintenance for clerks. |
| **Custom Python Script** | Fast and flexible for data engineers. | Requires developer execution; CLI errors panic non-technical clerks; no visual review queue for ambiguous records; lacks process governance across workstations. | **Clerk-Friendly UI:** Native desktop GUI; zero coding required; visual adjudication cards for non-programmers. |
| **Commercial Cloud AI** | Broad generative capabilities and language translation. | In remote blocks with unstable connectivity, cloud access is unreliable; departmental data-handling policies may restrict uploading citizen records to external multi-tenant cloud services. | **Local-First Processing:** Designed for on-premise execution; operational without cloud inference; external network behavior verified by audit. |

> *"The problem is not that matching records is impossible in Excel. The problem is that the same sensitive workflow must be executed consistently, transparently, and auditably by clerical staff who should not have to build, debug, and maintain their own data-processing tools."*

---

## 3. Locked V0 Prototype Scope (The Reconciliation Engine)

To guarantee a reliable, testable live demonstration during the 36-hour hackathon, the V0 MVP boundary is strictly locked:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                              LOCKED V0 PROTOTYPE BOUNDARY                              │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ INCLUDED IN V0 (THE CORE HERO WORKFLOW):                                               │
│  1. File Ingestion: Drag-and-drop 2 CSV/XLSX files; source files remain read-only.     │
│  2. Schema Mapping: Deterministic alias dictionary first; local model suggests unknown │
│     headers; user explicitly confirms mapping before processing.                       │
│  3. Normalization: Strict string preservation; whitespace trimming without coercion.   │
│  4. Exact Matching: Multi-key deterministic matching on validated identifiers.        │
│  5. Candidate Generation: Multi-signal fuzzy candidate generation (never auto-approved)│
│  6. Exception Review Queue: Visual cards displaying conflicting fields & match reasons;│
│     clerk explicitly chooses [Approve as Match], [Reject], or [Flag for Inquiry].      │
│  7. Financial Integrity: Strict parsing of currency into integer paise; zero float math.│
│  8. Audit Export: Structured Excel workbook + SHA-256 hash-chained event manifest.     │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ DELIBERATELY EXCLUDED FROM V0 (ROADMAP ONLY):                                          │
│  • Note-sheet (*Tippani*) generation (Stretch feature only; outside critical demo).   │
│  • Scanned document OCR, petition classification, or circular archival search.         │
│  • Autonomous beneficiary approval, payment initiation, or live portal API access.     │
│  • Use of actual citizen Aadhaar data during demonstrations.                           │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Data Contract & Matching Policy

### 4.1 Synthetic Data Contract (Zero Live PII Risk)
The V0 prototype and hackathon demo use **strictly synthetic data** with known expected outcomes. The canonical schema contains:

| Canonical Field | Example Synthetic Value | Governance Handling Rule |
| :--- | :--- | :--- |
| `source_row_id` | `SAN-01042` | Stable internal ID generated by the engine during run. |
| `sanction_id` / `application_no` | `SCH-2026-01042` | Strong key if both files share identical system semantics. |
| `beneficiary_name` | `MOHD IQBAL` | Raw text preserved; normalized string stored separately. |
| `parentage` / `relation_name` | `ABDUL RASHID` | Supporting identity signal; handles common abbreviations. |
| `masked_account_ref` | `XXXX-4812` | **Supporting signal only; never treated as a unique key.** |
| `bank_ifsc` | `SBIN0001234` | Format-checked; not proof of account ownership. |
| `sanctioned_amount` | `4000.00` | Strictly parsed to integer paise (`400000`). |
| `disbursed_amount` | `4000.00` | Strictly parsed to integer paise (`400000`). |
| `transaction_ref` / `pfms_ref` | `UTR-SYN-00881` | Bank transaction reference from disbursement log. |
| `primary_status` | `PENDING_REVIEW` | Exactly one primary disposition assigned per row. |

---

### 4.2 Canonical Reconciliation Unit & Cardinality Boundary (Locked V0 MVP)
To establish exact mathematical determinism, the V0 prototype locks a concrete first-workflow reconciliation unit:
1. **File A (Departmental Sanction List):** Each row represents a **single beneficiary entitlement sanction for the declared payment cycle** (e.g., Monthly Welfare Pension Sanction Order, where each sanctioned beneficiary case appears exactly once per cycle).
2. **File B (Bank / PFMS Disbursement Log):** Each row represents a **single beneficiary payment credit outcome for the same payment cycle** (e.g., PFMS Direct Benefit Transfer Monthly Success/Failure Report, where each beneficiary account has exactly one primary cycle payment record).
3. **Locked Pair Cardinality:** Strictly **one-to-one (1:1)** in V0 MVP.
4. **Handling of Multi-Payment / Split Transactions:** If an upstream bank file provides multiple transaction lines for a single entitlement (e.g., split payments ₹2,500 and ₹1,500 for a ₹4,000 sanction), V0's duplicate detection rule isolates the subsequent transaction against the same beneficiary to `DUPLICATE_EXCLUDED` with the secondary flag `MULTIPLE_TRANSACTIONS_DETECTED`. This prevents silent row reuse or distorting 1:1 row conservation. Multi-transaction aggregation (many-to-one ledger reconciliation) is documented as a Level 2 (reusable platform) roadmap module, explicitly excluded from the critical V0 hackathon demo.

---

### 4.3 Separating Record Linkage from Financial Reconciliation
The matching engine strictly decouples two operational questions:
1. **Record Linkage:** Do these two rows plausibly describe the same underlying beneficiary or case?
2. **Financial Reconciliation:** Once linked, do the sanctioned amount and disbursed amount agree, and what is the exact variance?

**Critical Rule:** An amount mismatch does **not** disqualify a candidate match. If File A has *"Mohd Iqbal, S/o Abdul Rashid, Acct ****-4812, Sanctioned ₹4,000"* and File B has *"Mohammad Iqbal, S/o Ab. Rashid, Acct ****-4812, Disbursed ₹2,500"*, the system links them as a candidate pair and highlights the **₹1,500 Financial Discrepancy** for administrative action.

---

### 4.4 Multi-Signal Candidate Matching & Review Rules
* **Multi-Signal Evaluation:** Candidate generation evaluates multiple corroborating identity signals (token/phonetic name similarity, parentage where present, masked account suffix where present, location identifiers).
* **Thresholds as Experiment Parameters:** A name similarity threshold (such as `0.82`) is an initial experiment parameter, not a locked business rule. Thresholds must be calibrated against labelled test datasets.
* **Missing vs. Conflicting Data:** Missing information does **not** count as agreement. Contradictory evidence (e.g., completely different parentage) is highlighted as a conflict.
* **No Auto-Approvals:** Every fuzzy candidate sits in `PENDING_REVIEW` until an authorized human officer explicitly chooses an action:
  - **[Approve as Match]:** Records the confirmed pair, reviewer label, and timestamp.
  - **[Reject / Keep Separate]:** Keeps both records unlinked.
  - **[Flag for Field Inquiry]:** Preserves the unresolved case and records the reason.

---

## 5. Mathematical Invariants & Strict Integer Paise Parser

### 5.1 Mutually Exclusive Row Partitioning
To guarantee zero record loss and mathematical consistency, every imported row in File A (Sanction List) and File B (Disbursement Report) is assigned **exactly one primary disposition**:

$$N_A = M + U_A + D_A + I_A + P_A$$
$$N_B = M + U_B + D_B + I_B + P_B$$

Where:
* $M$: Confirmed one-to-one matched pairs (each pair contains exactly one row from File A and one from File B).
* $U_A, U_B$: Unmatched records in File A and File B respectively.
* $D_A, D_B$: Duplicate-excluded records (canonical duplicate policy applied; duplicates remain visible in export).
* $I_A, I_B$: Invalid/malformed records (e.g., corrupt currency string or missing mandatory fields).
* **$P_A, P_B$:** **Distinct source rows assigned the primary status `PENDING_REVIEW`, NOT candidate relationships.** Candidate pairings are edges in an evidence graph (relationship metadata); they do **not** inflate $P_A$ or $P_B$. Even if Row $A_1$ has 3 candidate suggestions in File B ($B_1, B_2, B_3$), $A_1$ contributes **exactly 1** to $P_A$. When an officer confirms pair $(A_1, B_1)$, $A_1$ and $B_1$ move to `MATCHED`, while $B_2$ and $B_3$ fall back to `UNMATCHED` (or other remaining candidates). This preserves exact row conservation without double-counting.

Secondary flags (such as `AMOUNT_MISMATCH` or `PHONETIC_MATCH`) are metadata tags and are never added to primary row counts.

---

### 5.2 Strict Integer Paise Currency Parser
To prevent IEEE 754 binary floating-point representation errors (e.g., `5000.10` drifting to `5000.1000000000003`), all currency amounts are converted at ingestion into 64-bit integer paise.

```python
import re
import math
from decimal import Decimal

# Using [0-9] and [.] completely eliminates backslash-escaping ambiguities across
# markdown renderers, LaTeX listings, shell environments, and Python runtimes.
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
    Rejects malformed strings, negatives, multiple dots, and excess decimal precision.
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
        d = Decimal(str(val))
    elif isinstance(val, Decimal):
        if val < 0:
            raise ValueError(f"Negative currency amount not permitted: {val}")
        d = val
    else:
        raise TypeError(f"Unsupported cell type: {type(val)}")

    if d.as_tuple().exponent < -2:
        raise ValueError(f"Excess precision beyond 2 decimal places: {val}")
    return int((d * 100).to_integral_value())
```

---

### 5.3 Financial Reconciliation Bridge
The financial variance is calculated deterministically across verified categories:

$$\text{Matched-Pair Variance } (\Delta_{\text{matched}}) = \sum_{i \in M} (S_i - D_i)$$

Scheme-wide financial totals are reported across distinct accounting buckets:
1. Total Sanctioned Amount ($\sum S$)
2. Total Disbursed Amount ($\sum D$)
3. Matched-Pair Net Discrepancy ($\Delta_{\text{matched}}$)
4. Unmatched Sanctioned Total ($S_{\text{unmatched}}$)
5. Unmatched Disbursed Total ($D_{\text{unmatched}}$)

---

## 6. Technical Architecture & Local Inference Candidate

### 6.1 Logical System Components
The desktop application is built with a decoupled architecture:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                              LOGICAL ARCHITECTURE PIPELINE                             │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ [1. Desktop GUI (Tauri / Rust)] ──► Local file selection, progress, review cards       │
│                                                                                        │
│ [2. Import & Normalization]     ──► Raw preservation, type-safe parsing, paise conv.  │
│                                                                                        │
│ [3. Deterministic Engine]       ──► Exact matches, blocking, multi-signal fuzzy scoring│
│          ▲                                                                             │
│          │ (Optional Suggestion only; isolated from calculations)                      │
│ [4. Local SLM Adapter]         ──► Qwen2.5-1.5B Q4_K_M (Schema mapping assistance)    │
│                                                                                        │
│ [5. Human Adjudication Queue]  ──► Clerk reviews exceptions & confirms/rejects         │
│                                                                                        │
│ [6. Excel Report Generator]     ──► Writes structured output workbook (openpyxl)       │
│                                                                                        │
│ [7. Audit & Provenance Logger] ──► Monotonic sequence, SHA-256 hash-chaining manifest │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### 6.2 Model Selection: Primary Candidate
* **Selected Candidate Model:** **`Qwen2.5-1.5B-Instruct`** in quantized GGUF format (`Q4_K_M`).
* **Pinned Software Bill of Materials (SBOM):** Pinned artifact `qwen2.5-1.5b-instruct-q4_k_m.gguf` from repository `Qwen/Qwen2.5-1.5B-Instruct-GGUF`.
* **License & Attribution:** Governed by the **Apache 2.0 License** (commercial rights permitted; full attribution and notices bundled).
* **Storage Footprint:** Approximately **1.12 GB** GGUF file size on disk.
* **Inference Runtime:** `llama.cpp` CPU execution via AVX2 vector instructions.
* **Single-Model Lifecycle:** The model is loaded into RAM only when the user encounters unfamiliar column headers or requests an optional memo draft, and is released immediately after. The core matching engine runs with or without the model loaded.

---

## 7. Cybersecurity, Audit Trails & On-Device Data Minimization

### 7.1 Tamper-Evident Hash-Chained Audit Trail
Rather than claiming an impossible "immutable blockchain on a desktop," the application implements a **tamper-evident audit manifest** (`audit_manifest.json`):
1. **Monotonically Increasing Sequences:** Every user action and processing step is logged as an event with a sequential counter ($E_1, E_2, \dots, E_n$).
2. **Cryptographic Binding:** Each event contains the SHA-256 hash of the preceding event:
   $$H_k = \text{SHA-256}(H_{k-1} \,\|\, \text{CanonicalJSON}(E_k))$$
3. **Run Identity:** Records input file hashes, software build hash, local operator label, and execution timestamps generated at runtime.
4. **The Trusted Anchor & Verification Guarantees:** The terminal event hash $H_n$ and export manifest digest serve as the root anchor. When exported, this terminal hash can be printed on the official reconciliation summary sheet or logged to an external administrative register. A standalone verifier detects intermediate record alteration, reordering, or deletion; it cannot prevent a local administrator from regenerating an unanchored log from scratch.
5. **Operator Identity Mechanism:** The prototype records a **local operator label** (e.g., workstation username or clerk session identifier); this is an operational audit tag, not cryptographic proof of user identity.

---

### 7.2 On-Device Data Minimization & Privacy Posture (DPDP Context)
* **Data Minimization by Design:** The architecture is designed for local processing to minimize external data transfers and prevent unauthorized PII egress.
* **Grounded Legal Posture:** We do not claim that running offline automatically grants "100% statutory compliance" or that DPDP Section 16 is an active blanket ban on cloud services. Phased commencement of the Digital Personal Data Protection (DPDP) Act, 2023 was notified by the central government on 13 November 2025, with principal data-processing and fiduciary provisions scheduled to take effect 18 months post-notification. Applicable legal obligations must be reassessed for the actual pilot and deployment dates. We state that the software is architected to uphold the **data minimization and storage limitation principles** of the DPDP Act, leaving data fiduciary control with the department. Deployment remains subject to departmental policy, applicable law, and security review.

---

## 8. Grounded Commercial Model (Office Starter Pack)

### 8.1 Proposed Product Packaging Hypothesis
Government administrative offices purchase utility software for departmental branches rather than individual floating seats. We propose a simple, transparent commercial package hypothesis:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                        COMMERCIAL PACKAGING HYPOTHESIS                                 │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ OFFICE STARTER PACK: ₹49,995 (Initial License Hypothesis)                              │
│  • Covers up to 5 licensed desktop installations within one administrative office.     │
│  • Offline core software license for Scheme Reconciliation module.                     │
│  • Onboarding assistance, user guide, and 1st-year bug fixes and standard updates.     │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ ANNUAL MAINTENANCE & UPDATES: ₹9,999 / year per pack (Starting Year 2)                 │
│  • Annual support, scheme format updates, and certified offline patch media.          │
│  • Equivalent to ~₹1,999 per workstation per year.                                    │
└────────────────────────────────────────────────────────────────────────────────────────┘
```
*(All prices and commercial terms—including perpetuity, update entitlements, transferability, and AMC scope—are provisional business hypotheses, not final quotes; exclude applicable GST).*

---

### 8.2 Public Procurement Context (GFR Rule 149 Realities)
* **Procurement Rule Realities:** Under Rule 149 of the General Financial Rules (GFR), administrative heads possess direct online purchase authority on the Government e-Marketplace (GeM) up to **₹50,000** for products meeting required quality, price reasonableness, and delivery conditions.
* **Procurement Caution:** We do not market ₹49,995 as a guaranteed tender workaround. With applicable GST, total invoiced value may exceed ₹50,000. Furthermore, GeM category registration, supplier eligibility, and purchasing authority limits vary across departments. Pricing must reflect development and support costs, while procurement pathways must be verified with each buyer.

---

## 9. Itemized Prototype Validation Budget (The Restricted ₹50,000 Ask)

### 9.1 Executive Budget Decision & Principles
The venture operates on a **stage-gated, ₹50,000 cash budget** to validate a tightly scoped, offline-first prototype. Existing student-owned equipment, open-source software, and confirmed university resources are utilized wherever practical. Founder and student-team labour is treated strictly as an in-kind academic contribution, not as free cash or revenue.

The budget is allocated specifically to generate verifiable engineering and field evidence: that the software installs cleanly offline, executes deterministic spreadsheet reconciliation, preserves row and monetary invariants, routes ambiguous matches to human review, exports repeatable Excel workbooks, and produces a tamper-evident audit trail. It is **not** a budget to build an all-encompassing government platform, pay founder salaries, ingest live citizen PII, or deliver a production-scale deployment.

### 9.2 Funding & Contribution Summary

| Item | Amount (₹) | Accounting Treatment & Conditionality |
| :--- | ---:| :--- |
| **Prototype-validation grant requested** | **₹50,000.00** | Proposed funding ask (Innovathon 2.0 prize pool / incubator seed); not yet secured. |
| **Year 1 cash-spend ceiling** | **₹50,000.00** | Spend permitted only after formal written grant sanction and strictly against eligible items. |
| **Estimated academic/team in-kind contribution** | **₹1,00,000.00** | Non-cash estimate (CUJ lab computing, mentor hours, student R&D time); not available cash. |
| **Year 1 commercial revenue** | **₹0.00** | Zero commercial sales assumed during the academic prototype validation phase. |
| **Year 1 net cash position (if grant is fully spent)** | **₹0.00** | Cash-neutral prototype phase (grant-funded cash budget, not commercial break-even). |

*Note on In-Kind Contribution:* The ₹1,00,000 in-kind figure represents non-cash academic resources (workstation lab access, institutional mentorship, and student engineering hours). It is recorded as a contribution to the project, not as committed institutional cash.

---

### 9.3 Final Itemized Prototype-Validation Budget (₹50,000 Allocation)
Amounts represent conservative planning ceilings, not final vendor quotations. Actual quotations will be obtained prior to disbursement.

| No. | Budget Head | Allocation (₹) | Share (%) | Operational Purpose & Allowable Use |
| :---: | :--- | ---:| ---:| :--- |
| 1 | **Workflow discovery and local travel** | ₹7,000.00 | 14.0% | Local bus/auto transport between CUJ Campus and Samba/Jammu Tehsil offices for clerical workflow interviews, mapping legacy spreadsheet columns, and documenting reconciliation bottlenecks. No gifts or payments to public officials. |
| 2 | **Offline deployment media and accessories** | ₹5,000.00 | 10.0% | Hardware USB drives with hardware-encrypted partitions, tamper-evident physical pouches, labels, and USB adapters for moving signed/hash-verified offline installers without cloud networks. |
| 3 | **Installer packaging and security hardening** | ₹8,000.00 | 16.0% | Offline installer scripting (PyInstaller/InnoSetup), dependency pinning/SBOM generation, file-hash manifest generation, local loopback network-egress testing, and secure OS default policies. |
| 4 | **Functional, financial-integrity and edge-case QA** | ₹8,000.00 | 16.0% | Repeatable test harness execution for integer paise parsing, row conservation invariants, multi-key candidate generation, exception card rendering, multi-tab Excel export, and tamper-evident audit chaining. |
| 5 | **Low-spec hardware and compatibility testing** | ₹6,000.00 | 12.0% | Temporary lab access, RAM/peripheral compatibility checks, and testing on representative low-spec Core i3 / 8 GB RAM target workstations. Existing student/university PCs are used first; no workstation purchase. |
| 6 | **Bilingual documentation and training aids** | ₹4,000.00 | 8.0% | Concise 2-page laminated visual SOP user guides in Hindi and English, known-limitations register, and step-by-step clerical instructions for adjudicating exceptions and exporting audit manifests. |
| 7 | **Demonstration and evaluation logistics** | ₹5,000.00 | 10.0% | Printing verification reports, workshop demonstration materials, and feedback collection sheets for controlled hackathon and jury evaluation sessions. Strictly zero live citizen data. |
| 8 | **Data-governance and administrative preparation** | ₹2,000.00 | 4.0% | Preparation of formal academic introduction letters, student project consent filings, synthetic data checklists, and compliance documentation. Excludes formal legal retainers. |
| 9 | **Contingency reserve** | ₹5,000.00 | 10.0% | Restricted reserve for unforeseen eligible prototype expenses (e.g., replacement storage media, cables). Requires recorded justification and faculty/grant approval. |
| | **TOTAL PROTOTYPE VALIDATION BUDGET** | **₹50,000.00** | **100.0%** | **Stage-Gated Evidence Generation Budget** |

#### Why This Allocation Is Substantially Stronger
* **Funds Evidence, Not Merely Development:** It prioritizes deterministic testing, edge-case coverage, offline integrity, and user guidance over unneeded hardware acquisitions.
* **Eliminates Capital Waste:** Replaces the premature purchase of a dedicated test workstation with low-spec compatibility testing (capped at ₹6,000) leveraging existing CUJ lab assets.
* **Separates Operational Functions:** Clearly demarcates workflow discovery, engineering hardening, quality assurance, documentation, and evaluation.
* **Strictly Capped Contingency:** Contingency is capped at 10% (₹5,000) and governed by recorded justification, rather than functioning as a discretionary fund.

---

### 9.4 Scope Funded by the ₹50,000 Budget
The ₹50,000 validation budget delivers:
1. An offline desktop application for importing two cycle-level spreadsheet files (CSV/XLSX) and mapping columns deterministically.
2. Exact matching engine and multi-signal candidate suggestions, routing ambiguous records to a human review queue.
3. Integer-paise financial bridge calculations and explicit variance summaries.
4. Multi-sheet Excel export containing confirmed matches, unresolved exceptions, duplicates, and invalid records.
5. Tamper-evident SHA-256 audit manifest with automated verification against a recorded terminal anchor.
6. A reproducible synthetic dataset with ground truth, automated test suite, clean-machine installation guide, known limitations register, and bilingual SOP.

**Explicit Exclusions from V0 Scope:** Live payment authorization; ingestion of actual Aadhaar numbers or live citizen PII; production government server integration; automated document OCR or handwriting parsing; secretariat note-sheet (*Tippani*) generation; paid cloud AI APIs; custom LLM pre-training; formal ISO certifications; and market-rate founder salaries.

---

### 9.5 Spending Controls & Stage Gates

| Gate | Cumulative Spend Ceiling | Evidence Required Before Tranche Release |
| :--- | ---:| :--- |
| **Gate 1 — Define & Baseline** | ₹12,500.00 | Approved project scope document, clerical workflow map, synthetic test dataset specification, itemized procurement list, and designated budget custodian. |
| **Gate 2 — Build & Verify** | ₹37,500.00 | Pinned offline installer build, reproducible test suite report (passing 100% of unit/system tests), financial invariant check logs, and limitations register. |
| **Gate 3 — Demonstrate & Close** | ₹45,000.00 | Successful clean-install offline demonstration, stakeholder feedback sheets, bilingual SOP guides, issue backlog, and final validation report. |
| **Controlled Contingency** | ₹50,000.00 max | Unforeseen eligible expenses only; requires written justification, faculty endorsement, and expense vouchers. |

---

## 10. Reconciled 3-Year Operating Scenario & Market Sizing

### 10.1 Commercial Assumptions & Procurement Safeguards

| Commercial Assumption | Baseline Parameter | Governance & Financial Status |
| :--- | ---:| :--- |
| **Office Starter Pack** | ₹49,995 per 5-seat pack | Commercial pricing hypothesis; subject to validation against actual field support costs. |
| **Annual Maintenance / Support (AMC)** | ₹9,999 per pack (from Year 2) | Annual renewal hypothesis; defines software updates, bug fixes, and tier-1 clerical support. |
| **Year 1 New Paid Packs** | 0 | Zero commercial sales assumed during prototype validation phase. |
| **Year 2 New Paid Packs** | 5 | Planning scenario assumption; not a committed pipeline or order book. |
| **Year 3 New Paid Packs** | 20 | Ambitious regional growth scenario contingent on verified product-market fit. |
| **Year 3 Renewals** | 4 of 5 Year-2 packs | Assumed 80% renewal rate; illustrative hypothesis not yet backed by longitudinal churn data. |

#### Commercial Pricing & Tax Safeguards
* **GST Treatment:** All pricing figures exclude applicable Goods and Services Tax (GST). If 18% GST applies, a ₹49,995 pre-tax license invoices at **₹58,994.10** ($₹49,995 + ₹8,999.10\text{ GST}$), and a ₹9,999 renewal invoices at **₹11,798.82** ($₹9,999 + ₹1,799.82\text{ GST}$). GST collected from a buyer is a statutory pass-through and is never treated as company revenue.
* **Procurement Compliance:** The pricing of ₹49,995 is **not** positioned as a device to bypass public tender thresholds under General Financial Rules (GFR Rule 149). Buyer-specific financial delegated powers, GeM category availability, proprietary article certificates (PAC), and price-reasonableness criteria vary across departments. Splitting purchase orders to circumvent tender limits is strictly prohibited. Procurement routes will be aligned with the Department of Expenditure’s current procurement manuals.

---

### 10.2 Year 2 Rollout Budget (Cash Envelope of ₹4,00,000)
Year 2 commences only upon successful prototype validation and confirmed pilot interest. The ₹4,00,000 allocation is a **maximum cash spending envelope**, not an obligation to disburse funds without demand:

| No. | Budget Head | Allocation (₹) | Intended Operational Use |
| :---: | :--- | ---:| :--- |
| 1 | **Product hardening & maintenance** | ₹90,000.00 | Resolving prototype edge cases, improving desktop reliability, and part-time student stipends / contracted bug fixes. Excludes market-rate founder salaries. |
| 2 | **Independent security review, regression & compatibility QA** | ₹65,000.00 | Third-party code review, multi-version Excel schema testing, vulnerability scans, and regression test suites. |
| 3 | **Deployment, local travel & field support** | ₹55,000.00 | On-site installation, clerical user onboarding, and field support visits across initial 5 pilot offices. |
| 4 | **Legal, accounting, contract & data-governance administration** | ₹35,000.00 | Startup entity incorporation, basic bookkeeping, end-user license agreements (EULA), and data privacy advisory. |
| 5 | **Sales discovery, demonstrations & procurement preparation** | ₹35,000.00 | Administrative presentations, tender preparation, GeM vendor onboarding, and product demonstration collaterals. |
| 6 | **Release packaging, offline update media & documentation** | ₹25,000.00 | Offline update USB drives, digital signature certificates for installers, and updated technical documentation. |
| 7 | **Test devices, peripherals & compatibility upkeep** | ₹25,000.00 | Refurbished test devices and peripherals to verify performance across legacy departmental hardware profiles. |
| 8 | **Communications, basic tools & operational administration** | ₹15,000.00 | Team communication tools, repository hosting, domain registration, and administrative stationery. |
| 9 | **Working-capital & contingency reserve** | ₹55,000.00 | Liquidity buffer for delayed government receivables and unexpected operational expenses. Unspent funds are retained as cash reserves. |
| | **TOTAL YEAR 2 CASH ENVELOPE** | **₹4,00,000.00** | **Maximum Staged Operating Envelope** |

#### Year 2 Sales, Deficit & Bridge Funding Strategy
* **Pre-Tax Revenue:** 5 packs $\times$ ₹49,995 = **₹2,49,975.00**.
* **Maximum Cash Envelope:** **₹4,00,000.00**.
* **Modelled Cash Deficit:** $₹2,49,975 - ₹4,00,000 = \mathbf{-₹1,50,025.00}$.
* **Prudent Liquidity Planning:** To buffer against public-sector payment cycles (which frequently exceed 90–120 days), the venture targets **up to ₹2,00,000 in bridge funding capacity** before committing to the full Year 2 budget envelope.
* **Official Funding Context (J&K Startup Policy):** Under the **Jammu & Kashmir Startup Policy 2024–2027** (as published on Startup India), eligible startups recognized by the Jammu and Kashmir Entrepreneurship Development Institute (JKEDI) may apply for seed assistance of up to ₹20 Lakhs, disbursed in tranches subject to high-level committee appraisal and approval. This policy framework is identified as an avenue for institutional bridge funding; it is **not** an automatic grant, and no funding is assumed secured until formally sanctioned in writing. If institutional bridge capital is unavailable, founder stipends will be deferred and discretionary rollout spending reduced to maintain cash solvency.

---

### 10.3 Year 3 Regional-Scale Budget (Cash Envelope of ₹7,00,000)
Year 3 represents an expansion scenario assuming 20 new packs and 4 renewals across Jammu & Kashmir:

| No. | Budget Head | Allocation (₹) | Intended Operational Use |
| :---: | :--- | ---:| :--- |
| 1 | **Product engineering, maintenance & team support** | ₹1,50,000.00 | Core engine maintenance, adapting to regional scheme variations, release engineering, and junior developer stipends. |
| 2 | **Security review, QA & compatibility testing** | ₹1,00,000.00 | Continuous integration regression testing, cryptographic audit log verification, and OS patch compatibility checks. |
| 3 | **Customer deployments, travel & field support** | ₹1,20,000.00 | District-level onboarding travel, field training workshops, and on-site clerical support across multiple blocks. |
| 4 | **User support, training, documentation & offline releases** | ₹70,000.00 | Comprehensive visual training manuals, video tutorials on USB, and quarterly offline maintenance patches. |
| 5 | **Sales, demonstrations & procurement engagement** | ₹70,000.00 | Departmental demonstrations, GeM bid participation, and commercial proposal administration. |
| 6 | **Legal, accounting, contracts & governance review** | ₹60,000.00 | Annual corporate filings, GST compliance, contractual audit reviews, and external accounting retainers. |
| 7 | **Device/peripheral replacement & essential tools** | ₹30,000.00 | Maintenance of testing testbeds and hardware tokens for secure deployment. |
| 8 | **Working capital & contingency reserve** | ₹1,00,000.00 | Operational buffer to bridge invoice payment lags across 20+ active departmental client accounts. |
| | **TOTAL YEAR 3 CASH ENVELOPE** | **₹7,00,000.00** | **Regional Expansion Operating Envelope** |

---

### 10.4 Three-Year Illustrative Financial Scenario

| Operational Metric | Year 1 (Prototype) | Year 2 (Rollout) | Year 3 (Scale) |
| :--- | ---:| ---:| ---:|
| **New 5-Seat Packs Sold** | 0 | 5 | 20 |
| **Renewing Packs (AMC @ ₹9,999)** | 0 | 0 | 4 |
| **New License Revenue (Pre-Tax)** | ₹0.00 | ₹2,49,975.00 | ₹9,99,900.00 |
| **Maintenance / AMC Revenue** | ₹0.00 | ₹0.00 | ₹39,996.00 |
| **Total Commercial Revenue** | **₹0.00** | **₹2,49,975.00** | **₹10,39,896.00** |
| Prototype Seed Grant Requested | ₹50,000.00 | ₹0.00 | ₹0.00 |
| Maximum Cash Operating Envelope | ₹50,000.00 | ₹4,00,000.00 | ₹7,00,000.00 |
| **Cash Coverage Residual (Full Envelope Used)** | **₹0.00 (Net of Grant)** | **−₹1,50,025.00** | **+₹3,39,896.00** |

*Accounting Disclosure:* The net cash coverage line illustrates cash flow under the assumptions that the grant is received, the sales scenario occurs, all customer receivables are collected within the fiscal year, and the full budget envelope is consumed. It is **not a statutory profit-and-loss statement**. Actual financial performance will depend on incurred expenditure, revenue recognition timing, applicable corporate taxes, and working-capital cycles.

#### Sales Sensitivity Analysis (Mitigating Optimism Bias)

| Operating Scenario | Sales Calculation | Pre-Tax Commercial Revenue | Residual vs. Stated Cash Envelope |
| :--- | :--- | ---:| ---:|
| **Year 2 Downside (3 Packs)** | 3 packs $\times$ ₹49,995 | ₹1,49,985.00 | −₹2,50,015.00 against ₹4,00,000 |
| **Year 2 Base Scenario (5 Packs)** | 5 packs $\times$ ₹49,995 | ₹2,49,975.00 | −₹1,50,025.00 against ₹4,00,000 |
| **Year 2 Cash-Coverage Case (9 Packs)** | 9 packs $\times$ ₹49,995 | ₹4,49,955.00 | +₹49,955.00 against ₹4,00,000 |
| **Year 3 Downside (10 Packs + 2 Renewals)**| 10 $\times$ ₹49,995 + 2 $\times$ ₹9,999 | ₹5,19,948.00 | −₹1,80,052.00 against ₹7,00,000 |
| **Year 3 Base Scenario (20 Packs + 4 Renewals)**| 20 $\times$ ₹49,995 + 4 $\times$ ₹9,999 | ₹10,39,896.00 | +₹3,39,896.00 against ₹7,00,000 |

---

### 10.5 Explicit Financial Exclusions & Regulatory References

#### Explicit Exclusions from the 3-Year Projections
The financial projections deliberately exclude or defer:
* Market-rate founder salaries for all 6 student team members (opportunity cost absorbed as student R&D effort).
* Full-time dedicated payroll, commercial office leasing, and corporate legal retainers.
* Income tax liabilities, GST registration costs, and advance tax payments.
* Extensive travel outside Jammu, Samba, and adjoining districts of J&K.
* Third-party penetration testing or ISO/IEC 27001 formal audits.
* Complex bespoke integrations with state data centres or legacy departmental Oracle/SQL backends.

#### Official References for Regulatory Verification
1. **Startup India — Jammu & Kashmir Startup Policy 2024–2027:**  
   [https://www.startupindia.gov.in/content/sih/en/state-startup-policies/Jammu-and-Kashmir-state-policy.html](https://www.startupindia.gov.in/content/sih/en/state-startup-policies/Jammu-and-Kashmir-state-policy.html)  
   *Governs eligibility criteria and application pathways for JKEDI-recognized seed assistance.*
2. **Department of Expenditure (Ministry of Finance) — Procurement Manuals:**  
   [https://doe.gov.in/en/manuals](https://doe.gov.in/en/manuals)  
   *Governs procurement rules, direct purchase thresholds, and GeM purchasing guidelines.*
3. **Central Board of Indirect Taxes and Customs (CBIC) — GST Sectoral FAQs:**  
   [https://cbic-gst.gov.in/sectoral-faq.html](https://cbic-gst.gov.in/sectoral-faq.html)  
   *Provides guidance on classification and tax rates for software licensing versus professional IT services.*

---

### 10.6 Market Sizing: Grounded Administrative Baseline
* **Documented Administrative Universe:** Planning Development & Monitoring Department data (J&K, March 2020) records **285 Community Development Blocks** across the Union Territory.
* **Illustrative Scaling Ceiling:** If each block administrative complex operates 3 office starter packs (15 workstations across Welfare, Revenue, and BDO sections), 285 complexes represent an illustrative theoretical value of **₹4.27 Crores** ($855 \times ₹49,995$). This is an illustrative ceiling, not a verified serviceable obtainable market.
* **Grounded 3-Year Commercial Objective:** Deploying **25 packs** across Years 2 and 3 represents **₹12,49,875.00** in initial license revenue, establishing a disciplined, defendable beachhead.

---

## 11. Institutional Pilot Playbook (CUJ + JU SIIEDC Academic Bridge)

### 11.1 The Dual-University Incubation Ecosystem
As students of the **Central University of Jammu (CUJ)** presenting at **Innovathon 2.0 (University of Jammu)**, the venture leverages the combined strengths of both institutions:

1. **Central University of Jammu (CUJ) — Home Institution Anchor:**
   * **Administrative Introduction:** The Department of Computer Science & IT / Startup Cell issues a formal academic recommendation letter to the **Deputy Commissioner (DC) Samba** (where the CUJ campus is located) requesting permission for a 4-week zero-risk academic Proof-of-Concept (PoC) pilot. Campus computing labs and faculty mentorship are provided at zero cost.
   * **Separation of Milestones:** University sponsorship, office administrative permission, data-use approval, and commercial procurement are treated as four distinct institutional requirements.
2. **University of Jammu (SIIEDC) — Regional Innovation Host:**
   * **Incubation & Networking:** Mentorship through the Skill Incubation, Innovation and Entrepreneurship Development Centre (SIIEDC) under the *J&K Tech & Innovation Surge 2026*.
   * **Seed Funding:** Access to the Innovathon 2.0 prize pool to fund the ₹50,000 prototype validation stage.

---

### 11.2 The 6-Stage Go-to-Market Sequence
1. **Stage 1 (Hackathon):** Demonstrate working offline prototype on synthetic data with known expected outcomes.
2. **Stage 2 (University Intro):** Secure faculty sponsor and administrative introduction to DC Samba / DC Jammu.
3. **Stage 3 (Controlled Pilot):** Obtain written clearance for a 4-week pilot in one local BDO/Tehsil office using synthetic or de-identified data.
4. **Stage 4 (Pilot Validation Report):** Document measured task time saved, accuracy on real edge cases, and clerical usability.
5. **Stage 5 (Commercial Packaging):** Finalize GeM listing, vendor registration, and support documentation.
6. **Stage 6 (Repeatable Sales):** Expand to neighboring BDO offices across Jammu, Samba, and Udhampur districts.

---

## 12. Technical Acceptance Criteria, Known Limitations & Risk Register

### 12.1 Technical Acceptance Criteria (Evidence-Based Gates)
To replace unsubstantiated scorecards with engineering rigor, the prototype is evaluated against concrete acceptance gates:

| Dimension | Target Acceptance Criteria | Verification Method |
| :--- | :--- | :--- |
| **Record Conservation** | Every source row has exactly one primary disposition; no silent record dropping. | Invariant check ($N = M + U + D + I + P$) verified by unit test. |
| **Financial Accuracy** | All calculations performed in integer paise; zero floating-point drift. | Parser test suite covering standard, comma-grouped, and malformed currency. |
| **Matching Quality** | Exact-match precision, candidate precision, recall, and false-match rates reported separately on synthetic benchmarks; synthetic results explicitly noted as not guaranteeing dirty field accuracy. | Labeled synthetic test dataset with known ground truth. |
| **Human Control** | Zero auto-approval of fuzzy candidates without explicit clerk confirmation. | Event log verifies operator label and explicit action before row is moved to `MATCHED`. |
| **Offline Operation** | Full workflow executes with networking physically and technically disabled. | Live demonstration with Wi-Fi disabled + external network traffic audit. |
| **Audit Verification** | Altered, reordered, or deleted log entries are detected by the verifier. | Integrity test script deliberately corrupting intermediate hash links. |

---

### 12.2 Known Limitations & Risk Register
We openly acknowledge our current limitations and document our engineering mitigations:

| Identified Risk | Severity | Built-In Mitigation / Evidence Pathway |
| :--- | :---: | :--- |
| **False Match from Shared Account Suffix** | Critical | Masked account suffix (`****-1234`) is strictly supporting evidence; matching requires converging independent signals. |
| **Hidden Financial Discrepancies** | High | Record linkage and amount comparison are decoupled; matched-pair amount variances are explicitly flagged. |
| **Duplicate Row Distortion** | High | Canonical duplicate policy applied; duplicates are assigned to `DUPLICATE_EXCLUDED` and exported in a separate tab. |
| **LLM Inference Failure or Stall** | Medium | Local LLM is strictly an optional assistant for unknown column mapping; core reconciliation runs deterministically if LLM is disabled. |
| **Privileged Log Tampering** | Medium | SHA-256 hash chaining detects internal record alteration when anchored; external signed checkpoints required for high-assurance audit defense. |
| **Procurement Delays** | High | Phased pilot model: Stage 1 pilot is decoupled from commercial procurement by requiring four distinct non-commercial approvals: (1) CUJ academic sponsorship, (2) BDO/Tehsil administrative site permission, and (3) data-use approval on synthetic or de-identified datasets, before commercial procurement is ever initiated. |

---

### 12.3 The Core Closing Statement
> **“AI suggests. Deterministic code calculates. The officer decides. Every material action is traceable.”**  
> *By solving one real, measurable clerical workflow with uncompromising mathematical and legal rigor, we build the foundation of trust required for the digital governance of tomorrow.*
