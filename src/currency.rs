use std::fmt;
use std::str::FromStr;

/// Error type for currency parsing and arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrencyError {
    EmptyInput,
    NegativeAmount,
    ExcessPrecision(usize),
    MultipleDecimalPoints,
    InvalidCharacter(char),
    MalformedGrouping,
    PrefixOnly,
    ArithmeticOverflow,
    ArithmeticUnderflow,
}

impl fmt::Display for CurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "currency input is empty"),
            Self::NegativeAmount => write!(f, "negative currency amount not permitted"),
            Self::ExcessPrecision(digits) => {
                write!(f, "excess precision ({digits} decimals); max 2 allowed")
            }
            Self::MultipleDecimalPoints => write!(f, "multiple decimal points encountered"),
            Self::InvalidCharacter(c) => write!(f, "invalid character '{c}' in currency string"),
            Self::MalformedGrouping => write!(f, "malformed digit comma grouping"),
            Self::PrefixOnly => write!(f, "currency string contains only a prefix"),
            Self::ArithmeticOverflow => write!(f, "currency arithmetic overflow"),
            Self::ArithmeticUnderflow => write!(f, "currency arithmetic underflow"),
        }
    }
}

impl std::error::Error for CurrencyError {}

/// Represents a non-negative monetary quantity in integer paise.
///
/// Invariant: `0 <= amount <= i64::MAX`.
///
/// Note on Financial Aggregation: Cumulative ledger totals (e.g. total sanctioned,
/// total disbursed) across entire datasets should use checked `i128` arithmetic or
/// `Variance::checked_add` rather than summing an unbounded number of individual records
/// into `Paise::checked_add`, preventing aggregate overflow in high-volume reconciliation runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Paise(i64);

impl Paise {
    pub const ZERO: Self = Self(0);

    /// Constructs `Paise` from a raw integer count of paise.
    ///
    /// Returns `Err(CurrencyError::NegativeAmount)` if `amount < 0`.
    pub fn from_paise(amount: i64) -> Result<Self, CurrencyError> {
        if amount < 0 {
            Err(CurrencyError::NegativeAmount)
        } else {
            Ok(Self(amount))
        }
    }

    /// Constructs `Paise` from whole rupees, multiplying by 100 with overflow checking.
    ///
    /// Maximum accepted value is `92_233_720_368_547_758` rupees (since `* 100 <= i64::MAX`).
    /// Returns `Err(CurrencyError::NegativeAmount)` if `rupees < 0`.
    /// Returns `Err(CurrencyError::ArithmeticOverflow)` if `rupees * 100 > i64::MAX`.
    pub fn from_rupees(rupees: i64) -> Result<Self, CurrencyError> {
        if rupees < 0 {
            return Err(CurrencyError::NegativeAmount);
        }
        rupees
            .checked_mul(100)
            .ok_or(CurrencyError::ArithmeticOverflow)
            .and_then(Self::from_paise)
    }

    /// Returns the underlying paise amount as an `i64`.
    pub const fn as_paise(self) -> i64 {
        self.0
    }

    /// Returns the whole rupee component (`paise / 100`).
    pub const fn rupees(self) -> i64 {
        self.0 / 100
    }

    /// Returns the fractional subunit paise component (`paise % 100`).
    pub const fn subunit_paise(self) -> i64 {
        self.0 % 100
    }

    /// Performs checked addition, returning an error on overflow.
    pub fn checked_add(self, rhs: Self) -> Result<Self, CurrencyError> {
        self.0
            .checked_add(rhs.0)
            .ok_or(CurrencyError::ArithmeticOverflow)
            .and_then(Self::from_paise)
    }

    /// Performs checked subtraction, returning `ArithmeticUnderflow` if `rhs > self`.
    pub fn checked_sub(self, rhs: Self) -> Result<Self, CurrencyError> {
        if self.0 < rhs.0 {
            return Err(CurrencyError::ArithmeticUnderflow);
        }
        self.0
            .checked_sub(rhs.0)
            .ok_or(CurrencyError::ArithmeticUnderflow)
            .map(Self)
    }

