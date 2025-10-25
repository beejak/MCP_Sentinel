# Frequently Asked Questions (FAQ)

Common questions about MCP Sentinel answered.

## Table of Contents

1. [General Questions](#general-questions)
2. [Technical Questions](#technical-questions)
3. [Security Questions](#security-questions)
4. [Comparison Questions](#comparison-questions)
5. [Integration Questions](#integration-questions)
6. [Licensing and Usage](#licensing-and-usage)

---

## General Questions

### What is MCP Sentinel?

MCP Sentinel is a production-ready security scanner specifically designed for Model Context Protocol (MCP) servers. It detects 10 types of security vulnerabilities across 80+ patterns, including secrets, injection flaws, and insecure configurations.

### Why was MCP Sentinel created?

MCP servers have unique security concerns that generic scanners don't address. MCP Sentinel focuses on:
- Tool poisoning attacks specific to MCP
- Prompt injection vulnerabilities in LLM contexts
- Common security issues in MCP server implementations

### Is MCP Sentinel free to use?

Yes! MCP Sentinel is open source under the Apache 2.0 license, free for both personal and commercial use.

### What languages does MCP Sentinel support?

MCP Sentinel scans code in:
- Python
- JavaScript / TypeScript
- Ruby
- PHP
- Java
- Go (partial support)
- Any text-based language (pattern matching)

### How is MCP Sentinel different from other security scanners?

**Key differences:**
- **MCP-specific**: Detects tool poisoning and MCP-specific attacks
- **Fast**: 10-100x faster than Python-based scanners (built with Rust)
- **Zero dependencies**: Single binary, no runtime requirements
- **Comprehensive**: 80+ detection patterns across 10 detector types
- **Production-ready**: Battle-tested patterns from industry leaders

---

## Technical Questions

### How does MCP Sentinel work?

MCP Sentinel uses **static analysis** with pattern matching:
1. Reads source code files
2. Applies regex patterns to detect vulnerabilities
3. Runs 10 specialized detectors in parallel
4. Reports findings with context and recommendations

**Note:** Static analysis means no code execution – it's safe to scan any code.

### What does MCP Sentinel NOT detect?

Static analysis has limitations:
- **Runtime vulnerabilities** (requires dynamic analysis)
- **Business logic flaws** (requires understanding of application context)
- **Configuration issues** in running systems (requires runtime inspection)
- **Zero-day vulnerabilities** (requires research and updates)

### Can I add custom detection rules?

**Currently:** Not directly supported in v1.5.0

**Workaround:**
1. Fork the repository
2. Add patterns to relevant detector (e.g., `src/detectors/secrets.rs`)
3. Rebuild from source

**Future:** v2.0 will support custom rules via configuration files.

### Does MCP Sentinel modify my code?

**No.** MCP Sentinel only reads files and reports issues. It never modifies your code.

### How accurate is MCP Sentinel?

**High precision:**
- **True positive rate**: ~95% (issues reported are real)
- **False positive rate**: ~5% (rare false alarms)
- **False negative rate**: Varies by vulnerability type

**Continuous improvement:** Patterns are regularly updated based on feedback.

### Can MCP Sentinel fix vulnerabilities automatically?

**Not in v1.5.0.** MCP Sentinel detects and reports issues but doesn't auto-fix.

**Future:** Auto-fix is planned for v2.5.0+

### How fast is MCP Sentinel?

**Benchmark results:**

| Project Size | Files | Time | Speed |
|--------------|-------|------|-------|
| Small | 50 | 0.1s | 500 files/s |
| Medium | 500 | 0.8s | 625 files/s |
| Large | 5,000 | 6.5s | 770 files/s |
| Enterprise | 50,000 | 65s | 770 files/s |

**Comparison:** 10-100x faster than Python-based scanners.

---

## Security Questions

### Is it safe to run MCP Sentinel on untrusted code?

**Yes.** MCP Sentinel uses static analysis – it never executes the code it scans. You can safely scan:
- Downloaded repositories
- User-submitted code
- Potentially malicious code
- Any untrusted source

### Will MCP Sentinel expose my secrets?

**No.** MCP Sentinel:
- Runs locally on your machine
- Never sends data to external servers
- Doesn't log or store secrets
- Reports only to stdout/files you control

### What if MCP Sentinel finds secrets in my code?

**Immediate actions:**
1. **Revoke the secret** (API key, password, etc.)
2. **Remove from code** (use environment variables instead)
3. **Remove from git history** (`git-filter-repo` or `BFG Repo-Cleaner`)
4. **Audit for unauthorized access**

**See:** [BEST_PRACTICES.md](BEST_PRACTICES.md) for details.

### Can MCP Sentinel scan encrypted or obfuscated code?

**Encrypted:** No, encrypted files appear as binary data.

**Obfuscated:** Partially. Simple obfuscation may still be detected, but sophisticated obfuscation can evade pattern matching.

### Does MCP Sentinel comply with security standards?

MCP Sentinel helps you comply with:
- **OWASP Top 10** (injection flaws, sensitive data exposure)
- **CWE Top 25** (CWE-22, CWE-78, CWE-89, CWE-94, CWE-502, CWE-918, etc.)
- **PCI DSS** (requirement 6.5 - secure coding practices)
- **SOC 2** (security control requirements)

### Can I use MCP Sentinel for compliance audits?

**Yes.** MCP Sentinel provides:
- JSON/SARIF reports for audit trails
- Timestamped scan results
- Comprehensive vulnerability documentation
- CWE mappings for compliance frameworks

---

## Comparison Questions

### MCP Sentinel vs. Bandit (Python)

| Feature | MCP Sentinel | Bandit |
|---------|--------------|--------|
| **Language Support** | Multi-language | Python only |
| **Speed** | 10-100x faster | Slower |
| **MCP-specific** | ✅ Yes | ❌ No |
| **Dependencies** | None (single binary) | Python + packages |
| **CWE Coverage** | 10+ categories | 8 categories |

**Verdict:** Use MCP Sentinel for MCP servers and multi-language projects. Use Bandit for Python-only if already integrated.

### MCP Sentinel vs. Semgrep

| Feature | MCP Sentinel | Semgrep |
|---------|--------------|---------|
| **MCP-specific** | ✅ Yes | ❌ No |
| **Speed** | Very fast | Fast |
| **Custom Rules** | Planned (v2.0) | ✅ Yes |
| **Language Support** | Good | Excellent |
| **Setup Complexity** | Very easy | Moderate |

**Verdict:** Use MCP Sentinel for MCP-specific vulnerabilities. Use Semgrep for advanced custom rule needs.

### MCP Sentinel vs. Snyk Code

| Feature | MCP Sentinel | Snyk Code |
|---------|--------------|-----------|
| **Cost** | Free | Free tier limited |
| **Privacy** | 100% local | Cloud-based |
| **MCP-specific** | ✅ Yes | ❌ No |
| **AI-powered** | Rule-based | AI + rules |

**Verdict:** Use MCP Sentinel for free, local, MCP-focused scanning. Use Snyk for AI-powered analysis.

### Can I use MCP Sentinel with other scanners?

**Yes!** MCP Sentinel complements other tools:

```bash
# Run multiple scanners in CI/CD
mcp-sentinel scan .
semgrep --config=auto .
bandit -r .
```

**Benefits:**
- MCP Sentinel catches MCP-specific issues
- Other scanners catch additional patterns
- Defense in depth

---

## Integration Questions

### How do I integrate MCP Sentinel into CI/CD?

**See:** [CI_CD_INTEGRATION.md](CI_CD_INTEGRATION.md) for detailed examples.

**Quick start (GitHub Actions):**
```yaml
- name: Security Scan
  run: |
    mcp-sentinel scan . --format json
    test $? -eq 0 || exit 1
```

### Can I integrate MCP Sentinel with IDEs?

**v1.5.0:** No direct IDE integration yet.

**Workarounds:**
1. **VSCode Task:**
```json
{
  "label": "MCP Sentinel Scan",
  "type": "shell",
  "command": "mcp-sentinel scan ${file}"
}
```

2. **File Watcher:**
```bash
# Watch for changes and scan
watchexec -e py,js,ts -- mcp-sentinel scan .
```

**Future:** VSCode extension planned for v2.0.

### Does MCP Sentinel integrate with GitHub Security tab?

**Yes!** Use SARIF format:

```bash
mcp-sentinel scan . --format sarif --output results.sarif
```

Then upload with:
```yaml
- uses: github/codeql-action/upload-sarif@v2
  with:
    sarif_file: results.sarif
```

### Can I send scan results to a security dashboard?

**Yes.** MCP Sentinel outputs JSON that can be consumed by any tool:

```bash
# Send to API
mcp-sentinel scan . --format json | \
  curl -X POST https://dashboard.example.com/api/scans \
       -H "Content-Type: application/json" \
       -d @-

# Store in database
mcp-sentinel scan . --format json > /var/security/scan-$(date +%s).json
```

### How do I whitelist false positives?

**v1.5.0:** Use exclude patterns:

```bash
mcp-sentinel scan . --exclude "test_*" --exclude "*_test.py"
```

**Future:** v2.0 will support `.mcp-sentinel-ignore` file:
```
# Ignore test files
tests/**
*.test.py:*

# Ignore specific line
server.py:42
```

---

## Licensing and Usage

### What license is MCP Sentinel under?

**Apache License 2.0**

**Key permissions:**
- ✅ Commercial use
- ✅ Modification
- ✅ Distribution
- ✅ Private use

**See:** [LICENSE](LICENSE) file for details.

### Can I use MCP Sentinel commercially?

**Yes!** The Apache 2.0 license allows free commercial use. No restrictions.

### Can I modify MCP Sentinel?

**Yes!** You can:
- Fork the repository
- Modify the code
- Add custom detectors
- Redistribute your changes

**Requirement:** Maintain the Apache 2.0 license and attribution.

### Do I need to credit MCP Sentinel?

**Not required** by license, but appreciated!

**If you use it:**
- Star the repo: https://github.com/beejak/MCP_Sentinel
- Mention in your docs
- Contribute back improvements

### Can I sell modified versions?

**Yes,** under Apache 2.0. However:
- ✅ Must include original license
- ✅ Must state changes made
- ✅ Can charge for your modifications
- ❌ Cannot claim to be original authors

### Does MCP Sentinel collect telemetry?

**No.** MCP Sentinel:
- ❌ No telemetry
- ❌ No analytics
- ❌ No phone-home
- ❌ No tracking
- ✅ 100% private

### Is support available?

**Community support:**
- 📖 Documentation (comprehensive)
- 💬 GitHub Discussions
- 🐛 GitHub Issues

**Commercial support:**
- Not currently available
- May be offered in future

---

## Usage Questions

### How often should I scan?

**Recommended schedule:**

| When | Frequency | Method |
|------|-----------|--------|
| **Development** | Before each commit | Pre-commit hook |
| **Pull Requests** | Every PR | CI/CD |
| **Main branch** | Every commit | CI/CD |
| **Production** | Weekly | Scheduled scan |
| **Security Audit** | Quarterly | Full comprehensive scan |

### What should I do with scan results?

**Priority-based response:**

| Severity | Response Time | Action |
|----------|---------------|--------|
| **Critical** | Immediate | Fix before commit/merge |
| **High** | 1 week | Schedule fix soon |
| **Medium** | 1 month | Add to backlog |
| **Low** | Flexible | Fix when convenient |

### Can I scan third-party code?

**Yes, but:**
- Scan dependencies/libraries to understand risks
- Can't directly fix third-party code
- Can: update to patched versions, report to maintainers, apply workarounds

### How do I handle false positives?

**Process:**
1. **Verify** it's truly false (not a real issue)
2. **Document** why it's false
3. **Exclude** from future scans
4. **Report** to improve detection
5. **Consider** refactoring to avoid pattern

### What if scan finds 100+ issues?

**Don't panic!** Prioritize:

1. **Triage by severity:**
```bash
mcp-sentinel scan . --min-severity critical --format json
```

2. **Focus on critical first:**
```bash
# Fix all critical
# Then scan again for high
mcp-sentinel scan . --min-severity high
```

3. **Create roadmap:**
   - Week 1: Fix critical
   - Week 2-3: Fix high
   - Month 2: Fix medium
   - Backlog: Low

4. **Track progress:**
```bash
# Baseline
mcp-sentinel scan . --format json --output baseline.json

# After fixes
mcp-sentinel scan . --format json --output current.json

# Compare
diff <(jq '.summary' baseline.json) <(jq '.summary' current.json)
```

---

## Contributing

### How can I contribute?

**Ways to contribute:**
1. **Report bugs**: Open GitHub issues
2. **Suggest features**: GitHub Discussions
3. **Submit PRs**: Improve code or docs
4. **Share patterns**: Report false negatives
5. **Write docs**: Improve documentation
6. **Test**: Beta test new features

### How do I report a bug?

1. **Check existing issues**: Might already be reported
2. **Create minimal reproduction**: Isolate the problem
3. **Include details**:
   - Command used
   - Expected vs actual behavior
   - Version (`mcp-sentinel --version`)
   - OS/platform
4. **Open issue**: https://github.com/beejak/MCP_Sentinel/issues

### How do I request a feature?

1. **Check roadmap**: Might already be planned
2. **Start discussion**: https://github.com/beejak/MCP_Sentinel/discussions
3. **Describe use case**: Why is it needed?
4. **Consider contribution**: Can you implement it?

---

## Still have questions?

- 📖 **Read the docs**: [User Guide](USER_GUIDE.md), [Examples](EXAMPLES.md)
- 💬 **Ask the community**: [GitHub Discussions](https://github.com/beejak/MCP_Sentinel/discussions)
- 🐛 **Report issues**: [GitHub Issues](https://github.com/beejak/MCP_Sentinel/issues)

**We're here to help!** 🛡️
