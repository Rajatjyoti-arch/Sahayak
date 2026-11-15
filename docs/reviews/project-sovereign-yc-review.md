# Project Sovereign (Jarvis Local): YC Partner Review

**Verdict: pass at this stage.** I'd take a second meeting only if you come back with the evidence listed at the end. The idea is interesting, but as written it's four companies in one deck, and its headline claims contradict each other. The financials also have a structural error. Below is the full teardown, ranked by how badly each issue would hurt you.

---

## What's genuinely good

- Privacy-first local AI for regulated professionals is a real wedge.
- Deterministic tools plus structured JSON is the right call over screenshot-clicking.
- Near-zero inference COGS is real, if the product works.
- The 3-week MVP phase is the right instinct.

---

## Fatal contradictions

### 1. "Air-gapped" and "Call-Your-Laptop" can't both be true

The plan sells zero cloud dependency, zero telemetry, and "100% air-gap compliance certificates." The same plan ships:

- a reverse tunnel (WireGuard / Cloudflare)
- Twilio, Telegram, and WhatsApp bridges
- Cloudflare Workers license validation
- a web research agent
- a "Cloud Turbo Router" to 70B+ cloud models
- GitHub "telemetry bugs"

A compliance officer will read that list once and kill the deal. You can't audit "100% air-gap" on a build that has a tunnel daemon in it. Also, a "HIPAA/GDPR/SOC2 air-gap certificate" isn't a thing you can sell. Those are audits and attestations, and SOC2 Type II needs a months-long observation window, not "complete by Week 28."

### 2. The sandbox claim contradicts the tool list

The plan says the sandbox has no network and no access outside the scratch folder, and that a hallucinated command can never damage the system. But the tools include:

- PowerShell/COM and AppleScript dispatch
- local mail client control
- `fs_fast_search` across the whole disk
- web scraping
- model-generated Python (`transformation_code`)

That's an arbitrary-code-execution surface, and a folder isn't a sandbox. Real isolation on three OSes means AppContainer, seccomp/bubblewrap, and macOS sandboxing, which is serious engineering the plan doesn't budget for.

### 3. The latency math breaks on your own table

You claim ~600 ms for "Qwen reasoning and tool execution" at 14–20 tok/s, which is about 10 tokens. A tool-call JSON is 60–100+ tokens, so that's 4–7 seconds of generation alone.

- **Prompt processing is the bigger killer.** A 3B model on a 4-core laptop CPU reads prompts at roughly 20–60 tok/s (my estimate). Inject a skill recipe, tool schemas, and a spreadsheet excerpt, and you're at 50–150 seconds before the first output token.
- **Your 14–20 tok/s is roughly the theoretical ceiling for dual-channel DDR4.** Budget i3 laptops often have a single RAM stick, which gives about 5–8 tok/s in practice.
- **"Pages models in and out in under one second via mmap" doesn't work.** A 2 GB model off a 7200 RPM HDD (which you list as supported) takes 15+ seconds, and ~4 seconds even on a SATA SSD.

So the 1.15–1.4 s voice turnaround isn't a stretch goal. It's physically wrong, and the viral demo built on it would fall apart in front of users.

---

## Product and technical loopholes

### 4. A 3B model on accountant and lawyer work is a liability machine

- "Skills give a 3B model the accuracy of a 70B" is the weakest claim in the document. A skill injects procedure, not reasoning ability or knowledge.
- The self-healing loop catches crashes. The real failure mode is code that runs and produces wrong numbers. A tax reconciliation off by 3% with no error is worse than a failure.
- Contract analysis means 10k–50k tokens of context. At CPU prompt speeds that's minutes to tens of minutes, and the KV cache pressure on 8 GB is real.
- Where's the eval? You have no task-success rate on real, messy client files.

### 5. The target markets fight the model

South Asia, Africa, and Latin America mean Hindi, Swahili, Portuguese, and so on. Qwen 3B's tool-calling in those languages is weak, Piper's voice coverage is thin, and Whisper-base on 8 kHz phone audio with accents will be rough.

### 6. Office automation reality

- `openpyxl` doesn't compute formulas, and loading then saving can drop charts and images.
- COM needs Office actually installed, but your users run WPS, LibreOffice, or cracked copies.
- macOS gates AppleScript behind per-app consent prompts.
- Linux DBus isn't a real office-automation story.
- Three OS backends for three engineers is a trap.
- Moondream 2 isn't a serious OCR engine for financial documents. Tesseract/PaddleOCR-class tools beat it on accuracy.

### 7. Distribution friction

- An app with an embedded interpreter, PowerShell calls, a reverse tunnel, and model-generated code is what Defender and enterprise EDR flag as malware. You also need Windows code signing and Apple notarization.
- A 50 MB installer that pulls 4 GB on first launch is hard in your own beachhead markets.

### 8. Licensing

As far as I know, Qwen2.5-3B ships under a non-Apache research-style license, unlike the 1.5B and 7B. If so, your core model may not be legally usable in a commercial product without a separate agreement. **Verify this before anything else.** Also audit Piper's voice and data licenses and its espeak dependency (GPL).

Safer base-model candidates to evaluate: Qwen3-4B (Apache), Phi-4-mini (MIT), or Qwen2.5-7B (Apache, if you can fit it).

---

## The phone bridge (your "ultimate differentiator")