    /// Calculates the signed financial difference (`self - rhs`) as a `Variance`.
    ///
    /// Because both values are non-negative `i64`, conversion to `i128` is guaranteed
    /// never to overflow or underflow.
    pub fn difference(self, rhs: Self) -> Variance {
        let lhs_128 = self.0 as i128;
        let rhs_128 = rhs.0 as i128;
        Variance(lhs_128 - rhs_128)
    }
}

/// Represents a signed ledger difference between two financial amounts in integer paise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variance(i128);

impl Variance {
    pub const ZERO: Self = Self(0);

    /// Constructs a `Variance` directly from signed paise.
    pub const fn from_paise(amount: i128) -> Self {
        Self(amount)
    }

    /// Returns the underlying signed paise amount as an `i128`.
    pub const fn as_paise(self) -> i128 {
        self.0
    }

    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Performs checked addition of two variances.
    pub fn checked_add(self, rhs: Self) -> Result<Self, CurrencyError> {
        self.0
            .checked_add(rhs.0)
            .map(Self)
            .ok_or(CurrencyError::ArithmeticOverflow)
    }
}

impl FromStr for Paise {
    type Err = CurrencyError;

    /// Parses a currency string into non-negative `Paise`.
    ///
    /// Supported prefixes (case-insensitive): `₹`, `INR`, `Rs.`, `Rs`.
    /// Supported groupings: Indian (`12,34,567.89`, `1,00,000.00`) and Western (`1,000,000.00`).
    /// Supported decimals: exactly 0, 1 (`.5` -> 50 paise), or 2 digits (`.50` -> 50 paise).
    ///
    /// Rejects negative amounts, excess precision, multiple dots, and malformed comma grouping.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(CurrencyError::EmptyInput);
        }

        // 1. Identify and strip currency prefix
        let (after_prefix, had_prefix) = strip_currency_prefix(trimmed);

        // 2. Remove all whitespace from amount portion (matching Python re.sub(r"\s+", "", cleaned))
        let mut cleaned = String::with_capacity(after_prefix.len());
        for c in after_prefix.chars() {
            if !c.is_whitespace() {
                cleaned.push(c);
            }
        }

        if cleaned.is_empty() {
            if had_prefix {
                return Err(CurrencyError::PrefixOnly);
            } else {
                return Err(CurrencyError::EmptyInput);
            }
        }

        // 3. Reject negative amounts
        if cleaned.starts_with('-') {
            return Err(CurrencyError::NegativeAmount);
        }

        // 4. Split whole rupees and fractional decimals
        let dot_count = cleaned.chars().filter(|&c| c == '.').count();
        if dot_count > 1 {
            return Err(CurrencyError::MultipleDecimalPoints);
        }

        let (whole_str, frac_paise) = if dot_count == 1 {
            let (whole, frac) = cleaned
                .split_once('.')
                .expect("dot_count is 1, split_once must succeed");

            if frac.is_empty() {
                return Err(CurrencyError::InvalidCharacter('.'));
            }

            // Verify all characters in fractional part are ASCII digits
            for c in frac.chars() {
                if !c.is_ascii_digit() {
                    return Err(CurrencyError::InvalidCharacter(c));
                }
            }

            if frac.len() > 2 {
                return Err(CurrencyError::ExcessPrecision(frac.len()));
            }

            let parsed_frac = match frac.len() {
                1 => (frac.as_bytes()[0] - b'0') as i64 * 10,
                2 => {
                    let d1 = (frac.as_bytes()[0] - b'0') as i64;
                    let d2 = (frac.as_bytes()[1] - b'0') as i64;
                    d1 * 10 + d2
                }
                _ => unreachable!(),
            };

            (whole, parsed_frac)
        } else {
            (cleaned.as_str(), 0)
        };

        if whole_str.is_empty() {
            return Err(CurrencyError::InvalidCharacter('.'));
        }

        // 5. Validate comma grouping and characters
        if whole_str.starts_with(',') || whole_str.ends_with(',') {
            return Err(CurrencyError::MalformedGrouping);
        }

        let chunks: Vec<&str> = whole_str.split(',').collect();

        // Check for invalid characters across all chunks
        for chunk in &chunks {
            if chunk.is_empty() {
                return Err(CurrencyError::MalformedGrouping);
            }
            for c in chunk.chars() {
                if !c.is_ascii_digit() {
                    return Err(CurrencyError::InvalidCharacter(c));
                }
            }
        }

        if chunks.len() > 1 {
            let last_chunk = chunks.last().unwrap();
            if last_chunk.len() != 3 {
                return Err(CurrencyError::MalformedGrouping);
            }

            // Western: 1st group 1..=3 digits, all subsequent groups exactly 3 digits
            let is_western =
                (1..=3).contains(&chunks[0].len()) && chunks[1..].iter().all(|c| c.len() == 3);

            // Indian: 1st group 1..=2 digits, all intermediate groups 2 digits, last group 3 digits
            let is_indian = (1..=2).contains(&chunks[0].len())
                && chunks[1..chunks.len() - 1].iter().all(|c| c.len() == 2)
                && last_chunk.len() == 3;

            if !is_western && !is_indian {
                return Err(CurrencyError::MalformedGrouping);
            }
        }

        // 6. Accumulate whole rupee digits with checked arithmetic
        let mut rupees: i64 = 0;
        for c in whole_str.chars() {
            if c == ',' {
                continue;
            }
            let digit = (c as u8 - b'0') as i64;
            rupees = rupees
                .checked_mul(10)
                .ok_or(CurrencyError::ArithmeticOverflow)?
                .checked_add(digit)
                .ok_or(CurrencyError::ArithmeticOverflow)?;
        }

        let total_paise = rupees
            .checked_mul(100)
            .ok_or(CurrencyError::ArithmeticOverflow)?
            .checked_add(frac_paise)
            .ok_or(CurrencyError::ArithmeticOverflow)?;

        Paise::from_paise(total_paise)
    }
}

