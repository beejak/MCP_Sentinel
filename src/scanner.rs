//! Main scanner API

use anyhow::{Context, Result};
use std::path::Path;
use std::time::Instant;
use tracing::{debug, info};

use crate::models::{config::ScanConfig, scan_result::ScanResult};

/// Main scanner struct
pub struct Scanner {
    config: ScanConfig,
}

impl Scanner {
    /// Create a new scanner with the given configuration
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }

    /// Scan a directory
    pub async fn scan_directory(&self, path: impl AsRef<Path>) -> Result<ScanResult> {
        let path = path.as_ref();
        info!("Scanning directory: {}", path.display());

        let start = Instant::now();

        // Create result
        let mut result = ScanResult::new(
            path.to_string_lossy().to_string(),
            vec!["static".to_string()],
        );

        // Phase 1: Discover files
        debug!("Discovering files...");
        let files = crate::utils::file::discover_files(path, &self.config.exclude_patterns)
            .context("Failed to discover files")?;
        info!("Found {} files to scan", files.len());

        // Phase 1: Scan each file
        for file in &files {
            debug!("Scanning file: {}", file.display());
            // TODO: Implement actual scanning
            // let vulns = self.scan_file(file).await?;
            // result.add_vulnerabilities(vulns);
        }

        // Set scan duration
        let duration = start.elapsed();
        result.set_duration(duration.as_millis() as u64);

        info!(
            "Scan complete: {} issues found in {}ms",
            result.summary.total_issues, result.metadata.scan_duration_ms
        );

        Ok(result)
    }

    /// Scan a single file
    async fn _scan_file(&self, _path: &Path) -> Result<Vec<crate::models::Vulnerability>> {
        // Phase 1 implementation
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scanner_creation() {
        let config = ScanConfig::default();
        let _scanner = Scanner::new(config);
    }
}
