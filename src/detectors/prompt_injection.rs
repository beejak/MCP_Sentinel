//! Prompt injection detection

use anyhow::Result;

use crate::models::vulnerability::Vulnerability;

/// Detect prompt injection attempts
pub fn detect(_content: &str) -> Result<Vec<Vulnerability>> {
    // Phase 1 implementation
    Ok(Vec::new())
}
