# Getting Started with MCP Sentinel

Welcome to MCP Sentinel - the industry-leading security scanner for Model Context Protocol (MCP) servers. This guide will take you from installation to your first security scan in just a few minutes.

## Table of Contents

1. [What is MCP Sentinel?](#what-is-mcp-sentinel)
2. [System Requirements](#system-requirements)
3. [Installation](#installation)
4. [Your First Scan](#your-first-scan)
5. [Understanding Results](#understanding-results)
6. [Next Steps](#next-steps)

---

## What is MCP Sentinel?

MCP Sentinel is a production-ready security scanner designed specifically for Model Context Protocol servers. It detects:

- 🔑 **Hardcoded secrets** (API keys, passwords, tokens)
- 💉 **Injection vulnerabilities** (Command, Code, SQL, Prompt)
- 🔓 **Sensitive file access** (SSH keys, credentials, private keys)
- 🎣 **Tool poisoning** (Malicious tool descriptions, Unicode attacks)
- 🚀 **SSRF vulnerabilities** (Server-Side Request Forgery)
- 📂 **Path traversal** (Directory traversal attacks)
- 🧨 **Insecure deserialization** (Unsafe object loading)

**Key Advantages:**
- ⚡ **Fast**: 10-100x faster than Python-based scanners (written in Rust)
- 📦 **Zero dependencies**: Single binary, no runtime requirements
- 🔒 **Memory safe**: Built with Rust for security by design
- 📊 **Comprehensive**: 80+ detection patterns across 10 detectors
- 🛠️ **Production-ready**: Used in CI/CD pipelines, pre-commit hooks, and security audits

---

## System Requirements

### Minimum Requirements

- **Operating System**: Linux, macOS, or Windows
- **Architecture**: x86_64 (AMD64) or ARM64
- **Memory**: 50MB RAM
- **Disk Space**: 10MB for binary + space for scan targets

### Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|---------|
| Linux | x86_64 | ✅ Supported |
| Linux | ARM64 | ✅ Supported |
| macOS | x86_64 (Intel) | ✅ Supported |
| macOS | ARM64 (Apple Silicon) | ✅ Supported |
| Windows | x86_64 | ✅ Supported |
| Windows | ARM64 | ⚠️ Experimental |

---

## Installation

### Option 1: Build from Source (Recommended)

This is the most reliable method and ensures you get the latest version optimized for your system.

#### Prerequisites

First, install Rust if you haven't already:

```bash
# Install Rust using rustup (official Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions, then:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Build MCP Sentinel

```bash
# 1. Clone the repository
git clone https://github.com/beejak/MCP_Sentinel.git
cd MCP_Sentinel

# 2. Build the release binary (optimized)
cargo build --release

# 3. Verify the build
./target/release/mcp-sentinel --version
```

**Expected output:**
```
mcp-sentinel 1.5.0
```

#### Add to PATH (Optional but Recommended)

**On Linux/macOS:**
```bash
# Copy to a directory in your PATH
sudo cp target/release/mcp-sentinel /usr/local/bin/

# Or add to your PATH
echo 'export PATH="$PATH:'$(pwd)'/target/release"' >> ~/.bashrc
source ~/.bashrc

# Verify
mcp-sentinel --version
```

**On Windows (PowerShell):**
```powershell
# Copy to a directory in your PATH
Copy-Item target\release\mcp-sentinel.exe C:\Windows\System32\

# Or add to PATH permanently
$env:Path += ";$(Get-Location)\target\release"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::User)

# Verify
mcp-sentinel --version
```

---

### Option 2: Download Pre-built Binary

> **Note**: Pre-built binaries will be available in GitHub Releases after v1.5.0 is officially released.

```bash
# Example for Linux x86_64
wget https://github.com/beejak/MCP_Sentinel/releases/download/v1.5.0/mcp-sentinel-linux-x86_64
chmod +x mcp-sentinel-linux-x86_64
sudo mv mcp-sentinel-linux-x86_64 /usr/local/bin/mcp-sentinel

# Verify
mcp-sentinel --version
```

---

### Option 3: Install via Cargo

Once published to crates.io:

```bash
cargo install mcp-sentinel
mcp-sentinel --version
```

---

## Your First Scan

Now that MCP Sentinel is installed, let's run your first security scan!

### Basic Scan

Scan a single directory or file:

```bash
# Scan the current directory
mcp-sentinel scan .

# Scan a specific directory
mcp-sentinel scan /path/to/your/mcp-server

# Scan a specific file
mcp-sentinel scan ./server.py
```

### Example: Scanning a Sample MCP Server

Let's create a simple MCP server with some vulnerabilities and scan it:

```bash
# Create a test file
cat > test_server.py << 'EOF'
import os
import subprocess

# Hardcoded API key (vulnerability!)
API_KEY = "sk-1234567890abcdef"

def execute_command(cmd):
    # Command injection vulnerability!
    os.system(cmd)

def query_database(user_id):
    # SQL injection vulnerability!
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return query

def load_config(filename):
    # Path traversal vulnerability!
    with open(f"/config/{filename}") as f:
        return f.read()
EOF

# Scan the file
mcp-sentinel scan test_server.py
```

**Expected output:**

```
🔍 MCP Sentinel v1.5.0 - Security Scanner for MCP Servers
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📁 Scanning: test_server.py
⏱️  Started: 2025-10-25 10:45:23

🔍 Running 10 security detectors...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️  VULNERABILITIES FOUND

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 CRITICAL (1)

  [SECRETS-001] Hardcoded API key detected
  📄 File: test_server.py:5
  📝 Context: API_KEY = "sk-1234567890abcdef"
  💡 Recommendation: Store API keys in environment variables or secure vaults

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🟠 HIGH (2)

  [COMMAND-001] Command injection via os.system()
  📄 File: test_server.py:8
  📝 Context: os.system(cmd)
  💡 Recommendation: Use subprocess.run() with shell=False and argument list

  [SQL-001] SQL injection via string formatting
  📄 File: test_server.py:12
  📝 Context: query = f"SELECT * FROM users WHERE id = {user_id}"
  💡 Recommendation: Use parameterized queries with placeholders

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🟡 MEDIUM (1)

  [PATH-001] Path traversal vulnerability
  📄 File: test_server.py:15
  📝 Context: with open(f"/config/{filename}") as f:
  💡 Recommendation: Validate and sanitize file paths, use os.path.abspath()

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 SUMMARY

  Total files scanned: 1
  Total vulnerabilities: 4
    🔴 Critical: 1
    🟠 High: 2
    🟡 Medium: 1
    🟢 Low: 0
    ℹ️  Info: 0

⏱️  Scan completed in 0.03s
```

---

## Understanding Results

MCP Sentinel provides clear, actionable security findings. Let's break down what each part means:

### Severity Levels

MCP Sentinel classifies vulnerabilities into five severity levels:

| Level | Icon | Description | Action Required |
|-------|------|-------------|-----------------|
| **Critical** | 🔴 | Immediate security threat requiring urgent action | **Fix immediately** |
| **High** | 🟠 | Serious vulnerability that should be fixed soon | **Fix within 1 week** |
| **Medium** | 🟡 | Moderate risk that should be addressed | **Fix within 1 month** |
| **Low** | 🟢 | Minor issue or best practice violation | **Fix when convenient** |
| **Info** | ℹ️ | Informational finding for awareness | **Review and consider** |

### Vulnerability Report Structure

Each vulnerability finding includes:

```
[DETECTOR-ID] Brief description
📄 File: path/to/file.py:line_number
📝 Context: Actual code snippet showing the issue
💡 Recommendation: Specific guidance on how to fix
```

**Example breakdown:**

```
[SECRETS-001] Hardcoded API key detected
│
├─ Detector ID: SECRETS-001
│  └─ Unique identifier for this vulnerability type
│
├─ File location: test_server.py:5
│  └─ Line 5 in test_server.py
│
├─ Context: API_KEY = "sk-1234567890abcdef"
│  └─ The exact code that triggered the detection
│
└─ Recommendation: Store API keys in environment variables or secure vaults
   └─ Actionable advice on how to fix the issue
```

### Exit Codes

MCP Sentinel uses exit codes to indicate scan results (useful for CI/CD):

| Exit Code | Meaning | Description |
|-----------|---------|-------------|
| `0` | ✅ Success | No vulnerabilities found |
| `1` | ⚠️ Vulnerabilities | One or more vulnerabilities detected |
| `2` | ❌ Error | Scan failed due to an error |

**Check exit code in bash:**
```bash
mcp-sentinel scan .
echo "Exit code: $?"
```

**Use in CI/CD:**
```bash
# Fail the build if vulnerabilities are found
mcp-sentinel scan . || exit 1
```

### Output Formats

MCP Sentinel supports multiple output formats:

#### 1. **Human-readable (default)**
```bash
mcp-sentinel scan .
```
Beautiful, colored terminal output (shown above)

#### 2. **JSON format**
```bash
mcp-sentinel scan . --format json
```

**Output:**
```json
{
  "scan_id": "scan_20251025_104523",
  "timestamp": "2025-10-25T10:45:23Z",
  "scanner_version": "1.5.0",
  "target": "test_server.py",
  "vulnerabilities": [
    {
      "id": "SECRETS-001",
      "severity": "critical",
      "title": "Hardcoded API key detected",
      "file": "test_server.py",
      "line": 5,
      "context": "API_KEY = \"sk-1234567890abcdef\"",
      "recommendation": "Store API keys in environment variables or secure vaults"
    }
  ],
  "summary": {
    "total_files": 1,
    "total_vulnerabilities": 4,
    "critical": 1,
    "high": 2,
    "medium": 1,
    "low": 0,
    "info": 0
  },
  "duration_ms": 30
}
```

Perfect for automation, dashboards, and integrations!

#### 3. **SARIF format** (Static Analysis Results Interchange Format)
```bash
mcp-sentinel scan . --format sarif
```

Compatible with GitHub Code Scanning, Azure DevOps, and other security platforms.

---

## Next Steps

Congratulations! You've completed your first security scan with MCP Sentinel. Here's what to explore next:

### 1. **Learn All Features**
Read the [User Guide](USER_GUIDE.md) to explore all scanning options, filters, and advanced features.

### 2. **See Real-world Examples**
Check out [Examples](EXAMPLES.md) for practical use cases and scanning strategies.

### 3. **Integrate with CI/CD**
Learn how to add MCP Sentinel to your CI/CD pipeline: [CI/CD Integration Guide](CI_CD_INTEGRATION.md)

### 4. **Configure for Your Needs**
Customize MCP Sentinel with whitelists and rules: [Configuration Guide](CONFIGURATION.md)

### 5. **Master Advanced Features**
Become a power user: [Advanced Usage](ADVANCED_USAGE.md)

### 6. **Troubleshooting**
Having issues? Check the [Troubleshooting Guide](TROUBLESHOOTING.md)

### 7. **Best Practices**
Learn security scanning best practices: [Best Practices](BEST_PRACTICES.md)

### 8. **Get Help**
- 📖 [FAQ](FAQ.md) - Frequently asked questions
- 💬 [GitHub Issues](https://github.com/beejak/MCP_Sentinel/issues) - Report bugs or request features
- 📚 [Command Reference](COMMAND_REFERENCE.md) - Complete command documentation

---

## Quick Reference Card

```bash
# Basic scan
mcp-sentinel scan ./my-project

# Scan with specific severity filter
mcp-sentinel scan ./my-project --min-severity high

# Output as JSON
mcp-sentinel scan ./my-project --format json

# Scan specific file types only
mcp-sentinel scan ./my-project --include "*.py" --include "*.js"

# Exclude directories
mcp-sentinel scan ./my-project --exclude "node_modules/*"

# Get help
mcp-sentinel --help
mcp-sentinel scan --help
```

---

## Need Help?

- 📖 **Documentation**: Check the [docs](.) directory for comprehensive guides
- 🐛 **Report Issues**: [GitHub Issues](https://github.com/beejak/MCP_Sentinel/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/beejak/MCP_Sentinel/discussions)
- 📧 **Contact**: Reach out through GitHub

---

**Happy Scanning! 🛡️**

Stay secure with MCP Sentinel - the fastest, most comprehensive MCP security scanner available.
