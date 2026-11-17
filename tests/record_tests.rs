use sahayak_core::{DisbursementRecord, Paise, RecordError, RecordId, SanctionRecord};

#[test]
fn test_valid_record_id_construction() {
    let id_res = RecordId::new("ROW-1001");
    assert!(id_res.is_ok());
    let id = id_res.unwrap();
    assert_eq!(id.as_str(), "ROW-1001");
    assert_eq!(id.as_ref(), "ROW-1001");
}

#[test]
fn test_empty_record_id_rejection() {
    let err = RecordId::new("").unwrap_err();
    assert_eq!(err, RecordError::EmptyRecordId);
    assert_eq!(
        format!("{err}"),
        "record identifier cannot be empty or whitespace-only"
    );
}

#[test]
fn test_whitespace_only_record_id_rejection() {
    assert_eq!(RecordId::new(" ").unwrap_err(), RecordError::EmptyRecordId);
    assert_eq!(
        RecordId::new("\t\n  \r").unwrap_err(),
        RecordError::EmptyRecordId
    );
}

#[test]
fn test_record_id_whitespace_trimming() {
    let id = RecordId::new("  REC-042 \t\n").unwrap();
    assert_eq!(id.as_str(), "REC-042");
    assert_eq!(id.as_ref(), "REC-042");
}

#[test]
fn test_record_id_display_and_as_ref() {
    let id = RecordId::new("BENEFICIARY-99").unwrap();
    assert_eq!(format!("{id}"), "BENEFICIARY-99");
    assert_eq!(id.as_ref(), "BENEFICIARY-99");
    assert_eq!(id.as_str(), "BENEFICIARY-99");
}

#[test]
fn test_record_id_traits() {
    let id1 = RecordId::new("ID-A").unwrap();
    let id2 = RecordId::new("ID-A").unwrap();
    let id3 = RecordId::new("ID-B").unwrap();

    // Clone, Eq, Ord
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
    assert!(id1 < id3);

    let cloned = id1.clone();
    assert_eq!(cloned, id1);

    // Hash into HashSet
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(id1);
    assert!(set.contains(&id2));
}

#[test]
fn test_sanction_record_construction_and_accessors() {
    let row_id = RecordId::new("S-001").unwrap();
    let amount = Paise::from_rupees(5000).unwrap();

    let record = SanctionRecord::new(
        row_id.clone(),
        Some("APP/2026/001".to_string()),
        "Sunita Devi".to_string(),
        Some("W/O Ramesh Kumar".to_string()),
        amount,
        Some("XXXX-XXXX-1234".to_string()),
        Some("SBIN0001234".to_string()),
        Some("Kupwara".to_string()),
    );

    assert_eq!(record.source_row_id(), &row_id);
    assert_eq!(record.application_no(), Some("APP/2026/001"));
    assert_eq!(record.beneficiary_name(), "Sunita Devi");
    assert_eq!(record.parentage(), Some("W/O Ramesh Kumar"));
    assert_eq!(record.sanctioned_amount(), amount);
    assert_eq!(record.sanctioned_amount().as_paise(), 500_000);
    assert_eq!(record.masked_account_ref(), Some("XXXX-XXXX-1234"));
    assert_eq!(record.bank_ifsc(), Some("SBIN0001234"));
    assert_eq!(record.tehsil(), Some("Kupwara"));
}

#[test]
fn test_disbursement_record_construction_and_accessors() {
    let row_id = RecordId::new("D-001").unwrap();
    let amount = Paise::from_rupees(4950).unwrap();

    let record = DisbursementRecord::new(
        row_id.clone(),
        Some("APP/2026/001".to_string()),
        "Sunita Devi".to_string(),
        Some("W/O Ramesh Kumar".to_string()),
        amount,
        Some("XXXX-XXXX-1234".to_string()),
        Some("SBIN0001234".to_string()),
        Some("TXN987654321".to_string()),
    );

    assert_eq!(record.source_row_id(), &row_id);
    assert_eq!(record.application_no(), Some("APP/2026/001"));
    assert_eq!(record.beneficiary_name(), "Sunita Devi");
    assert_eq!(record.parentage(), Some("W/O Ramesh Kumar"));
    assert_eq!(record.disbursed_amount(), amount);
    assert_eq!(record.disbursed_amount().as_paise(), 495_000);
    assert_eq!(record.masked_account_ref(), Some("XXXX-XXXX-1234"));
    assert_eq!(record.bank_ifsc(), Some("SBIN0001234"));
    assert_eq!(record.transaction_ref(), Some("TXN987654321"));
}

#[test]
fn test_optional_fields_none_handling() {
    let row_id_s = RecordId::new("S-NONE").unwrap();
    let amount_s = Paise::from_rupees(1000).unwrap();

    let sanction = SanctionRecord::new(
        row_id_s.clone(),
        None,
        "Amina Begum".to_string(),
        None,
        amount_s,
        None,
        None,
        None,
    );

    assert_eq!(sanction.source_row_id(), &row_id_s);
    assert_eq!(sanction.application_no(), None);
    assert_eq!(sanction.parentage(), None);
    assert_eq!(sanction.masked_account_ref(), None);
    assert_eq!(sanction.bank_ifsc(), None);
    assert_eq!(sanction.tehsil(), None);

    let row_id_d = RecordId::new("D-NONE").unwrap();
    let amount_d = Paise::from_rupees(1000).unwrap();

    let disbursement = DisbursementRecord::new(
        row_id_d.clone(),
        None,
        "Amina Begum".to_string(),
        None,
        amount_d,
        None,
        None,
        None,
    );

    assert_eq!(disbursement.source_row_id(), &row_id_d);
    assert_eq!(disbursement.application_no(), None);
    assert_eq!(disbursement.parentage(), None);
    assert_eq!(disbursement.masked_account_ref(), None);
    assert_eq!(disbursement.bank_ifsc(), None);
    assert_eq!(disbursement.transaction_ref(), None);
}

#[test]
fn test_record_equality_and_paise_preservation() {
    let row_id = RecordId::new("REC-1").unwrap();
    let amount = Paise::from_paise(123456).unwrap();

    let r1 = SanctionRecord::new(
        row_id.clone(),
        Some("APP-1".to_string()),
        "Mohammad Rafiq".to_string(),
        Some("S/O Abdul Gani".to_string()),
        amount,
        Some("1234".to_string()),
        Some("JAKA0001".to_string()),
        Some("Baramulla".to_string()),
    );

    let r2 = SanctionRecord::new(
        row_id.clone(),
        Some("APP-1".to_string()),
        "Mohammad Rafiq".to_string(),
        Some("S/O Abdul Gani".to_string()),
        amount,
        Some("1234".to_string()),
        Some("JAKA0001".to_string()),
        Some("Baramulla".to_string()),
    );

    let r3 = SanctionRecord::new(
        row_id,
        Some("APP-2".to_string()),
        "Mohammad Rafiq".to_string(),
        Some("S/O Abdul Gani".to_string()),
        amount,
        Some("1234".to_string()),
        Some("JAKA0001".to_string()),
        Some("Baramulla".to_string()),
    );

    assert_eq!(r1, r2);
    assert_ne!(r1, r3);
    assert_eq!(r1.sanctioned_amount(), amount);
}

