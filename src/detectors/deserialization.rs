//! Insecure deserialization detection module
//!
//! Detects unsafe object deserialization that can lead to arbitrary code execution.
//! CWE-502: Deserialization of Untrusted Data

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::models::vulnerability::{Location, Severity, Vulnerability, VulnerabilityType};

struct DeserializationPattern {
    name: &'static str,
    language: &'static str,
    regex: Regex,
    description: &'static str,
    severity: Severity,
}

static DESERIALIZATION_PATTERNS: Lazy<Vec<DeserializationPattern>> = Lazy::new(|| {
    vec![
        // Python - pickle.loads
        DeserializationPattern {
            name: "Python pickle.loads()",
            language: "Python",
            regex: Regex::new(r#"pickle\.loads?\s*\("#).unwrap(),
            description: "Unsafe deserialization using pickle detected",
            severity: Severity::Critical,
        },
        // Python - yaml.load without SafeLoader
        DeserializationPattern {
            name: "Python yaml.load() without SafeLoader",
            language: "Python",
            regex: Regex::new(r#"yaml\.load\s*\([^,)]*\)"#).unwrap(),
            description: "Unsafe YAML deserialization without SafeLoader detected",
            severity: Severity::Critical,
        },
        // Python - marshal.loads
        DeserializationPattern {
            name: "Python marshal.loads()",
            language: "Python",
            regex: Regex::new(r#"marshal\.loads?\s*\("#).unwrap(),
            description: "Unsafe deserialization using marshal detected",
            severity: Severity::High,
        },
        // Python - shelve
        DeserializationPattern {
            name: "Python shelve usage",
            language: "Python",
            regex: Regex::new(r#"shelve\.open\s*\("#).unwrap(),
            description: "Shelve uses pickle internally, potential unsafe deserialization",
            severity: Severity::Medium,
        },
        // Java - ObjectInputStream.readObject
        DeserializationPattern {
            name: "Java ObjectInputStream.readObject()",
            language: "Java",
            regex: Regex::new(r#"ObjectInputStream.*\.readObject\s*\("#).unwrap(),
            description: "Unsafe Java object deserialization detected",
            severity: Severity::Critical,
        },
        // PHP - unserialize
        DeserializationPattern {
            name: "PHP unserialize()",
            language: "PHP",
            regex: Regex::new(r#"\bunserialize\s*\("#).unwrap(),
            description: "Unsafe PHP deserialization detected",
            severity: Severity::Critical,
        },
        // Ruby - Marshal.load
        DeserializationPattern {
            name: "Ruby Marshal.load()",
            language: "Ruby",
            regex: Regex::new(r#"Marshal\.load\s*\("#).unwrap(),
            description: "Unsafe Ruby deserialization using Marshal detected",
            severity: Severity::Critical,
        },
        // Node.js - node-serialize
        DeserializationPattern {
            name: "Node.js node-serialize",
            language: "JavaScript/TypeScript",
            regex: Regex::new(r#"serialize\.unserialize\s*\("#).unwrap(),
            description: "Unsafe deserialization using node-serialize detected",
            severity: Severity::Critical,
        },
    ]
});

pub fn detect(content: &str, file_path: &str) -> Result<Vec<Vulnerability>> {
    let mut vulnerabilities = Vec::new();
    let mut id_counter = 1;

    for (line_num, line) in content.lines().enumerate() {
        for pattern in DESERIALIZATION_PATTERNS.iter() {
            if pattern.regex.is_match(line) {
                let vuln = Vulnerability::new(
                    format!("DESER-{:03}", id_counter),
                    VulnerabilityType::UnsafeDeserialization,
                    pattern.severity,
                    format!("{} Detected", pattern.name),
                    pattern.description.to_string(),
                )
                .with_location(Location::new(file_path).with_line(line_num + 1))
                .with_impact(
                    "Attackers can craft malicious serialized objects that execute \
                     arbitrary code when deserialized, leading to full system compromise."
                        .to_string(),
                )
                .with_remediation(
                    format!("For {}: Use safe alternatives like JSON, or implement strict \
                             type checking and validation before deserialization. \
                             Consider using allowlists for allowed classes.",
                            pattern.language)
                )
                .with_code_snippet(line.to_string())
                .with_confidence(0.88);

                let mut evidence = HashMap::new();
                evidence.insert("language".to_string(), serde_json::json!(pattern.language));
                evidence.insert("cwe".to_string(), serde_json::json!("CWE-502"));
                let vuln = vuln.with_evidence(evidence);

                vulnerabilities.push(vuln);
                id_counter += 1;
            }
        }
    }

    Ok(vulnerabilities)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_pickle_loads() {
        let content = r#"data = pickle.loads(untrusted_data)"#;
        let vulns = detect(content, "test.py").unwrap();
        assert!(!vulns.is_empty());
    }

    #[test]
    fn test_detect_yaml_load() {
        let content = r#"config = yaml.load(user_input)"#;
        let vulns = detect(content, "test.py").unwrap();
        assert!(!vulns.is_empty());
    }
}
