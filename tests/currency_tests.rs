use sahayak_core::{CurrencyError, Paise, Variance};
use std::str::FromStr;

#[test]
fn test_python_parity_valid_text_amounts() {
    let cases = [
        ("5000", 500_000),
        ("5000.50", 500_050),
        ("5000.5", 500_050),
        ("₹ 5,000.50", 500_050),
        ("₹5000", 500_000),
        ("INR 1,00,000.00", 10_000_000),
        ("Rs. 25,000", 2_500_000),
        ("Rs 12,34,567.89", 123_456_789),
        ("100", 10_000),
        ("0.50", 50),
        ("0", 0),
    ];

    for (text, expected_paise) in cases {
        let parsed =
            Paise::from_str(text).unwrap_or_else(|e| panic!("Failed to parse '{text}': {e}"));
        assert_eq!(
            parsed.as_paise(),
            expected_paise,
            "Failed on input text: {text}"
        );
    }
}

#[test]
fn test_python_parity_invalid_text_amounts() {
    let invalid_cases = [
        ("5000.999", CurrencyError::ExcessPrecision(3)),
        ("₹", CurrencyError::PrefixOnly),
        ("", CurrencyError::EmptyInput),
        ("   ", CurrencyError::EmptyInput),
        ("-5000", CurrencyError::NegativeAmount),
        ("50,00", CurrencyError::MalformedGrouping),
        ("1,000,00", CurrencyError::MalformedGrouping),
        ("abc", CurrencyError::InvalidCharacter('a')),
        ("5000.50.50", CurrencyError::MultipleDecimalPoints),
        ("12,345,67", CurrencyError::MalformedGrouping),
    ];

    for (text, expected_err) in invalid_cases {
        let res = Paise::from_str(text);
        assert_eq!(
            res,
            Err(expected_err.clone()),
            "Expected error {expected_err:?} on input '{text}', got {res:?}"
        );
    }
}

#[test]
fn test_paise_constructors_and_accessors() {
    let p = Paise::from_paise(500_050).unwrap();
    assert_eq!(p.as_paise(), 500_050);
    assert_eq!(p.rupees(), 5000);
    assert_eq!(p.subunit_paise(), 50);

    assert_eq!(Paise::from_paise(-1), Err(CurrencyError::NegativeAmount));
    assert_eq!(Paise::ZERO.as_paise(), 0);
}

#[test]
fn test_from_rupees_boundaries() {
    // 5000 rupees -> 500,000 paise
    let p = Paise::from_rupees(5000).unwrap();
    assert_eq!(p.as_paise(), 500_000);

    // Negative rupees rejected
    assert_eq!(Paise::from_rupees(-500), Err(CurrencyError::NegativeAmount));

    // Maximum safe whole rupees: 92_233_720_368_547_758 * 100 <= i64::MAX
    let max_rupees = 92_233_720_368_547_758;
    let p_max = Paise::from_rupees(max_rupees).unwrap();
    assert_eq!(p_max.as_paise(), 9_223_372_036_854_775_800);

    // Overflow by 1 rupee
    assert_eq!(
        Paise::from_rupees(max_rupees + 1),
        Err(CurrencyError::ArithmeticOverflow)
    );
}

#[test]
fn test_checked_arithmetic_overflow_and_underflow() {
    let max_paise = Paise::from_paise(i64::MAX).unwrap();
    let one_paise = Paise::from_paise(1).unwrap();

    // Checked addition overflow
    assert_eq!(
        max_paise.checked_add(one_paise),
        Err(CurrencyError::ArithmeticOverflow)
    );

    // Checked subtraction underflow (cannot represent negative Paise)
    let ten_paise = Paise::from_paise(10).unwrap();
    let twenty_paise = Paise::from_paise(20).unwrap();
    assert_eq!(
        ten_paise.checked_sub(twenty_paise),
        Err(CurrencyError::ArithmeticUnderflow)
    );
}

#[test]
fn test_variance_and_difference() {
    let sanction = Paise::from_rupees(5000).unwrap();
    let disbursement = Paise::from_rupees(3500).unwrap();

    // Under-disbursed (positive variance: Sanction > Disbursement)
    let var_pos = sanction.difference(disbursement);
    assert_eq!(var_pos.as_paise(), 150_000);
    assert!(var_pos.is_positive());
    assert!(!var_pos.is_negative());
    assert!(!var_pos.is_zero());

    // Over-disbursed (negative variance: Disbursement > Sanction)
    let var_neg = disbursement.difference(sanction);
    assert_eq!(var_neg.as_paise(), -150_000);
    assert!(var_neg.is_negative());
    assert!(!var_neg.is_positive());
    assert!(!var_neg.is_zero());

    // Zero variance
    let var_zero = sanction.difference(sanction);
    assert_eq!(var_zero.as_paise(), 0);
    assert!(var_zero.is_zero());

    // Extreme difference: i64::MAX - 0 does not overflow Variance(i128)
    let max_diff = Paise::from_paise(i64::MAX).unwrap().difference(Paise::ZERO);
    assert_eq!(max_diff.as_paise(), i64::MAX as i128);

    // Variance checked add overflow
    let var_huge = Variance::from_paise(i128::MAX);
    let var_one = Variance::from_paise(1);
    assert_eq!(
        var_huge.checked_add(var_one),
        Err(CurrencyError::ArithmeticOverflow)
    );
}

#[test]
fn test_indian_formatting_parity() {
    let cases = [
        (Paise::from_paise(0).unwrap(), "₹ 0.00"),
        (Paise::from_paise(50).unwrap(), "₹ 0.50"),
        (Paise::from_paise(10_000).unwrap(), "₹ 100.00"),
        (Paise::from_paise(500_050).unwrap(), "₹ 5,000.50"),
        (Paise::from_paise(2_500_000).unwrap(), "₹ 25,000.00"),
        (Paise::from_paise(10_000_000).unwrap(), "₹ 1,00,000.00"),
        (Paise::from_paise(123_456_789).unwrap(), "₹ 12,34,567.89"),
    ];

    for (p, expected_str) in cases {
        assert_eq!(p.to_string(), expected_str);
    }
}

#[test]
fn test_variance_formatting_including_i128_min() {
    let pos_var = Variance::from_paise(150_000);
    assert_eq!(pos_var.to_string(), "₹ 1,500.00");

    let neg_var = Variance::from_paise(-150_000);
    assert_eq!(neg_var.to_string(), "-₹ 1,500.00");

    let zero_var = Variance::ZERO;
    assert_eq!(zero_var.to_string(), "₹ 0.00");

    // Critical: i128::MIN formatting must not panic on unsigned_abs
    let min_var = Variance::from_paise(i128::MIN);
    let min_str = min_var.to_string();
    assert!(min_str.starts_with("-₹ "));
}

#[test]
fn test_parsing_and_formatting_roundtrip() {
    let test_values = [
        0,
        1,
        50,
        99,
        100,
        500_050,
        10_000_000,
        123_456_789,
        9_223_372_036_854_775_800,
    ];

    for val in test_values {
        let p = Paise::from_paise(val).unwrap();
        let formatted = p.to_string();
        let parsed = Paise::from_str(&formatted)
            .unwrap_or_else(|e| panic!("Roundtrip parsing failed for '{formatted}': {e}"));
        assert_eq!(parsed, p, "Roundtrip mismatch on value {val}");
    }
}
