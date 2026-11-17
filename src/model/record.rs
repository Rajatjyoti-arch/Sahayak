use crate::currency::Paise;
use std::fmt;

/// Error conditions produced during record primitive construction or validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordError {
    /// The record identifier is empty or consists entirely of whitespace.
    EmptyRecordId,
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRecordId => {
                write!(f, "record identifier cannot be empty or whitespace-only")
            }
        }
    }
}

impl std::error::Error for RecordError {}

/// A strongly typed newtype wrapper representing a source row identifier.
///
/// Invariants:
/// - The identifier is trimmed of leading and trailing whitespace at construction.
/// - The identifier is non-empty (`len() > 0`).
///
/// Note on Uniqueness: `RecordId` establishes syntactic validity of the identifier string.
/// Physical uniqueness across the dataset is an independent domain invariant enforced
/// at partition and row-conservation boundaries.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordId(String);

impl RecordId {
    /// Constructs a validated `RecordId`.
    ///
    /// Trims leading and trailing whitespace. Returns `Err(RecordError::EmptyRecordId)`
    /// if the trimmed identifier is empty.
    pub fn new(id: impl Into<String>) -> Result<Self, RecordError> {
        let s = id.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(RecordError::EmptyRecordId)
        } else if trimmed.len() == s.len() {
            Ok(Self(s))
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Borrows the inner identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for RecordId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Represents a validated record from the Department Sanction Spreadsheet (File A).
///
/// Fields remain private to protect the canonical record representation. Access to field
/// data is provided via read-only borrowing accessors.
///
/// Complete records deliberately do not implement `Clone`: domain records represent physical
/// rows that move through reconciliation partitions (Matched, Unmatched, Duplicate, Invalid, Pending).
#[derive(Debug, PartialEq, Eq)]
pub struct SanctionRecord {
    source_row_id: RecordId,
    application_no: Option<String>,
    beneficiary_name: String,
    parentage: Option<String>,
    sanctioned_amount: Paise,
    masked_account_ref: Option<String>,
    bank_ifsc: Option<String>,
    tehsil: Option<String>,
}

impl SanctionRecord {
    /// Constructs a new `SanctionRecord`.
    ///
    /// Values are stored as provided without implicit normalization. Text normalization
    /// (e.g. name lowercase stripping) is deferred to key generation and matching.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_row_id: RecordId,
        application_no: Option<String>,
        beneficiary_name: String,
        parentage: Option<String>,
        sanctioned_amount: Paise,
        masked_account_ref: Option<String>,
        bank_ifsc: Option<String>,
        tehsil: Option<String>,
    ) -> Self {
        Self {
            source_row_id,
            application_no,
            beneficiary_name,
            parentage,
            sanctioned_amount,
            masked_account_ref,
            bank_ifsc,
            tehsil,
        }
    }

    /// Returns a reference to the source row identifier.
    pub fn source_row_id(&self) -> &RecordId {
        &self.source_row_id
    }

    /// Returns the application number if present.
    pub fn application_no(&self) -> Option<&str> {
        self.application_no.as_deref()
    }

    /// Returns a borrowed slice of the beneficiary name.
    pub fn beneficiary_name(&self) -> &str {
        &self.beneficiary_name
    }

    /// Returns the parentage if present.
    pub fn parentage(&self) -> Option<&str> {
        self.parentage.as_deref()
    }

    /// Returns the sanctioned amount in integer paise by value (`Paise` is `Copy`).
    pub fn sanctioned_amount(&self) -> Paise {
        self.sanctioned_amount
    }

    /// Returns the masked account reference if present.
    pub fn masked_account_ref(&self) -> Option<&str> {
        self.masked_account_ref.as_deref()
    }

    /// Returns the bank IFSC if present.
    pub fn bank_ifsc(&self) -> Option<&str> {
        self.bank_ifsc.as_deref()
    }

    /// Returns the administrative tehsil if present.
    pub fn tehsil(&self) -> Option<&str> {
        self.tehsil.as_deref()
    }
}

/// Represents a validated record from the Bank Disbursement Log (File B).
///
/// Fields remain private to protect the canonical record representation. Access to field
/// data is provided via read-only borrowing accessors.
///
/// Complete records deliberately do not implement `Clone`: domain records represent physical
/// rows that move through reconciliation partitions (Matched, Unmatched, Duplicate, Invalid, Pending).
#[derive(Debug, PartialEq, Eq)]
pub struct DisbursementRecord {
    source_row_id: RecordId,
    application_no: Option<String>,
    beneficiary_name: String,
    parentage: Option<String>,
    disbursed_amount: Paise,
    masked_account_ref: Option<String>,
    bank_ifsc: Option<String>,
    transaction_ref: Option<String>,
}

impl DisbursementRecord {
    /// Constructs a new `DisbursementRecord`.
    ///
    /// Values are stored as provided without implicit normalization.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_row_id: RecordId,
        application_no: Option<String>,
        beneficiary_name: String,
        parentage: Option<String>,
        disbursed_amount: Paise,
        masked_account_ref: Option<String>,
        bank_ifsc: Option<String>,
        transaction_ref: Option<String>,
    ) -> Self {
        Self {
            source_row_id,
            application_no,
            beneficiary_name,
            parentage,
            disbursed_amount,
            masked_account_ref,
            bank_ifsc,
            transaction_ref,
        }
    }

    /// Returns a reference to the source row identifier.
    pub fn source_row_id(&self) -> &RecordId {
        &self.source_row_id
    }

    /// Returns the application number if present.
    pub fn application_no(&self) -> Option<&str> {
        self.application_no.as_deref()
    }

    /// Returns a borrowed slice of the beneficiary name.
    pub fn beneficiary_name(&self) -> &str {
        &self.beneficiary_name
    }

    /// Returns the parentage if present.
    pub fn parentage(&self) -> Option<&str> {
        self.parentage.as_deref()
    }

    /// Returns the disbursed amount in integer paise by value (`Paise` is `Copy`).
    pub fn disbursed_amount(&self) -> Paise {
        self.disbursed_amount
    }

    /// Returns the masked account reference if present.
    pub fn masked_account_ref(&self) -> Option<&str> {
        self.masked_account_ref.as_deref()
    }

    /// Returns the bank IFSC if present.
    pub fn bank_ifsc(&self) -> Option<&str> {
        self.bank_ifsc.as_deref()
    }

    /// Returns the transaction reference if present.
    pub fn transaction_ref(&self) -> Option<&str> {
        self.transaction_ref.as_deref()
    }
}