- Laptops sleep, close their lids, and drop off Wi-Fi. Wake-on-LAN generally doesn't work over Wi-Fi, and it doesn't work from outside the LAN without extra infrastructure.
- A phone number that can command a machine holding client data and send email needs strong authentication. Caller ID is spoofable, and there's no voice-auth or confirm-before-act design in the plan.
- Prompt injection is unaddressed. A 3B model reading documents and web pages with email and file tools is an exfiltration path.
- Telegram bots don't support end-to-end encryption, so "E2E, zero leakage" is false for that channel.
- **WhatsApp:** Meta changed its Business API terms to bar general-purpose AI chatbots from the platform, effective January 15, 2026. A voice-driven general assistant over the WhatsApp Business API is exactly the thing that policy targets. (See sources below.)
- Frontier labs and Microsoft are shipping phone-to-desktop agent handoff themselves. Your busy-founder Ultra buyer already has $20 alternatives that work better.

---

## Financial model errors

- **Year 1 EBITDA is wrong.** You treat year-end ARR ($1.044M) as full-year revenue (1.044M − 105k COGS − 750k opex = +189k). But your own milestones ramp MRR from $0 at Month 3 to $18k at M6, $50k at M9, and $85k at M12. Interpolating, recognized Y1 revenue is roughly $350–400k, so Y1 EBITDA is about **−$400k, not +$189k**. The "positive EBITDA from Year 1" claim in the abstract is false.
- **Opex equals the entire raise.** Y1 opex is $750k, the same as the round, with no founder salaries, no buffer, and no SOC2 or audit costs beyond $60k.
- **Three versions of the Year 3 number:**
  - Abstract: $25.8M ARR
  - Section 2: $18.5M "SOM" built on ~120k paid users
  - Section 5: 121k paid plus 32k seats gives $25.8M

  Also, TAM is $1.3T (all enterprise software) and SAM is the $45B market. Neither is a bottom-up number.
- **Gross margin is stated three ways:** ~90%, 92–95%, and 84–90% by tier.
- **Table arithmetic:** the model sizes sum to 4.96 GB, not the stated ~3.96 GB.
- **Memory figures:** the plan gives <2.8 GB, <2.5 GB, and a 4.4 GB pool for the same app. The "1,942 MB headroom" is just the leftover from your own ceiling, not a measurement. A real 8 GB user is running Excel, Chrome, and Teams, so the 3.2–3.8 GB host OS estimate is optimistic.
- **Conversion and churn:** about 4–5% free-to-paid, zero modeled churn, and no CAC anywhere.
- **Unmodeled costs:** the Turbo Router's 70B cloud inference isn't in COGS, and support at $0.50/user ignores crashes, OOM, and weird hardware. For desktop software across three OSes, that's your real cost center.
- **Pricing vs. market:** you target the developing world but price in USD at $12 and $29 a month. Card penetration is low, Stripe isn't available in many of those countries, and willingness to pay is far below that.
- **Piracy:** license-key gating on an open-source, offline app is trivially bypassed. Anyone can fork and unlock Pro.
- **Enterprise math:** $8/seat with a 20-seat minimum is $1,920 ACV, and that "includes" dedicated support, custom fine-tuning, and an independent audit. No sales motion works at that ACV.

---

## Moat and competition

- The moat is thin. Open models, `llama.cpp`, open libraries, and skills as copyable markdown files. The `SKILL.md` format is an open pattern, and anyone can clone your Skill Store.
- Real competitors are missing from the plan. Claude Cowork-style desktop agents, Copilot's Office agents, and Gemini in Workspace already target non-developers. The "Claude Code requires Python and Git" comparison is a strawman, because those products no longer require that.
- Windows is shipping on-device models and NPUs. Your 8 GB gap shrinks every hardware refresh, which makes it a depreciating wedge.
- The "viral open-source on GitHub" channel targets developers, but your customers are accountants.
- OEM preinstall deals in Months 9–18 are a 12–24 month enterprise sales cycle for a 6-person team.

---

## Team, scope, and execution

- **No team section, no traction, no customer evidence.** YC invests in founders first, and this doc has none of it.
- **Spend is misallocated.** The biggest line is two C++/Rust engineers writing AVX2 kernels, but `llama.cpp` already has them. Your gap is evals, reliability, and distribution, not kernels.
- **Scope is impossible for the budget.** Three engineers must deliver three-OS automation, a Skill Store, a no-code Skill Creator, a duplex voice stack, an enterprise console, and SOC2 prep in 28 weeks.
- **Skill counts conflict:** hundreds, 500+, and 1,000+ in different places, while $112.5k also has to cover fine-tuning. Domain-expert verification of legal and tax skills costs far more than that.
- **Timeline conflict:** the roadmap puts Ultra in Weeks 15–20, but the milestones say Month 9.

---

## What would change my mind

1. **Pick one wedge.** For example, "private reconciliation and redaction for small CA/bookkeeping firms," with no phone bridge, no fleet console, and no skill marketplace.
2. **Publish a benchmark.** Run 100 real messy files through the system on an actual 8 GB i3 and report task success rate, wall-clock time, and silent-error rate. If success is under ~85% on a narrow task set, the thesis fails.
3. **Talk to 20 target users and get 5 to use a janky prototype daily.** Bring letters of intent if you can.
4. **Resolve the license question** and swap in a clearly commercial-safe model if needed.
5. **Rebuild the financials bottom-up** with monthly revenue recognition, churn, support, and CAC.
6. **Pick one privacy story:** either truly offline for enterprise, or connected for prosumers. Don't sell both on the same build.

---

## Sources and caveats

- WhatsApp Business API policy on general-purpose AI chatbots (effective Jan 15, 2026): <https://respond.io/blog/whatsapp-general-purpose-chatbots-ban> and <https://www.business-standard.com/technology/tech-news/meta-bans-ai-chatbots-from-whatsapp-business-api-chatgpt-to-shut-down-jan-2026-125102200335_1.html>
- Throughput, memory-bandwidth, and prompt-processing figures are my estimates, not benchmarks. Measure them on your target hardware.
- Model license claims (Qwen2.5-3B, Piper voices) are from my own knowledge and were not re-verified here. Check the current license files before relying on them.
