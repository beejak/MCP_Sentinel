//! Path traversal detection - CWE-22

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::vulnerability::{Location, Severity, Vulnerability, VulnerabilityType};

static PATH_TRAVERSAL_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r#"\.\./"#).unwrap(),
        Regex::new(r#"\.\.\\"#).unwrap(),
        Regex::new(r#"%2e%2e/"#).unwrap(),
        Regex::new(r#"\.\.\.\.//\.\.\.\./"#).unwrap(),
        Regex::new(r#"open\s*\([^)]*\+[^)]*\)"#).unwrap(), // open() with concatenation
    ]
});

pub fn detect(content: &str, file_path: &str) -> Result<Vec<Vulnerability>> {
    let mut vulnerabilities = Vec::new();
    let mut id_counter = 1;

    for (line_num, line) in content.lines().enumerate() {
        for pattern in PATH_TRAVERSAL_PATTERNS.iter() {
            if pattern.is_match(line) {
                let vuln = Vulnerability::new(
                    format!("PATH-TRAV-{:03}", id_counter),
                    VulnerabilityType::PathTraversal,
                    Severity::High,
                    "Path Traversal Pattern Detected",
                    "Potential directory traversal vulnerability detected",
                )
                .with_location(Location::new(file_path).with_line(line_num + 1))
                .with_impact("Attackers can access files outside intended directory")
                .with_remediation("Validate and sanitize file paths, use os.path.abspath(), check path prefix")
                .with_code_snippet(line.to_string())
                .with_confidence(0.75);

                vulnerabilities.push(vuln);
                id_counter += 1;
                break;
            }
        }
    }

    Ok(vulnerabilities)
}
