//! Main scanner API
//!
//! This module provides the core scanning functionality for MCP Sentinel.
//! It orchestrates file discovery, content analysis, and vulnerability detection
//! across multiple security detectors.
//!
//! # Architecture
//!
//! The scanner operates in phases:
//! 1. **Discovery**: Find all scannable files using glob patterns
//! 2. **Scanning**: Analyze each file with all enabled detectors
//! 3. **Aggregation**: Collect and organize all vulnerabilities
//! 4. **Scoring**: Calculate risk scores and generate summaries
//!
//! # Error Handling
//!
//! The scanner uses graceful degradation:
//! - Individual detector failures are logged but don't stop the scan
//! - Unreadable files (binary, permissions) are skipped with debug logs
//! - Critical failures (directory not found) return errors immediately
//!
//! # Performance
//!
//! - Files are scanned sequentially (parallel scanning planned for Phase 2)
//! - Regex patterns are compiled once using Lazy static
//! - File content is read into memory (acceptable for MCP servers, typically <10MB)

use anyhow::{Context, Result};
use std::path::Path;
use std::time::Instant;
use tracing::{debug, error, info, warn};

use crate::models::{config::ScanConfig, scan_result::ScanResult};

/// Main scanner struct that coordinates vulnerability detection
///
/// The scanner uses a configuration object to control which files are scanned
/// and how detectors behave. It maintains no internal state between scans,
/// making it safe to reuse for multiple scanning operations.
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
        debug!("Discovering files in {}...", path.display());
        let files = match crate::utils::file::discover_files(path, &self.config.exclude_patterns) {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to discover files in {}: {}", path.display(), e);
                return Err(e).context("Failed to discover files");
            }
        };
        info!("Found {} files to scan", files.len());

        if files.is_empty() {
            warn!("No scannable files found in {}. Looking for: .py, .js, .ts, .jsx, .tsx, .json, .yaml", path.display());
        }

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

    /// Scan a single file with all enabled detectors
    ///
    /// This method orchestrates running all security detectors on a single file.
    /// Each detector runs independently and failures in one detector don't affect others.
    ///
    /// # Error Handling Strategy
    ///
    /// - **File read failures**: Skipped with debug log (binary files, permissions, UTF-8 issues)
    /// - **Detector failures**: Logged at WARN level but don't stop other detectors
    /// - **Success**: Returns all found vulnerabilities (can be empty vector)
    ///
    /// # Detectors Run (in order) - v1.5.0
    ///
    /// **Phase 1.0 Detectors:**
    /// 1. Secrets detection - API keys, credentials, tokens
    /// 2. Command injection - os.system(), subprocess, etc.
    /// 3. Sensitive file access - SSH keys, AWS credentials, cookies
    /// 4. Tool poisoning - Malicious tool descriptions, invisible Unicode
    /// 5. Prompt injection - LLM manipulation attempts
    ///
    /// **Phase 1.5 Detectors (NEW):**
    /// 6. Code injection - eval(), exec(), dynamic code execution
    /// 7. Insecure deserialization - pickle, yaml, marshal
    /// 8. Path traversal - Directory traversal patterns
    /// 9. SQL injection - String concatenation in queries
    /// 10. SSRF - Server-side request forgery
    async fn scan_file(&self, path: &Path) -> Result<Vec<crate::models::Vulnerability>> {
        let mut vulnerabilities = Vec::new();

        // Read file content
        let content = match crate::utils::file::read_file(path) {
            Ok(c) => c,
            Err(e) => {
                // Common scenarios: binary files, permission denied, invalid UTF-8
                // These are expected and not errors - we simply skip them
                debug!("Skipping file {}: {}", path.display(), e);
                return Ok(vulnerabilities);
            }
        };

        let file_path = path.to_string_lossy().to_string();

        // Run all detectors independently
        // Each detector runs even if previous ones fail
        debug!("Running detectors on {}", file_path);

        // 1. Secrets detection
        match crate::detectors::secrets::detect(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Secrets detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Secrets detector failed on {}: {}", file_path, e),
        }

        // 2. Command injection detection
        match crate::detectors::code_vulns::detect_command_injection(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Command injection detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Command injection detector failed on {}: {}", file_path, e),
        }

        // 3. Sensitive file access detection
        match crate::detectors::code_vulns::detect_sensitive_file_access(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Sensitive file detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Sensitive file detector failed on {}: {}", file_path, e),
        }

        // 4. Tool poisoning detection
        match crate::detectors::tool_poisoning::detect(&content) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Tool poisoning detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Tool poisoning detector failed on {}: {}", file_path, e),
        }

        // 5. Prompt injection detection
        match crate::detectors::prompt_injection::detect(&content) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Prompt injection detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Prompt injection detector failed on {}: {}", file_path, e),
        }

        // ===== NEW v1.5.0 DETECTORS =====

        // 6. Code injection detection (eval, exec, dynamic execution)
        match crate::detectors::code_injection::detect(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Code injection detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Code injection detector failed on {}: {}", file_path, e),
        }

        // 7. Insecure deserialization detection
        match crate::detectors::deserialization::detect(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Deserialization detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Deserialization detector failed on {}: {}", file_path, e),
        }

        // 8. Path traversal detection
        match crate::detectors::path_traversal::detect(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("Path traversal detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("Path traversal detector failed on {}: {}", file_path, e),
        }

        // 9. SQL injection detection
        match crate::detectors::sql_injection::detect(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("SQL injection detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("SQL injection detector failed on {}: {}", file_path, e),
        }

        // 10. SSRF detection
        match crate::detectors::ssrf::detect(&content, &file_path) {
            Ok(vulns) => {
                if !vulns.is_empty() {
                    debug!("SSRF detector found {} issues in {}", vulns.len(), file_path);
                }
                vulnerabilities.extend(vulns)
            },
            Err(e) => warn!("SSRF detector failed on {}: {}", file_path, e),
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
