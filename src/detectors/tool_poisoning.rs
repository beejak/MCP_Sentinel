//! Tool poisoning detection

use anyhow::Result;

use crate::models::vulnerability::Vulnerability;

/// Detect tool poisoning attacks in MCP tool descriptions
pub fn detect(_content: &str) -> Result<Vec<Vulnerability>> {
    // Phase 1 implementation
    Ok(Vec::new())
}
