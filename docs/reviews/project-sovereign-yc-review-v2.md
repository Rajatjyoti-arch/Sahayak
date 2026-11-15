# Project Sovereign: YC Partner Review (Revision 2)

Reviewed against the revised `plan.md`: the narrowed "Local Workflow Engine" version with a $450k pre-seed, Linux-first V0, and the phone bridge pushed to V3.

---

## Verdict

**Better, and now the right shape for a pre-seed pitch. Still a pass today.**

The first version was four companies in one deck. This one is a single product (offline spreadsheet/document deliverables on cheap laptops) with an honest scope. But three things would stop me from writing a check right now:

1. **The core model can't legally be shipped in a commercial product** as written (Qwen2.5-3B is non-commercial).
2. **The financial model still treats year-end ARR as full-year revenue**, and the Year 2 and Year 3 ARR don't match your own inputs.
3. **The "deterministic checksum" gives users false confidence**, and the plan's description of what `openpyxl` can do is wrong in ways that matter to accountants.

On top of that there is still no team section, no customer evidence, and no measured benchmark.

### Scorecard

| Dimension | Score | Note |
| :--- | :---: | :--- |
| Problem clarity | 3/5 | Privacy-sensitive small firms is real, but needs customer proof |
| Wedge focus | 4/5 | Big improvement from v1 |
| Technical feasibility | 3/5 | Right architecture direction, unproven reliability |
| Differentiation / moat | 2/5 | Open models plus open libraries, easy to copy |
| Financial credibility | 2/5 | Structural error plus arithmetic that doesn't tie |
| Team / traction | n/a | Not in the document |

---

## What got fixed since v1 (credit where due)

- **Scope cut hard.** Phone bridge, Cloud Turbo Router, and 3-OS automation are out of V0.
- **The false "air-gap" claim is gone** (section 3.4 says so explicitly).
- **The LLM is forbidden from doing arithmetic.** It plans, Python computes. That's the right principle.
- **Metadata-first ingestion** (column names, types, 3 sample rows) is the correct way to feed a 3B model on a slow CPU.
- **Prefix caching and compact tool schemas** show real awareness of the prompt-latency problem.
- **Financials are far less fantastical**, with ~1.5% conversion and a $450k raise.
- **Moondream is replaced** with a proper OCR stack.

---

## Critical issues (ranked)

### 1. The model license kills the business as written

Qwen2.5-3B-Instruct is distributed under the **Qwen Research License**, which limits use to non-commercial research and evaluation. Commercial use requires a separate license from Alibaba Cloud. A paid Pro tier and an Enterprise fleet built on that model is a commercial use.

You budgeted "model license audits" in the $54k reserve, which is good. But this is a Day-0 blocker, not a reserve line item.

**Fix:** evaluate commercially safe models now, such as Qwen3-4B (Apache 2.0), Phi-4-mini (MIT), or Qwen2.5-1.5B/7B (Apache 2.0), or negotiate a license with Alibaba before you build further on 3B. Check the current license file for each candidate yourself.

### 2. Financial model: the same structural error, plus numbers that don't tie

**ARR is not revenue.** Your net cash flow line subtracts COGS and opex from *year-end ARR*. Your own milestones say MRR is $0.6k at Month 3, $3.6k at Month 6 and ~$15k at Month 12. Interpolating those, actual Year 1 recognized revenue is roughly **$70k, not $177.6k**.

| | Plan says | My reconstruction |
| :--- | :--- | :--- |
| Y1 net cash flow | −$63,400 | about **−$160,000** |
| Y2 net cash flow | **+$199,000** ("cash-flow positive") | about **−$60k to −$15k** |
| Y3 net cash flow | +$1,271,000 | about **+$0.4M to +$0.6M** |

(Reconstruction: revenue recognized ≈ average MRR across the year × 12, COGS at ~11–12%, opex as stated.)

Good news: cumulative loss through Year 2 is roughly $220k, still inside your $450k raise. The company survives, but **"cash-flow positive by Year 2" is false**. At best it's Year 3.

**ARR doesn't match your own inputs.** Using your prices ($12 × 12 = $144 Pro, $8 × 12 = $96 per seat):

| Year | Inputs | Computed ARR | Plan states |
| :--- | :--- | :--- | :--- |
| Y1 | 900 Pro + 500 seats | $177,600 | $177,600 (ties) |
| Y2 | 4,200 Pro + 2,800 seats | **$873,600** | $768,000 |
| Y3 | 14,500 Pro + 11,000 seats | **$3,144,000** | $2,781,000 |

