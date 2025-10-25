//! Vulnerability detectors
//!
//! This module contains all security vulnerability detectors.
//! Each detector is independent and implements pattern-based detection.
//!
//! # Detectors (v1.5.0)
//!
//! **Phase 1 Detectors** (v1.0.0):
//! - `secrets` - API keys, credentials, tokens (15+ patterns)
//! - `code_vulns` - Command injection, sensitive file access
//! - `tool_poisoning` - Malicious tool descriptions
//! - `prompt_injection` - LLM manipulation attempts
//!
//! **Phase 1.5 Detectors** (NEW in v1.5.0):
//! - `code_injection` - eval(), exec(), dynamic code execution (20+ patterns)
//! - `deserialization` - Unsafe pickle, yaml, marshal usage
//! - `path_traversal` - Directory traversal vulnerabilities
//! - `sql_injection` - SQL injection via string concatenation
//! - `ssrf` - Server-side request forgery patterns
//!
//! **Total**: 10 detector types with 80+ detection patterns

// Phase 1.0 detectors
pub mod code_vulns;
pub mod prompt_injection;
pub mod secrets;
pub mod tool_poisoning;

// Phase 1.5 detectors (NEW)
pub mod code_injection;
pub mod deserialization;
pub mod path_traversal;
pub mod sql_injection;
pub mod ssrf;

// Phase 2+ detectors (planned)
// pub mod pii;
// pub mod toxic_flows;
// pub mod anomalies;
