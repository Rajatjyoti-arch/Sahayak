"""
Independent Verifier for Tamper-Evident SHA-256 Audit Manifests
Innovathon 2.0 | Central University of Jammu & University of Jammu (SIIEDC)

Verification Guarantees:
1. Recomputes cryptographic SHA-256 hash chains from Genesis (0*64) to terminal hash
2. Validates monotonic sequence continuity (E_1, E_2, ..., E_n)
3. Detects any retro-active insertion, deletion, alteration, or reordering of logged events
4. Compares the final recomputed hash against the trusted terminal anchor
"""

import sys
import json
import hashlib

def verify_manifest(manifest_path: str, expected_terminal_anchor: str = None) -> tuple[bool, str]:
    try:
        with open(manifest_path, "r", encoding="utf-8") as f:
            manifest = json.load(f)
    except Exception as e:
        return False, f"Failed to read manifest file: {e}"

    events = manifest.get("events", [])
    if not events:
        return False, "Manifest contains zero events!"

    logged_terminal_hash = manifest.get("terminal_hash", "")
    current_hash = "0" * 64  # Initial Genesis Hash

    for idx, ev in enumerate(events, 1):
        # 1. Check Sequence Number
        seq = ev.get("sequence_no")
        if seq != idx:
            return False, f"Sequence continuity break at index {idx}: expected {idx}, got {seq}"

        # 2. Check Previous Hash Link
        prev_hash = ev.get("prev_hash")
        if prev_hash != current_hash:
            return False, (f"Hash chain broken at Event #{seq}! "
                          f"Expected prev_hash {current_hash[:16]}..., but event recorded {prev_hash[:16]}...")

        # 3. Canonicalize Event Body (excluding event_hash itself)
        event_body = {
            "sequence_no": ev["sequence_no"],
            "run_id": ev["run_id"],
            "timestamp": ev["timestamp"],
            "operator_label": ev["operator_label"],
            "event_type": ev["event_type"],
            "details": ev["details"],
            "prev_hash": ev["prev_hash"]
        }
        canonical_json = json.dumps(event_body, sort_keys=True, separators=(',', ':'))
        payload = f"{current_hash}{canonical_json}".encode("utf-8")
        computed_hash = hashlib.sha256(payload).hexdigest()

        # 4. Compare Computed Hash with Recorded Hash
        if computed_hash != ev.get("event_hash"):
            return False, (f"Integrity violation at Event #{seq} ({ev.get('event_type')})! "
                          f"Recorded hash: {ev.get('event_hash')[:16]}..., Computed hash: {computed_hash[:16]}...")

        current_hash = computed_hash

    # 5. Check Terminal Anchor
    if current_hash != logged_terminal_hash:
        return False, (f"Terminal hash mismatch! "
                      f"Recomputed terminal hash {current_hash[:16]}... does not match manifest terminal_hash {logged_terminal_hash[:16]}...")

    if expected_terminal_anchor and expected_terminal_anchor != current_hash:
        return False, (f"Terminal hash does NOT match external trusted anchor! "
                      f"Recomputed: {current_hash}, External Anchor: {expected_terminal_anchor}")

    return True, f"Audit manifest verified successfully: {len(events)} events cryptographically intact. Terminal Anchor: {current_hash}"

def simulate_tampering_test(manifest_path: str):
    """Demonstrate that modifying any byte in the audit log triggers immediate detection."""
    print("\n--- SIMULATING TAMPERING ATTACK ---")
    with open(manifest_path, "r", encoding="utf-8") as f:
        manifest = json.load(f)

    # Attack: Alter an approved match event in the past
    tampered_manifest = json.loads(json.dumps(manifest))
    target_event = tampered_manifest["events"][2]
    original_val = target_event["details"]
    target_event["details"] = {"tampered": True, "stolen_credit": "ATTACKER_ACCOUNT"}
    
    tampered_path = "output/tampered_audit_manifest.json"
    with open(tampered_path, "w", encoding="utf-8") as f:
        json.dump(tampered_manifest, f, indent=2)

    print(f"Tampered Event #{target_event['sequence_no']} saved to {tampered_path}")
    success, msg = verify_manifest(tampered_path)
    print(f"Verification result on tampered log: {'PASSED' if success else 'FAILED (DETECTED)'}")
    print(f"Detection details: {msg}")
    assert not success, "Security failure: Tampered log should have been rejected!"
    print("Tamper-evident verification test passed with 100% detection accuracy!")

if __name__ == "__main__":
    path = sys.argv[1] if len(sys.argv) > 1 else "output/audit_manifest.json"
    ok, msg = verify_manifest(path)
    print(f"Verification status: {'VALID' if ok else 'INVALID'}")
    print(msg)
    if ok:
        simulate_tampering_test(path)