**Other cracks:**

- "1.5% conversion" is the stated assumption, but your inputs imply 1.5% in Y1 and ~1.7% in Y2 and Y3. 1.5% of 250k is 3,750, not 4,200.
- The model uses $144/yr Pro ARPU, but you also sell $99/yr. If half of buyers go annual, blended Pro ARPU is about $121.50, which is roughly 16% lower.
- Zero churn is modeled, and CAC isn't modeled at all.
- Year 1 opex of $220k doesn't match a $450k raise with an 18-month runway ($25k/month, or about $300k per 12 months).
- Two engineers for $225k over 18 months only works if they're offshore, and there are no founder salaries.
- Support at $0.70/user/month is optimistic for desktop software across thousands of different laptops (crashes, AV blocks, failed 3–4 GB downloads).
- The $450k raise has no stated valuation or cap, and the Seed target of $5–7M on ~$180k ARR is 28–39x ARR.

### 3. The "Deterministic Checksum Verifier" is mostly theater

`Assert Sum(Debits) == Sum(Credits)` verifies that a conservation property holds. It does **not** verify that the logic was right. It cannot catch:

- a wrong filter (what counts as "overdue"?)
- date parsing errors (dd/mm vs mm/dd, a classic cause of silent errors)
- the wrong column picked (amount vs. tax-inclusive amount)
- duplicate handling, rounding policy, or dropped rows that net to zero

Showing **"[✓] Checksum passed (100%)"** in the UI will make professionals trust output the system hasn't actually validated. That's a liability risk, not a feature.

**Fix:** rename it ("Integrity checks"), list exactly what was and wasn't verified, and show row counts in/out, changed-cell diffs, and the generated script so the user can review them.

### 4. `openpyxl` doesn't do what the plan says it does

- **"Preserves existing Excel charts and styling" is false.** openpyxl's own documentation warns that charts and images in existing files are lost when you open and save the file. Pivot tables, slicers, and some data-validation extensions are also at risk, and macros need special handling. For accountants' real-world workbooks this is a trust-destroying bug.
- **openpyxl writes formulas but doesn't calculate them.** Files open with empty cached values until Excel recalculates. Your "Live Data Preview" needs real computed values, so you need a calc engine (LibreOffice headless, a formula library) or a second pandas computation, and then you have two sources of truth that can disagree.

**Fix:** write output to a *new* workbook or new sheets by default, never round-trip the user's original, and test against 50 real client-style files.

### 5. The LLM's role is undefined, and that's the product

"The model acts strictly as a Code Planner: it selects the appropriate Python logic." Which one?

- **(a) Free-form code generation by a 3B model.** Error rates are high, and silent wrong-but-running code is the failure mode. You also need a real sandbox, and "standard process sandboxing" isn't a spec.
- **(b) Selection from a fixed library of vetted operations.** Much safer, but then the product is a deterministic ops engine with a natural-language front end. The value is the ops library, and a 3B model must reliably map messy requests to ops.

Option (b) is the one I'd back, and it's the one you should say out loud. Then add **grammar/JSON-schema constrained decoding** (llama.cpp supports GBNF grammars), which dramatically improves valid-tool-call rate on small models and isn't mentioned anywhere in the plan.

### 6. Latency math is still off

- **3,000 tokens at 25–40 tok/s is 75–120 seconds, not 30 seconds.** A 30-second freeze corresponds to roughly 750–1,200 tokens.
- Your architecture actually fixes most of this (prefix cache plus ~150-token metadata), so a typical spreadsheet request might be ~300–500 uncached tokens, or about 10–20 seconds of prompt time. Say that honestly.
- Generation speed isn't stated. On single-channel DDR4, realistic Q4 3B generation is roughly 5–8 tok/s (my estimate), so a 200-token plan takes 25–40 seconds.
- Realistic end-to-end is **~30–90 seconds per task**. That's fine for deliverables (an accountant won't mind if it saves hours) but your demo must show it truthfully and the UI needs honest progress states.
- Documents are the opposite case. A 10-page PDF is ~5–7k tokens, so 2–5 minutes just to read it, and a 50-page report takes 10+ minutes, before any chunking overhead.

### 7. The memory budget is optimistic

