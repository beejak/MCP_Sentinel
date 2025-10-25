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
            let vulns = self.scan_file(file).await?;
            result.add_vulnerabilities(vulns);
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
    async fn scan_file(&self, path: &Path) -> Result<Vec<crate::models::Vulnerability>> {
        let mut vulnerabilities = Vec::new();

        // Read file content
        let content = match crate::utils::file::read_file(path) {
            Ok(c) => c,
            Err(e) => {
                debug!("Failed to read {}: {}", path.display(), e);
                return Ok(vulnerabilities);
            }
        };

        let file_path = path.to_string_lossy().to_string();

        // Run all detectors
        debug!("Running detectors on {}", file_path);

        // 1. Secrets detection
        if let Ok(vulns) = crate::detectors::secrets::detect(&content, &file_path) {
            vulnerabilities.extend(vulns);
        }

        // 2. Command injection detection
        if let Ok(vulns) = crate::detectors::code_vulns::detect_command_injection(&content, &file_path) {
            vulnerabilities.extend(vulns);
        }

        // 3. Sensitive file access detection
        if let Ok(vulns) = crate::detectors::code_vulns::detect_sensitive_file_access(&content, &file_path) {
            vulnerabilities.extend(vulns);
        }

        // 4. Tool poisoning detection
        if let Ok(vulns) = crate::detectors::tool_poisoning::detect(&content) {
            vulnerabilities.extend(vulns);
        }

        // 5. Prompt injection detection
        if let Ok(vulns) = crate::detectors::prompt_injection::detect(&content) {
            vulnerabilities.extend(vulns);
        }

        Ok(vulnerabilities)
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
