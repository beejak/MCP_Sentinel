# Contributing to MCP Sentinel

Thank you for your interest in contributing to MCP Sentinel! This document provides guidelines and instructions for contributing.

## 🎯 Project Vision

MCP Sentinel aims to be the most comprehensive, performant, and user-friendly security scanner for Model Context Protocol servers. We welcome contributions that align with this vision.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)

## 📜 Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Familiarity with MCP (Model Context Protocol)
- Understanding of security scanning concepts

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/MCP_Scanner.git
   cd MCP_Scanner
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Run the scanner**
   ```bash
   cargo run -- scan tests/fixtures/vulnerable_servers/test-server/
   ```

## 🛠️ How to Contribute

### Reporting Bugs

**Before submitting a bug report:**
- Check existing issues to avoid duplicates
- Collect information (OS, Rust version, error messages)
- Create a minimal reproducible example

**When submitting:**
- Use the bug report template
- Include steps to reproduce
- Add relevant logs (use `--verbose`)
- Attach sample files if applicable (ensure no sensitive data)

### Suggesting Features

**Feature requests should include:**
- Clear use case
- Expected behavior
- Why it aligns with project goals
- Proposed implementation (optional)

Use the feature request template when creating an issue.

### Contributing Code

**Areas we need help with:**

1. **New Detectors** (Phase 2+)
   - PII detection patterns
   - Toxic flow analysis
   - Behavioral anomalies
   - Supply chain checks

2. **Language Support**
   - Go support
   - Ruby support
   - Additional tree-sitter parsers

3. **Output Formats** (Phase 4)
   - PDF report generation
   - SARIF format improvements
   - Custom report templates

4. **Performance Optimizations**
   - Parallel scanning improvements
   - Memory usage reduction
   - Caching strategies

5. **Documentation**
   - User guides
   - API documentation
   - Video tutorials
   - Translation to other languages

## 💻 Coding Standards

### Rust Style