- **"Tauri + WebView2 ≈ 45 MB"** is the app shell, not the web view. WebView2 runs as separate `msedgewebview2.exe` processes and typically costs well over 100 MB (measure it). On Linux, Tauri uses WebKitGTK, which behaves differently.
- **The KV cache isn't counted.** Qwen2.5-3B has 36 layers and 2 KV heads, which is about 36 KB per token in fp16, so roughly 0.3 GB at 8k context, plus compute buffers.
- **"Embedded Python ≈ 80 MB"** drops sharply once `pandas`, `numpy`, and `openpyxl` are imported. PaddleOCR with its runtime is far heavier than that, and Workflow 3 needs OCR and the LLM in the same pipeline.
- Large `.xlsx` files can take many times their size in RAM under openpyxl.
- **"3.8–4.2 GB free on an 8 GB PC" ignores that the user is running Excel, a browser, and Defender.**
- "Guaranteeing zero system thrashing" is an overclaim. Say "target" and show measurements.

### 8. Linux-first dev vs. Windows-first customers

The beachhead is Windows accountants, but V0 is "battle-tested on Linux" with "zero OS overhead," which means your numbers are optimistic by construction. "Packaging for Windows/macOS is seamless" is never true. Expect:

- Defender real-time scanning slowing file I/O
- SmartScreen and code-signing reputation
- AV false positives on bundled-Python-plus-LLM apps
- Different web engines (WebView2 vs. WebKitGTK) producing UI differences
- AVX2 runtime dispatch (some budget Celeron/Pentium CPUs lack it)
- Embedded-Python packaging on Windows
- Sandbox primitives that differ per OS (the spec only names Linux paths)

**Fix:** put one real Windows 8 GB i3 laptop on the benchmark bench from day one.

### 9. Workflows 2 and 3 are weak links