/// Strips known currency prefixes (₹, INR, Rs., Rs), case-insensitively.
fn strip_currency_prefix(s: &str) -> (&str, bool) {
    if let Some(rest) = s.strip_prefix('₹') {
        return (rest.trim_start(), true);
    }

    let lower = s.to_ascii_lowercase();
    if lower.starts_with("inr") {
        return (s[3..].trim_start(), true);
    }
    if lower.starts_with("rs.") {
        return (s[3..].trim_start(), true);
    }
    if lower.starts_with("rs") {
        return (s[2..].trim_start(), true);
    }

    (s, false)
}

/// Formats a signed `i128` paise amount using Indian comma grouping (`₹ 12,34,567.89`).
///
/// Uses `.unsigned_abs()` to safely format `i128::MIN` without arithmetic negation overflow.
fn format_inr(paise: i128) -> String {
    let sign = if paise < 0 { "-" } else { "" };
    let abs_paise = paise.unsigned_abs();
    let rupees = abs_paise / 100;
    let rem_paise = (abs_paise % 100) as u8;

    let s = rupees.to_string();
    let formatted_rupees = if s.len() <= 3 {
        s
    } else {
        let last3 = &s[s.len() - 3..];
        let mut remaining = &s[..s.len() - 3];
        let mut groups = Vec::new();

        while remaining.len() > 2 {
            groups.push(&remaining[remaining.len() - 2..]);
            remaining = &remaining[..remaining.len() - 2];
        }
        if !remaining.is_empty() {
            groups.push(remaining);
        }
        groups.reverse();
        groups.push(last3);
        groups.join(",")
    };

    format!("{sign}₹ {formatted_rupees}.{rem_paise:02}")
}

impl fmt::Display for Paise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_inr(self.0 as i128))
    }
}

impl fmt::Display for Variance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_inr(self.0))
    }
}
