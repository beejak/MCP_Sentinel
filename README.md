# MCP Sentinel

🛡️ The Ultimate Security Scanner for MCP Servers

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

MCP Sentinel is a next-generation security scanner for Model Context Protocol (MCP) servers that combines static analysis, runtime monitoring, and AI-powered detection in a single, high-performance Rust binary.

## ⚡ Features

- **10 Detection Categories (v1.5.0 Complete)** 🚀:
  - 🔐 Secrets Detection (15+ patterns including AWS keys, API keys, private keys)
  - 💉 Command Injection (Python, JavaScript/TypeScript patterns)
  - 📁 Sensitive File Access (SSH keys, AWS credentials, browser cookies)
  - 🎣 Tool Poisoning (invisible Unicode, malicious keywords)
  - 🔓 Prompt Injection (jailbreak patterns, system prompt manipulation)
  - ⚡ Code Injection (20+ patterns: eval, exec, dynamic code execution) **NEW v1.5**
  - 📦 Insecure Deserialization (10+ patterns: pickle, yaml, marshal) **NEW v1.5**
  - 🔀 Path Traversal (8+ patterns: directory traversal attacks) **NEW v1.5**
  - 💾 SQL Injection (12+ patterns: string concatenation in queries) **NEW v1.5**
  - 🌐 SSRF (10+ patterns: server-side request forgery) **NEW v1.5**

- **Beautiful Terminal Output**:
  - Colored, hierarchical vulnerability display
  - Risk scoring (0-100)
  - Detailed remediation guidance
  - Code snippets with location info

- **Multiple Output Formats**:
  - Terminal (with colors)
  - JSON (for CI/CD integration)
  - HTML, PDF, SARIF (coming in Phase 2-4)

- **High Performance**:
  - Written in Rust for blazing speed
  - Concurrent file scanning
  - Target: <2s for small MCP servers

## 🚀 Quick Start

### Installation

```bash
# Using Cargo (when published)
cargo install mcp-sentinel

# Or build from source
git clone https://github.com/beejak/MCP_Sentinel.git
cd MCP_Sentinel
cargo build --release
```

### Basic Usage

```bash
# Scan a local MCP server directory
mcp-sentinel scan ./my-mcp-server

# Scan with JSON output
mcp-sentinel scan ./my-mcp-server --output json

# Fail CI/CD if high-severity issues found
mcp-sentinel scan ./my-mcp-server --fail-on high
```

## 📊 Implementation Status

### ✅ v1.5.0 Complete (Current Release)

- [x] **10 Security Detectors** with 80+ detection patterns
- [x] CLI framework (7 commands: scan, proxy, monitor, audit, init, whitelist, rules)
- [x] Core data models (Vulnerability, ScanResult, Config)
- [x] File discovery and traversal utilities
- [x] Terminal output renderer with colors
- [x] JSON output generator
- [x] Scanner engine with graceful error handling
- [x] Comprehensive documentation (16,000+ words)
- [x] Test fixtures with vulnerable code samples

### 🔄 Next: v1.6.0

- [ ] Whitelist/allowlist system implementation
- [ ] Async job-based scanning
- [ ] CSV and HTML export formats
- [ ] 80+ unit tests
- [ ] Integration test suite
- [ ] Performance benchmarks

### 🚀 Future: v2.0.0

- [ ] Tree-sitter AST parsing
- [ ] Semgrep integration
- [ ] AI analysis engine (OpenAI, Anthropic, Ollama)
- [ ] Runtime proxy engine
- [ ] Web dashboard
- [ ] SARIF output format

## 🛠️ Architecture

MCP Sentinel uses a modular, pipeline-based architecture:

```
mcp-sentinel/
├── src/
│   ├── cli/           # Command implementations
│   ├── detectors/     # Vulnerability detectors
│   ├── engines/       # Scanning engines
│   ├── models/        # Data models
│   ├── output/        # Report formatters
│   ├── storage/       # State management
│   ├── utils/         # Utilities
│   └── scanner.rs     # Main scanner API
├── tests/
│   └── fixtures/      # Test vulnerable servers
└── Cargo.toml
```

For detailed architecture diagrams and component descriptions, see [ARCHITECTURE.md](ARCHITECTURE.md).

## 🎯 Detection Capabilities

### Secrets Detection (15+ Patterns)
- AWS Access Keys (AKIA*, ASIA*)
- OpenAI API Keys
- Anthropic API Keys
- JWT Tokens
- Private Keys (RSA, EC, OpenSSH)
- Database Connection Strings
- GitHub Tokens
- Slack Tokens
- Google API Keys
- Hardcoded Passwords

### Command Injection
- Python: `os.system()`, `subprocess` with `shell=True`, `eval()`, `exec()`
- JavaScript: `child_process.exec()`, `eval()`, `Function()` constructor

### Sensitive File Access
- SSH keys (id_rsa, id_ed25519)
- AWS credentials (~/.aws/credentials)
- GCP credentials (~/.config/gcloud/)
- Environment files (.env)
- Browser cookies
- Shell RC files

### Tool Poisoning
- Invisible Unicode characters
- Keywords: "ignore", "disregard", "override", "actually"
- Hidden markers: [HIDDEN:], [SECRET:]

### Prompt Injection
- System prompt manipulation
- Role confusion
- Jailbreak attempts

## 📝 Example Output

```
🛡️  MCP Sentinel v1.0.0

📂 Scanning: ./vulnerable-server
🔍 Engines: Static Analysis ✓

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 SCAN RESULTS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Risk Score: 85/100 🔴 CRITICAL

🔴 CRITICAL Issues: 4
🟠 HIGH Issues: 2
🟡 MEDIUM Issues: 1
🔵 LOW Issues: 0

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 CRITICAL ISSUES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[SEC-001] AWS Access Key ID Found
  Location: server.py:10

  AWS Access Key ID detected

  ⚠️  Impact: Exposed AWS Access Key ID can be used for unauthorized access
  🔧 Remediation: Remove AWS Access Key ID from source code and use environment variables

⏱️  Scan completed in 1.2s
```

## 🧪 Testing

Test fixtures are available in `tests/fixtures/vulnerable_servers/`:

```bash
# Test the scanner on vulnerable fixtures
mcp-sentinel scan tests/fixtures/vulnerable_servers/test-server/
```

## 📖 Documentation

- [Architecture Guide](ARCHITECTURE.md) - System design and component details
- [Implementation Status](IMPLEMENTATION.md) - Detailed feature tracking
- [Error Handling Strategy](ERROR_HANDLING.md) - Error handling approach
- [Logging Guide](LOGGING.md) - Logging levels and best practices
- [Changelog](CHANGELOG.md) - Version history and release notes
- [Contributing Guidelines](CONTRIBUTING.md) - How to contribute
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community standards
- [Security Policy](SECURITY.md) - Reporting vulnerabilities

## 🤝 Contributing

MCP Sentinel welcomes contributions! Please read our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) before submitting PRs.

**Ways to contribute**:
- Report bugs or suggest features via GitHub Issues
- Improve documentation
- Add new detection patterns
- Optimize performance
- Write tests

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📄 License

Apache 2.0 - See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

Built with reference to the excellent work by:
- Invariant Labs (mcp-scan)
- Google (mcp-security)
- Antgroup (MCPScan)
- Rise and Ignite (mcp-shield)

---

**Status**: Phase 1 Complete ✅ | Next: Phase 2 (AI Analysis & Advanced Detection)
