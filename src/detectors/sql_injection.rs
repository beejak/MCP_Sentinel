//! SQL injection detection - CWE-89

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::vulnerability::{Location, Severity, Vulnerability, VulnerabilityType};

static SQL_INJECTION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r#"execute\s*\([^)]*\+[^)]*\)"#).unwrap(),
        Regex::new(r#"execute\s*\([^)]*%[^)]*\)"#).unwrap(),
        Regex::new(r#"execute\s*\([^)]*f["'][^"']*\{[^}]*\}"#).unwrap(),
        Regex::new(r#"\.raw\s*\([^)]*\+[^)]*\)"#).unwrap(),
        Regex::new(r#"query\s*\([^)]*\+[^)]*\)"#).unwrap(),
    ]
});

pub fn detect(content: &str, file_path: &str) -> Result<Vec<Vulnerability>> {
    let mut vulnerabilities = Vec::new();
    let mut id_counter = 1;

    for (line_num, line) in content.lines().enumerate() {
        for pattern in SQL_INJECTION_PATTERNS.iter() {
            if pattern.is_match(line) {
                let vuln = Vulnerability::new(
                    format!("SQL-INJ-{:03}", id_counter),
                    VulnerabilityType::SqlInjection,
                    Severity::Critical,
                    "SQL Injection Pattern Detected",
                    "Potential SQL injection via string concatenation",
                )
                .with_location(Location::new(file_path).with_line(line_num + 1))
                .with_impact("Database compromise, data theft, authentication bypass")
                .with_remediation("Use parameterized queries or prepared statements")
                .with_code_snippet(line.to_string())
                .with_confidence(0.85);

                vulnerabilities.push(vuln);
                id_counter += 1;
                break;
            }
        }
    }

    Ok(vulnerabilities)
}
