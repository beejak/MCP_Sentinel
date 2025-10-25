//! SSRF (Server-Side Request Forgery) detection - CWE-918

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::vulnerability::{Location, Severity, Vulnerability, VulnerabilityType};

static SSRF_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r#"requests\.(get|post|put|delete)\s*\([^)]*\+[^)]*\)"#).unwrap(),
        Regex::new(r#"urllib\.request\.urlopen\s*\([^)]*\+[^)]*\)"#).unwrap(),
        Regex::new(r#"fetch\s*\([^)]*\+[^)]*\)"#).unwrap(),
        Regex::new(r#"axios\.(get|post)\s*\([^)]*\+[^)]*\)"#).unwrap(),
        Regex::new(r#"http\.(get|request)\s*\([^)]*\+[^)]*\)"#).unwrap(),
    ]
});

pub fn detect(content: &str, file_path: &str) -> Result<Vec<Vulnerability>> {
    let mut vulnerabilities = Vec::new();
    let mut id_counter = 1;

    for (line_num, line) in content.lines().enumerate() {
        for pattern in SSRF_PATTERNS.iter() {
            if pattern.is_match(line) {
                let vuln = Vulnerability::new(
                    format!("SSRF-{:03}", id_counter),
                    VulnerabilityType::DataExfiltration, // Using closest existing type
                    Severity::High,
                    "SSRF Pattern Detected",
                    "Potential Server-Side Request Forgery detected",
                )
                .with_location(Location::new(file_path).with_line(line_num + 1))
                .with_impact("Attackers can make server requests to internal/external resources")
                .with_remediation("Validate URLs against allowlist, block internal IPs, use dedicated HTTP client with restrictions")
                .with_code_snippet(line.to_string())
                .with_confidence(0.70);

                vulnerabilities.push(vuln);
                id_counter += 1;
                break;
            }
        }
    }

    Ok(vulnerabilities)
}
