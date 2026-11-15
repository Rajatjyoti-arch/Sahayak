# Sahayak (सहायक)
### The Sovereign Offline Governance & Scheme Reconciliation Assistant

> **Confidential & Proprietary** | Copyright © 2026 Rajatjyoti & Contributors. All Rights Reserved. See [LICENSE](LICENSE) for strict trademark and usage terms.

---

## Overview
**Sahayak** is an air-gapped, offline-first system designed for grassroots public administration (Tehsils, Block Development Offices, District Collectorates). It automates Direct Benefit Transfer (DBT) scheme reconciliation, citizen petition ingestion, and administrative note-sheet generation under strict government hardware constraints (commodity 8 GB RAM PCs, zero GPU, zero cloud exposure).

### Core Invariants
1. **Zero Floating-Point Drift**: Financial calculations are executed in strict integer paise (`i64`).
2. **Mathematical Row Conservation**: Every input row is accounted for in mutually exclusive partitions:
   $$N = M + U + D + I + P$$
   *(Matched, Unmatched, Duplicates, Invalid Format, Pending Review)*.
3. **Decoupled Record Linkage**: Entity resolution is decoupled from financial variance.
4. **Cryptographic Provenance**: SHA-256 hash-chained audit manifests ensure tamper-evident records for administrative vigilance.

---

## Repository Structure
```
Sahayak/
├── AGENTS.md                  # Development invariants & agent engineering contract
├── LICENSE                    # Strict proprietary source code & trademark license
├── README.md                  # Project overview
├── Cargo.toml                 # (Upcoming) Rust library manifest
├── src/                       # (Upcoming) Rust core engine & harness
├── prototype/                 # Tested reference implementation (Python 3)
│   ├── reconciliation_engine.py
│   ├── test_reconciliation_suite.py
│   ├── verify_audit_log.py
│   ├── generate_synthetic_data.py
│   └── data/
├── docs/                      # Architectural specifications & business records
│   ├── specs/                 # Technical specifications & blueprints
│   ├── business/              # Proposals, financial budgets & pitch decks
│   ├── reviews/               # Red team & critique reports
│   └── diagrams/              # Architecture vector graphics
```

---

## Development & Verification
This repository adheres to strict engineering and verification contracts outlined in [AGENTS.md](AGENTS.md). All new business logic is being authored in pure Rust, guided by Gravitas evidence standards.
