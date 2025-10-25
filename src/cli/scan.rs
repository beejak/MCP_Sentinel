//! Scan command implementation

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{debug, info};

use super::types::{LlmProvider, OutputFormat, ScanMode, SeverityLevel};
use crate::models::config::ScanConfig;
use crate::scanner::Scanner;

#[allow(clippy::too_many_arguments)]
pub async fn execute(
    target: String,
    mode: ScanMode,
    _llm_provider: Option<LlmProvider>,
    _llm_model: Option<String>,
    _llm_api_key: Option<String>,
    output: OutputFormat,
    output_file: Option<String>,
    _severity: SeverityLevel,
    fail_on: Option<SeverityLevel>,
    _config: Option<String>,
) -> Result<()> {
    info!("📂 Scanning: {}", target);
    debug!("Mode: {:?}", mode);
    debug!("Output format: {:?}", output);

    // Parse target path
    let target_path = PathBuf::from(&target);

    // Check if target exists
    if !target_path.exists() {
        anyhow::bail!("Target path does not exist: {}", target);
    }

    if !target_path.is_dir() {
        anyhow::bail!("Target must be a directory: {}", target);
    }

    // Create scanner configuration
    let config = ScanConfig::default();
    let scanner = Scanner::new(config);

    // Run scan
    let result = scanner
        .scan_directory(&target_path)
        .await
        .context("Scan failed")?;

    // Output results
    match output {
        OutputFormat::Terminal => {
            crate::output::terminal::render(&result)?;
        }
        OutputFormat::Json => {
            let json = crate::output::json::generate(&result)?;
            if let Some(file_path) = &output_file {
                std::fs::write(file_path, json)?;
                println!("Report saved to: {}", file_path);
            } else {
                println!("{}", json);
            }
        }
        _ => {
            anyhow::bail!("Output format {:?} not yet implemented", output);
        }
    }

    // Check fail_on threshold
    if let Some(threshold) = fail_on {
        let threshold_severity = match threshold {
            SeverityLevel::Low => crate::models::vulnerability::Severity::Low,
            SeverityLevel::Medium => crate::models::vulnerability::Severity::Medium,
            SeverityLevel::High => crate::models::vulnerability::Severity::High,
            SeverityLevel::Critical => crate::models::vulnerability::Severity::Critical,
        };

        if result.has_issues_at_level(threshold_severity) {
            anyhow::bail!("Found vulnerabilities at or above {:?} level", threshold);
        }
    }

    Ok(())
}
