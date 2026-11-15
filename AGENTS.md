# Engineering Contract & Development Invariants

## Core Principles
1. **Learning-First Engineering**: AI acts as an engineering assistant. Do not generate code without explaining its design, rationale, and underlying Rust concepts first.
2. **Minimalism (Ponytail Philosophy)**:
   - Write the smallest complete solution.
   - Do not introduce premature abstractions, generic frameworks, or speculative extensibility.
   - Prefer Rust standard library features before adding external dependencies.
3. **Reliability & Verification (Gravitas Invariants)**:
   - Ground before mutating: Read target code, tests, and callers before proposing changes.
   - Stay within declared file scope.
   - No claim of completion without fresh verification evidence (compiler output, test runs).
   - Never exact-retry a failed command without a material change in diagnosis.
   - State limitations and unverified claims honestly.
4. **Domain Integrity (GovTech & Sahayak Rules)**:
   - Zero IEEE 754 floating point arithmetic in financial paths: strict integer paise only.
   - Mathematical row conservation invariant is inviolable: N = M + U + D + I + P.
   - Record linkage is decoupled from financial variance.
   - Never silently change business rules or thresholds.

## Step-by-Step Workflow
1. **Understand**: Examine the problem, existing prototype, and Rust concepts.
2. **Design**: Agree on the smallest reasonable implementation and trade-offs.
3. **Implement**: Keep changes small, self-contained, and accompanied by tests.
4. **Verify**: Compile, test, check edge cases, and inspect the diff.
5. **Master**: Ensure every type, function, and borrowing decision can be explained and defended.

