//! Schema version constants.
//!
//! Every request carries a `schema_version` field so the backend can
//! detect mismatches before attempting deserialization.

/// Current API schema version (ISO date of contract freeze).
pub const CURRENT_SCHEMA_VERSION: &str = "2026-06-15";

/// Minimum supported schema version.
pub const MIN_SCHEMA_VERSION: &str = "2026-06-15";