Follow the official [Rust style guide](https://doc.rust-lang.org/1.0.0/style/):

```rust
// ✅ Good
pub fn detect_secrets(content: &str, file_path: &str) -> Result<Vec<Vulnerability>> {
    let mut vulnerabilities = Vec::new();
    // Implementation
    Ok(vulnerabilities)
}

// ❌ Bad
pub fn detectSecrets(content:&str,file_path:&str)->Result<Vec<Vulnerability>>{
    let mut vulnerabilities=Vec::new();
    // Implementation
    Ok(vulnerabilities)
}
```

### Code Organization

- **Modules**: One file per major component
- **Functions**: Keep under 50 lines when possible
- **Documentation**: Doc comments for all public items
- **Tests**: Unit tests in same file, integration tests in `tests/`

### Error Handling

```rust
// ✅ Use Result for fallible operations
pub fn scan_file(path: &Path) -> Result<Vec<Vulnerability>> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read {}", path.display()))?;
    // ...
}

// ✅ Log errors with appropriate level
match detector.detect(content) {
    Ok(vulns) => vulnerabilities.extend(vulns),
    Err(e) => warn!("Detector failed on {}: {}", path, e),
}

// ❌ Never use unwrap() in runtime code
let content = std::fs::read_to_string(path).unwrap();  // DON'T DO THIS
```

### Logging

```rust
use tracing::{debug, error, info, warn};

// ERROR: Critical failures
error!("Failed to scan directory: {}", e);

// WARN: Non-critical issues
warn!("Detector timed out on file: {}", path);

// INFO: Progress updates
info!("Found {} files to scan", count);

// DEBUG: Detailed information
debug!("Running detector on {}", file);
```

### Documentation

```rust
/// Detect secrets in source code
///
/// # Arguments
///
/// * `content` - The file content to scan
/// * `file_path` - Path to the file being scanned
///
/// # Returns
///
/// A vector of vulnerabilities found, or an error if scanning fails
///
/// # Examples
///
/// ```
/// let content = "API_KEY = 'secret123'";
/// let vulns = detect_secrets(content, "config.py")?;
/// assert!(!vulns.is_empty());
/// ```
pub fn detect_secrets(content: &str, file_path: &str) -> Result<Vec<Vulnerability>> {
    // Implementation
}
```

## 🧪 Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_aws_keys() {
        let content = r#"AWS_KEY = "AKIAIOSFODNN7EXAMPLE""#;
        let vulns = detect_secrets(content, "test.py").unwrap();
        assert_eq!(vulns.len(), 1);
        assert_eq!(vulns[0].vuln_type, VulnerabilityType::SecretsLeakage);
    }

    #[test]
    fn test_no_false_positives() {
        let content = r#"# Example: AWS_KEY = "AKIA...""#;
        let vulns = detect_secrets(content, "docs.md").unwrap();
        // Should detect even in comments (secrets are secrets)
    }
}
```

### Integration Tests

```rust
// tests/integration/scan_test.rs
#[tokio::test]
async fn test_scan_vulnerable_server() {
    let scanner = Scanner::new(ScanConfig::default());
    let result = scanner
        .scan_directory("tests/fixtures/vulnerable_servers/test-server")
        .await
        .unwrap();

    assert!(result.summary.critical > 0);
    assert!(result.summary.total_issues >= 5);
}
```

### Test Coverage

- Aim for >80% code coverage
- All detectors must have unit tests
- Add integration tests for new features
- Include both positive and negative test cases

## 📝 Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding tests
- `chore`: Maintenance tasks

### Examples

```bash
feat(detectors): add PII detection for phone numbers

Implements detection for US and international phone number formats
using regex patterns. Includes tests for common formats.

Closes #123

---

fix(scanner): handle binary files gracefully

Previously crashed on binary files. Now skips with debug log.

Fixes #456

---

docs(readme): update installation instructions

Add Homebrew installation method and clarify build requirements.
```

## 🔄 Pull Request Process

### Before Submitting

1. **Update your branch**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run all tests**
   ```bash
   cargo test --all
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

3. **Update documentation**
   - Add/update doc comments
   - Update README if needed
   - Add to CHANGELOG.md

4. **Add tests**
   - New features require tests
   - Fixes should include regression tests

### Submitting

1. **Create a focused PR**
   - One feature/fix per PR
   - Keep changes minimal
   - Link related issues

2. **Write a clear description**
   - What does this change?
   - Why is it needed?
   - How was it tested?

3. **Use the PR template**
   - Fill out all sections
   - Add screenshots for UI changes
   - List breaking changes

### Review Process

1. **Automated checks must pass**
   - Tests
   - Linting
   - Formatting
   - Security scan

2. **Code review**
   - At least one approval required
   - Address all feedback
   - Be respectful and collaborative

3. **Merge**
   - Squash commits unless logical separation needed
   - Use PR title as commit message
   - Delete branch after merge

## 🏗️ Project Structure

```
MCP_Scanner/
├── src/
│   ├── cli/           # Command implementations
│   ├── detectors/     # Vulnerability detectors
│   │   ├── secrets.rs
│   │   ├── code_vulns.rs
│   │   └── ...
│   ├── engines/       # Scanning engines
│   ├── models/        # Data models
│   ├── output/        # Report formatters
│   ├── storage/       # State management
│   ├── utils/         # Utilities
│   ├── scanner.rs     # Main scanner API
│   ├── lib.rs
│   └── main.rs
├── tests/
│   ├── integration/
│   ├── unit/
│   └── fixtures/
├── docs/
└── ...
```

## 🎓 Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Security Best Practices](./docs/security-best-practices.md)

## ❓ Questions?

- **General questions**: Open a Discussion
- **Bug reports**: Open an Issue
- **Security issues**: See [SECURITY.md](SECURITY.md)
- **Chat**: Join our Discord (coming soon)

## 🙏 Recognition

Contributors are recognized in:
- CHANGELOG.md
- README.md contributors section
- Release notes

Thank you for contributing to MCP Sentinel! 🛡️
