# Security Policy

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

We take security seriously. If you discover a security vulnerability in MCP Sentinel, please report it responsibly.

### How to Report

Send an email to: **security@mcp-sentinel.dev** (or create a private security advisory on GitHub)

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### What to Expect

1. **Acknowledgment**: Within 48 hours
2. **Assessment**: Within 7 days
3. **Fix & Release**: Depends on severity
   - Critical: 1-7 days
   - High: 7-14 days
   - Medium/Low: Next release cycle

### Disclosure Policy

- We will notify you when the vulnerability is fixed
- We will credit you in the security advisory (unless you prefer anonymity)
- We request a 90-day embargo before public disclosure

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Security Best Practices

### For Users

1. **Keep MCP Sentinel Updated**
   ```bash
   cargo install mcp-sentinel --force
   ```

2. **Verify Downloads**
   - Check SHA256 checksums
   - Verify GPG signatures (when available)

3. **Run with Least Privilege**
   - Don't run as root/administrator
   - Use appropriate file permissions

4. **Review Scan Results**
   - Act on critical/high findings immediately
   - Don't commit sensitive data revealed by scans

### For Contributors

1. **Never Commit Secrets**
   - Use `.gitignore` for sensitive files
   - Review diffs before committing
   - Use git-secrets or similar tools

2. **Dependency Management**
   - Keep dependencies updated
   - Review dependency changes
   - Use `cargo audit` regularly

3. **Code Review**
   - All code must be reviewed
   - Security-sensitive changes require additional review
   - Use static analysis tools

## Known Security Considerations

### 1. File System Access

MCP Sentinel reads files from the filesystem. Ensure you:
- Trust the directories you scan
- Don't scan untrusted symbolic links
- Be aware of file size limits

### 2. Regular Expressions

Some detectors use regex patterns that could be vulnerable to ReDoS (Regular Expression Denial of Service). We:
- Test patterns against ReDoS
- Use timeouts for pattern matching
- Accept PRs that improve pattern safety

### 3. External Dependencies

We minimize external dependencies and:
- Pin dependency versions in Cargo.lock
- Regularly audit with `cargo audit`
- Review security advisories

### 4. Secrets in Output

Detected secrets are redacted in output:
```rust
// Example: "AKIAIOSFODNN7EXAMPLE" → "AKIA...MPLE"
```

However, JSON output may contain more detail. Secure your reports appropriately.

## Security Features

### What MCP Sentinel Scans For

1. **Secrets**: API keys, private keys, credentials
2. **Command Injection**: Unsafe system calls
3. **File Access**: Sensitive file operations
4. **Tool Poisoning**: Malicious tool descriptions
5. **Prompt Injection**: LLM manipulation attempts

### What MCP Sentinel Does NOT Do

- **No Network Scanning**: We don't scan remote servers
- **No Code Execution**: We only analyze, never execute
- **No Data Collection**: No telemetry or analytics
- **No Cloud Storage**: Everything stays local

## Responsible Disclosure

We believe in responsible disclosure. If you discover a vulnerability:

1. **Contact us privately first**
2. **Give us time to fix it**
3. **Coordinate public disclosure**

We will:
- Acknowledge your contribution
- Keep you updated on progress
- Credit you in release notes (unless anonymous preferred)

## Security Hall of Fame

Contributors who responsibly disclose vulnerabilities:

*(None yet - be the first!)*

## Security Updates

Subscribe to security updates:
- Watch this repository
- Follow releases
- Join our security mailing list (coming soon)

## Audit History

| Date | Type | Auditor | Result |
|------|------|---------|--------|
| 2025-10-25 | Internal | Core Team | Phase 1 Complete - No issues found |

## Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Database](https://rustsec.org/)
- [CWE Database](https://cwe.mitre.org/)

## Contact

- **Security Issues**: security@mcp-sentinel.dev
- **General Questions**: discussions@mcp-sentinel.dev
- **Twitter**: @mcp_sentinel (coming soon)

---

**Thank you for helping keep MCP Sentinel secure!** 🛡️
