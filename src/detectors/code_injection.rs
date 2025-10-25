//! Code injection detection module
//!
//! This module detects dynamic code execution vulnerabilities where user input
//! is evaluated as code. This is one of the most dangerous vulnerability types
//! as it allows arbitrary code execution.
//!
//! # Detected Patterns
//!
//! - **Python**: `eval()`, `exec()`, `compile()`, `__import__()`
//! - **JavaScript**: `eval()`, `Function()` constructor, `vm.runInNewContext()`
//! - **Ruby**: `eval()`, `instance_eval()`, `class_eval()`
//!
//! # CWE Reference
//!
//! - CWE-94: Improper Control of Generation of Code ('Code Injection')
//! - CWE-95: Improper Neutralization of Directives in Dynamically Evaluated Code

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::models::vulnerability::{Location, Severity, Vulnerability, VulnerabilityType};

/// Code injection pattern definition
struct CodeInjectionPattern {
    name: &'static str,
    language: &'static str,
    regex: Regex,
    description: &'static str,
    severity: Severity,
}

/// All code injection patterns we scan for
static CODE_INJECTION_PATTERNS: Lazy<Vec<CodeInjectionPattern>> = Lazy::new(|| {
    vec![
        // Python - eval()
        CodeInjectionPattern {
            name: "Python eval() usage",
            language: "Python",
            regex: Regex::new(r#"\beval\s*\("#).unwrap(),
            description: "Dynamic code evaluation using eval() detected",
            severity: Severity::Critical,
        },
        // Python - exec()
        CodeInjectionPattern {
            name: "Python exec() usage",
            language: "Python",
            regex: Regex::new(r#"\bexec\s*\("#).unwrap(),
            description: "Dynamic code execution using exec() detected",
            severity: Severity::Critical,
        },
        // Python - compile()
        CodeInjectionPattern {
            name: "Python compile() usage",
            language: "Python",
            regex: Regex::new(r#"\bcompile\s*\("#).unwrap(),
            description: "Dynamic code compilation using compile() detected",
            severity: Severity::High,
        },
        // Python - __import__()
        CodeInjectionPattern {
            name: "Python __import__() usage",
            language: "Python",
            regex: Regex::new(r#"__import__\s*\("#).unwrap(),
            description: "Dynamic module import using __import__() detected",
            severity: Severity::High,
        },
        // Python - eval with getattr
        CodeInjectionPattern {
            name: "Python eval via getattr",
            language: "Python",
            regex: Regex::new(r#"getattr\s*\([^)]*,\s*['"]eval['"]\s*\)"#).unwrap(),
            description: "Obfuscated eval() usage via getattr detected",
            severity: Severity::Critical,
        },
        // JavaScript - eval()
        CodeInjectionPattern {
            name: "JavaScript eval() usage",
            language: "JavaScript/TypeScript",
            regex: Regex::new(r#"\beval\s*\("#).unwrap(),
            description: "Dynamic code evaluation using eval() detected",
            severity: Severity::Critical,
        },
        // JavaScript - Function constructor
        CodeInjectionPattern {
            name: "JavaScript Function() constructor",
            language: "JavaScript/TypeScript",
            regex: Regex::new(r#"\bnew\s+Function\s*\("#).unwrap(),
            description: "Dynamic function creation using Function() constructor detected",
            severity: Severity::Critical,
        },
        // JavaScript - Function constructor without new
        CodeInjectionPattern {
            name: "JavaScript Function() without new",
            language: "JavaScript/TypeScript",
            regex: Regex::new(r#"\bFunction\s*\([^)]*\)\s*\("#).unwrap(),
            description: "Dynamic function creation using Function() detected",
            severity: Severity::Critical,
        },
        // Node.js - vm.runInNewContext
        CodeInjectionPattern {
            name: "Node.js vm.runInNewContext",
            language: "JavaScript/TypeScript",
            regex: Regex::new(r#"vm\.runInNewContext\s*\("#).unwrap(),
            description: "Code execution in new context using vm.runInNewContext detected",
            severity: Severity::Critical,
        },
        // Node.js - vm.runInThisContext
        CodeInjectionPattern {
            name: "Node.js vm.runInThisContext",
            language: "JavaScript/TypeScript",
            regex: Regex::new(r#"vm\.runInThisContext\s*\("#).unwrap(),
            description: "Code execution in current context using vm.runInThisContext detected",
            severity: Severity::Critical,
        },
        // Node.js - vm.runInContext
        CodeInjectionPattern {
            name: "Node.js vm.runInContext",
            language: "JavaScript/TypeScript",
            regex: Regex::new(r#"vm\.runInContext\s*\("#).unwrap(),
            description: "Code execution using vm.runInContext detected",
            severity: Severity::Critical,
        },
        // Ruby - eval()
        CodeInjectionPattern {
            name: "Ruby eval() usage",
            language: "Ruby",
            regex: Regex::new(r#"\beval\s*\("#).unwrap(),
            description: "Dynamic code evaluation using eval() detected",
            severity: Severity::Critical,
        },
        // Ruby - instance_eval
        CodeInjectionPattern {
            name: "Ruby instance_eval usage",
            language: "Ruby",
            regex: Regex::new(r#"\.instance_eval\s*\("#).unwrap(),
            description: "Dynamic code evaluation using instance_eval detected",
            severity: Severity::Critical,
        },
        // Ruby - class_eval
        CodeInjectionPattern {
            name: "Ruby class_eval usage",
            language: "Ruby",
            regex: Regex::new(r#"\.class_eval\s*\("#).unwrap(),
            description: "Dynamic code evaluation using class_eval detected",
            severity: Severity::Critical,
        },
        // Ruby - module_eval
        CodeInjectionPattern {
            name: "Ruby module_eval usage",
            language: "Ruby",
            regex: Regex::new(r#"\.module_eval\s*\("#).unwrap(),
            description: "Dynamic code evaluation using module_eval detected",
            severity: Severity::Critical,
        },
        // Python - execfile() (Python 2)
        CodeInjectionPattern {
            name: "Python execfile() usage",
            language: "Python",
            regex: Regex::new(r#"\bexecfile\s*\("#).unwrap(),
            description: "Dynamic file execution using execfile() detected (Python 2)",
            severity: Severity::Critical,
        },
        // Python - code.InteractiveInterpreter
        CodeInjectionPattern {
            name: "Python InteractiveInterpreter",
            language: "Python",
            regex: Regex::new(r#"code\.InteractiveInterpreter"#).unwrap(),
            description: "Interactive code interpreter usage detected",
            severity: Severity::High,
        },
        // PHP - eval()
        CodeInjectionPattern {
            name: "PHP eval() usage",
            language: "PHP",
            regex: Regex::new(r#"\beval\s*\("#).unwrap(),
            description: "Dynamic code evaluation using eval() detected",
            severity: Severity::Critical,
        },
        // PHP - assert() with string
        CodeInjectionPattern {
            name: "PHP assert() with code string",
            language: "PHP",
            regex: Regex::new(r#"\bassert\s*\(\s*['"]"#).unwrap(),
            description: "Code execution using assert() with string detected",
            severity: Severity::Critical,
        },
        // PHP - preg_replace with /e modifier
        CodeInjectionPattern {
            name: "PHP preg_replace /e modifier",
            language: "PHP",
            regex: Regex::new(r#"preg_replace\s*\([^)]*['"]/.*e.*['"]"#).unwrap(),
            description: "Code execution using preg_replace with /e modifier detected",
            severity: Severity::Critical,
        },
    ]
});

/// Detect code injection vulnerabilities
///
/// Scans the provided content for patterns that indicate dynamic code execution.
/// These patterns are extremely dangerous as they allow arbitrary code execution
/// if user input reaches them.
///
/// # Arguments
///
/// * `content` - The file content to scan
/// * `file_path` - Path to the file being scanned
///
/// # Returns
///
/// A vector of vulnerabilities (empty if no code injection patterns found)
///
/// # Example
///
/// ```rust
/// let content = r#"result = eval(user_input)"#;
/// let vulns = detect(content, "server.py")?;
/// assert!(!vulns.is_empty());
/// ```
pub fn detect(content: &str, file_path: &str) -> Result<Vec<Vulnerability>> {
    let mut vulnerabilities = Vec::new();
    let mut id_counter = 1;

    for (line_num, line) in content.lines().enumerate() {
        // Skip comments (basic heuristic)
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        for pattern in CODE_INJECTION_PATTERNS.iter() {
            if pattern.regex.is_match(line) {
                let column = line.find(pattern.regex.as_str()).unwrap_or(0) + 1;

                let vuln = Vulnerability::new(
                    format!("CODE-INJ-{:03}", id_counter),
                    VulnerabilityType::CodeInjection,
                    pattern.severity,
                    format!("{} Detected", pattern.name),
                    pattern.description.to_string(),
                )
                .with_location(
                    Location::new(file_path)
                        .with_line(line_num + 1)
                        .with_column(column),
                )
                .with_impact(
                    "Attackers can execute arbitrary code on the server, \
                     leading to complete system compromise, data theft, or \
                     service disruption."
                        .to_string(),
                )
                .with_remediation(format!(
                    "Never use {} with untrusted input. Instead:\n\
                     - Use safe alternatives (e.g., ast.literal_eval() for Python)\n\
                     - Implement input validation and sanitization\n\
                     - Use a whitelist of allowed operations\n\
                     - Consider sandboxed execution environments\n\
                     - Review security guidelines for {}",
                    pattern.name, pattern.language
                ))
                .with_code_snippet(line.to_string())
                .with_confidence(0.90);

                // Add evidence
                let mut evidence = HashMap::new();
                evidence.insert("language".to_string(), serde_json::json!(pattern.language));
                evidence.insert("pattern".to_string(), serde_json::json!(pattern.name));
                evidence.insert(
                    "cwe".to_string(),
                    serde_json::json!("CWE-94: Code Injection"),
                );
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
    fn test_detect_python_eval() {
        let content = r#"
            user_input = request.get('input')
            result = eval(user_input)
        "#;

        let vulns = detect(content, "test.py").unwrap();
        assert!(!vulns.is_empty());
        assert!(vulns.iter().any(|v| v.title.contains("eval")));
    }

    #[test]
    fn test_detect_python_exec() {
        let content = r#"exec(user_code)"#;

        let vulns = detect(content, "test.py").unwrap();
        assert!(!vulns.is_empty());
        assert!(vulns.iter().any(|v| v.title.contains("exec")));
    }

    #[test]
    fn test_detect_javascript_eval() {
        let content = r#"eval(userInput);"#;

        let vulns = detect(content, "test.js").unwrap();
        assert!(!vulns.is_empty());
    }

    #[test]
    fn test_detect_javascript_function_constructor() {
        let content = r#"const fn = new Function(userCode);"#;

        let vulns = detect(content, "test.js").unwrap();
        assert!(!vulns.is_empty());
        assert!(vulns.iter().any(|v| v.title.contains("Function")));
    }

    #[test]
    fn test_detect_nodejs_vm() {
        let content = r#"vm.runInNewContext(code, sandbox);"#;

        let vulns = detect(content, "test.js").unwrap();
        assert!(!vulns.is_empty());
    }

    #[test]
    fn test_skip_comments() {
        let content = r#"
            # This is a comment: eval(bad_code)
            // This is also a comment: eval(bad_code)
        "#;

        let vulns = detect(content, "test.py").unwrap();
        assert!(vulns.is_empty());
    }

    #[test]
    fn test_python_compile() {
        let content = r#"compiled = compile(user_code, '<string>', 'exec')"#;

        let vulns = detect(content, "test.py").unwrap();
        assert!(!vulns.is_empty());
        assert!(vulns.iter().any(|v| v.title.contains("compile")));
    }

    #[test]
    fn test_ruby_eval() {
        let content = r#"eval(user_input)"#;

        let vulns = detect(content, "test.rb").unwrap();
        assert!(!vulns.is_empty());
    }

    #[test]
    fn test_no_false_positives_safe_code() {
        let content = r#"
            # Safe code - no dynamic evaluation
            result = calculate(user_input)
            data = json.loads(user_input)
        "#;

        let vulns = detect(content, "test.py").unwrap();
        assert!(vulns.is_empty());
    }
}