- **Workflow 2 (PDF/text to DOCX):** generic summarization and formatting is commodity (free chatbots and Word's built-in AI do it) and a 3B model summarizes poorly and can hallucinate. It dilutes the accountant story. Cut it or make it accountant-specific (e.g., engagement letters, audit working papers).
- **Workflow 3 (OCR to Excel):** table structure from phone photos is hard (skew, glare, handwriting). Tesseract has no real table recognition. And having Qwen "format" OCR output **re-introduces the very error you banned**: a transcription that quietly changes `1,420` to `1,402` can't be caught by a debit/credit checksum. PaddleOCR with its framework is also a packaging headache. Lighter ONNX-based OCR stacks are worth evaluating. Show the user the source image next to extracted values with low-confidence cells flagged.

---

## Positioning and claims that don't hold up

- **"Zero subscriptions"** (the 1-sentence pitch) vs. a **$12/month Pro** subscription and an **$8/seat/month** enterprise plan.
- **"Zero outbound networking"** vs. license validation and skill sync via Cloudflare KV in the unit economics, and "private enterprise sync" in V2.
- **"Air-gapped compliance"** and "verified offline air-gap attestation" in V2, right after section 3.4 promises no false air-gap claims. Decide which one you are. Also, "verified" by whom?
- **The "hardware divide" argument is inverted.** Cloud AI runs fine on a budget laptop. It's *local* AI that is hardware-hungry. Your real arguments are privacy, cost, and offline use. Say that.
- **"82% of knowledge workers... i3/i5, 8 GB, no GPU"** is unsourced and implausible. Cite it or cut it.
- **"Legally prohibited or terrified."** Regulations like HIPAA and GDPR typically require agreements and safeguards, not a blanket ban. "Terrified" is a hypothesis to validate with interviews.
- **The $30 Copilot anchor is outdated for your beachhead.** Microsoft launched Copilot Business for organizations up to 300 users at about $21/user/month (annual commitment), and Copilot Chat is included at no extra cost for Microsoft 365 customers. Your $12 vs. $30 contrast is really $12 vs. $21, and a free chat tier exists.
- **"Existing local AI merely outputs raw text"** is outdated. Several local tools already ship code-execution or agent modes. Your pitch is *reliable, packaged, and non-technical*, not "first."
- **"Replaces 4–6 hours of manual spreadsheet formatting per week"** is unmeasured. Time 10 real bookkeepers.

---

## Competition you need to name

- **Excel and Microsoft 365 Copilot** (Business tier, plus free Copilot Chat) and Power Query, which is free, deterministic, and already familiar to accountants.
- **Accounting-suite built-ins:** bank reconciliation and receipt OCR in tools like QuickBooks, Xero, Dext, and Hubdoc.
- **Local-AI apps:** Ollama, LM Studio, Jan, AnythingLLM, Open WebUI (with code execution).
- **Frontier-lab desktop and Excel agents,** such as Claude Cowork and Claude in Excel, paired with an anonymize-then-send workflow.
- **On-device Windows AI** on Copilot+ PCs.

The honest moat question: if the LLM only picks from an ops library, your durable asset is that library plus distribution and trust with one profession. Skills are copyable files, so there is no network effect at 900 subscribers.

---

## Pricing, roadmap and go-to-market

- **Enterprise math:** $8/seat × 20-seat minimum = **$1,920/year per firm.** The Year 1 target of 500 seats means about 25 firms, yet your Month 6 milestone is just 2 pilots (pilots are usually free). The V2 fleet console is scheduled for Months 5–8, leaving little time to sell it.
- **Skill counts disagree:** 10 free, "1,000+" Pro (pricing), 250 (roadmap and budget).
- **$90k for 250 verified skills is $360 per skill** with accountant verification. That's 3 to 7 hours of professional time each, including test data. And which jurisdiction (US GAAP, IFRS, UK VAT, India GST)?
- **V1 (Months 2–4)** includes a skill hub, BM25 retrieval, a **no-code Skill Creator**, and batch processing of 50 files, built by two engineers while V0 is still unproven. The Skill Creator alone is a product, and user-authored skills run on a 3B model make the reliability problem worse. A marketplace is premature at this scale.
- **The beachhead isn't one persona:** "accountants, clerks, and students" (diagram) vs. "solo accountants, bookkeepers, tax preparers, legal clerks, SME admin" (section 6.1). Students won't pay $12/month. Pick one. I'd pick **solo bookkeepers in one country**.
- **"Viral adoption" for accountants** isn't how that profession spreads software. Associations, trainers, and reseller or accounting-software communities are the real channel, and they have long cycles.
- **License enforcement** in an offline desktop app is easy to bypass. Make Pro valuable through updates, curated content, and support rather than key checks alone.
- **Professional liability:** wrong numbers mean angry accountants. You need terms of use, clear disclaimers, and a plan for errors-and-omissions exposure.
- **"Audit-ready" is undefined.** If you mean it, ship a provenance log (input file hash, operations applied, script, output hash) that can be exported with every deliverable. That could become a genuine differentiator.

---

## What's still missing entirely

- **Team.** Who are the founders and why are they the people to build this? YC funds people first.
- **Traction or evidence.** Zero interviews, design partners, LOIs, or prototype demos.
- **Measured benchmarks.** No task-success rate, no error rate, no timings on actual target hardware.
- **Market size.** No bottom-up count of potential buyers in the chosen segment.

---

## What would turn this into a yes

1. **Resolve the model license** this week and re-benchmark on a commercially safe model.
2. **Build a benchmark of 100 real, messy files** (bookkeeper-style spreadsheets, scanned receipts). Report task success rate, **silent-error rate**, and wall-clock time on an actual 8 GB i3 running Windows. Target: 85–90%+ on a narrow set of operations.
3. **Define the LLM's job** as selecting from a vetted operations library with constrained decoding. Publish the list of supported operations.
4. **Fix the financial model:** monthly revenue recognition, churn, support cost, CAC, consistent ARPU (monthly vs. annual), and numbers that tie to their inputs.
5. **Talk to 20 bookkeepers or small CPA firms** in one country. Get 5 using a rough prototype weekly, ideally 2 willing to pre-pay.
6. **Rename the checksum** to something honest and add an exportable provenance log and a visible diff.
7. **Cut the roadmap** to V0 plus one paid feature. Drop the Skill Creator, marketplace, and fleet console until the first 50 paying users exist.
8. **Add the team slide.**

---

## Sources and caveats

- Qwen Research License terms (non-commercial research/evaluation only; commercial use needs a separate license), as shown on Hugging Face model and license pages for Qwen2.5-3B-Instruct, e.g. <https://huggingface.co/Qwen/Qwen2.5-3B-Instruct-GPTQ-Int8/commit/53114ac80f042543a1084f693c8b3dbac705e830>
- Microsoft 365 Copilot Business (launched December 1, 2025; up to 300 users; about $21/user/month on annual commitment; Copilot Chat included at no extra cost): <https://www.computerworld.com/article/4093224/> and <https://office-watch.com/2025/microsoft-365-copilot-business-ai-add-on/>
- The revenue reconstruction is my own interpolation of your stated MRR milestones. Treat it as an estimate, not a forecast.
- Throughput, memory, and WebView2 figures are my estimates, not benchmarks. Measure on your target hardware.
- The openpyxl chart/image-loss behavior is from its documentation and widely reported experience. Verify against your own sample workbooks.
