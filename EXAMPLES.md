# MCP Sentinel Examples

Real-world examples and use cases for scanning MCP servers with MCP Sentinel.

## Table of Contents

1. [Quick Start Examples](#quick-start-examples)
2. [Language-Specific Examples](#language-specific-examples)
3. [Real-World Scenarios](#real-world-scenarios)
4. [CI/CD Examples](#cicd-examples)
5. [Advanced Scanning Patterns](#advanced-scanning-patterns)
6. [Before and After Fixes](#before-and-after-fixes)

---

## Quick Start Examples

### Example 1: Scanning a Python MCP Server

**Scenario:** You have a Python-based MCP server and want to check for security issues.

```bash
# Your project structure
my-mcp-server/
├── server.py
├── tools/
│   ├── file_ops.py
│   ├── database.py
│   └── api_client.py
└── config.py

# Scan the entire project
mcp-sentinel scan my-mcp-server/

# Scan only Python files
mcp-sentinel scan my-mcp-server/ --include "*.py"

# Scan excluding test files
mcp-sentinel scan my-mcp-server/ --exclude "*test*.py"
```

### Example 2: Scanning a Node.js MCP Server

```bash
# Your project
mcp-server-js/
├── index.js
├── tools/
│   ├── fileOps.js
│   ├── apiClient.js
│   └── database.js
└── package.json

# Scan JavaScript/TypeScript
mcp-sentinel scan mcp-server-js/ --include "*.js" --include "*.ts"

# Exclude node_modules
mcp-sentinel scan mcp-server-js/ --exclude "node_modules/*"
```

### Example 3: Quick Security Check

**Before committing code:**

```bash
# Scan only changed files
git diff --name-only | xargs mcp-sentinel scan --min-severity high

# If clean, commit
git commit -m "Add new feature"
```

---

## Language-Specific Examples

### Python MCP Server

**server.py - With Vulnerabilities:**

```python
import os
import subprocess
import pickle
import sqlite3

# Config with hardcoded secrets (❌ CRITICAL)
API_KEY = "sk-1234567890abcdefghijklmnop"
DATABASE_URL = "postgresql://admin:password123@localhost/mcpdb"

class MCPServer:
    def __init__(self):
        self.api_key = API_KEY

    def execute_tool(self, tool_name, command):
        """Execute tool command (❌ COMMAND INJECTION)"""
        os.system(f"{tool_name} {command}")

    def query_user(self, user_id):
        """Query database (❌ SQL INJECTION)"""
        conn = sqlite3.connect('mcp.db')
        cursor = conn.cursor()
        query = f"SELECT * FROM users WHERE id = {user_id}"
        cursor.execute(query)
        return cursor.fetchall()

    def load_tool_config(self, config_data):
        """Load tool configuration (❌ INSECURE DESERIALIZATION)"""
        return pickle.loads(config_data)

    def read_file(self, filename):
        """Read file (❌ PATH TRAVERSAL)"""
        with open(f"/tools/data/{filename}") as f:
            return f.read()

    def fetch_url(self, url):
        """Fetch external URL (❌ SSRF)"""
        import requests
        return requests.get(url).text
```

**Scan it:**

```bash
mcp-sentinel scan server.py
```

**Expected findings:**

```
🔴 CRITICAL (2)
  [SECRETS-001] Hardcoded API key: line 6
  [SECRETS-002] Hardcoded database password: line 7

🟠 HIGH (5)
  [COMMAND-001] Command injection via os.system(): line 15
  [SQL-001] SQL injection vulnerability: line 21
  [DESERIAL-001] Insecure pickle deserialization: line 26
  [PATH-001] Path traversal vulnerability: line 30
  [SSRF-001] Server-side request forgery: line 35

Total: 7 vulnerabilities
```

**server.py - Fixed Version:**

```python
import os
import subprocess
import json
import sqlite3
from pathlib import Path

# Use environment variables (✅ SECURE)
API_KEY = os.getenv("MCP_API_KEY")
DATABASE_URL = os.getenv("DATABASE_URL")

class MCPServer:
    def __init__(self):
        if not API_KEY:
            raise ValueError("MCP_API_KEY environment variable required")
        self.api_key = API_KEY

    def execute_tool(self, tool_name, command):
        """Execute tool command safely (✅ SECURE)"""
        # Use subprocess with argument list
        allowed_tools = ['grep', 'find', 'cat']
        if tool_name not in allowed_tools:
            raise ValueError(f"Tool {tool_name} not allowed")
        subprocess.run([tool_name] + command.split(), check=True, shell=False)

    def query_user(self, user_id):
        """Query database safely (✅ SECURE)"""
        conn = sqlite3.connect('mcp.db')
        cursor = conn.cursor()
        # Use parameterized query
        query = "SELECT * FROM users WHERE id = ?"
        cursor.execute(query, (user_id,))
        return cursor.fetchall()

    def load_tool_config(self, config_data):
        """Load tool configuration safely (✅ SECURE)"""
        # Use JSON instead of pickle
        return json.loads(config_data)

    def read_file(self, filename):
        """Read file safely (✅ SECURE)"""
        # Validate path
        base_path = Path("/tools/data")
        file_path = (base_path / filename).resolve()

        if not str(file_path).startswith(str(base_path)):
            raise ValueError("Path traversal attempt detected")

        with open(file_path) as f:
            return f.read()

    def fetch_url(self, url):
        """Fetch external URL safely (✅ SECURE)"""
        import requests
        from urllib.parse import urlparse

        # Whitelist allowed domains
        allowed_domains = ['api.example.com', 'trusted.api.io']
        parsed = urlparse(url)

        if parsed.netloc not in allowed_domains:
            raise ValueError(f"Domain {parsed.netloc} not allowed")

        # Block internal IPs
        if parsed.netloc in ['localhost', '127.0.0.1', '0.0.0.0']:
            raise ValueError("Internal IP access blocked")

        return requests.get(url, timeout=5).text
```

**Rescan:**

```bash
mcp-sentinel scan server.py
```

**Result:**

```
✅ No vulnerabilities found!

Total files scanned: 1
Total vulnerabilities: 0
```

---

### JavaScript/TypeScript MCP Server

**server.js - With Vulnerabilities:**

```javascript
const express = require('express');
const { exec } = require('child_process');
const fs = require('fs');
const axios = require('axios');

// Hardcoded secrets (❌ CRITICAL)
const API_KEY = 'sk-1234567890abcdefghijklmnop';
const DB_PASSWORD = 'admin123';

class MCPServer {
  constructor() {
    this.apiKey = API_KEY;
  }

  // Command injection (❌ HIGH)
  async executeTool(toolName, command) {
    return new Promise((resolve, reject) => {
      exec(`${toolName} ${command}`, (error, stdout) => {
        if (error) reject(error);
        else resolve(stdout);
      });
    });
  }

  // SQL injection (❌ HIGH)
  async queryUser(userId) {
    const query = `SELECT * FROM users WHERE id = ${userId}`;
    return await this.db.query(query);
  }

  // Path traversal (❌ MEDIUM)
  readFile(filename) {
    return fs.readFileSync(`/data/${filename}`, 'utf8');
  }

  // SSRF (❌ HIGH)
  async fetchUrl(url) {
    const response = await axios.get(url);
    return response.data;
  }

  // Code injection (❌ CRITICAL)
  evaluateExpression(expr) {
    return eval(expr);
  }
}
```

**Scan it:**

```bash
mcp-sentinel scan server.js
```

**server.js - Fixed Version:**

```javascript
const express = require('express');
const { execFile } = require('child_process');
const fs = require('fs').promises;
const axios = require('axios');
const path = require('path');

// Use environment variables (✅ SECURE)
const API_KEY = process.env.MCP_API_KEY;
const DB_PASSWORD = process.env.DB_PASSWORD;

if (!API_KEY) {
  throw new Error('MCP_API_KEY environment variable required');
}

class MCPServer {
  constructor() {
    this.apiKey = API_KEY;
    this.allowedTools = ['grep', 'find', 'cat'];
    this.allowedDomains = ['api.example.com', 'trusted.api.io'];
  }

  // Safe command execution (✅ SECURE)
  async executeTool(toolName, args) {
    if (!this.allowedTools.includes(toolName)) {
      throw new Error(`Tool ${toolName} not allowed`);
    }

    return new Promise((resolve, reject) => {
      // Use execFile with array of arguments (no shell)
      execFile(toolName, args, (error, stdout) => {
        if (error) reject(error);
        else resolve(stdout);
      });
    });
  }

  // Safe SQL query (✅ SECURE)
  async queryUser(userId) {
    // Use parameterized query
    const query = 'SELECT * FROM users WHERE id = ?';
    return await this.db.query(query, [userId]);
  }

  // Safe file reading (✅ SECURE)
  async readFile(filename) {
    const basePath = path.resolve('/data');
    const filePath = path.resolve(basePath, filename);

    // Prevent path traversal
    if (!filePath.startsWith(basePath)) {
      throw new Error('Path traversal attempt detected');
    }

    return await fs.readFile(filePath, 'utf8');
  }

  // Safe URL fetching (✅ SECURE)
  async fetchUrl(url) {
    const urlObj = new URL(url);

    // Whitelist domains
    if (!this.allowedDomains.includes(urlObj.hostname)) {
      throw new Error(`Domain ${urlObj.hostname} not allowed`);
    }

    // Block internal IPs
    if (['localhost', '127.0.0.1', '0.0.0.0'].includes(urlObj.hostname)) {
      throw new Error('Internal IP access blocked');
    }

    const response = await axios.get(url, { timeout: 5000 });
    return response.data;
  }

  // Safe expression evaluation (✅ SECURE)
  evaluateExpression(expr) {
    // Use a safe math evaluator library instead of eval()
    const math = require('mathjs');
    return math.evaluate(expr);
  }
}
```

---

## Real-World Scenarios

### Scenario 1: Securing a File Operations MCP Tool

**Problem:** Your MCP server has a file operations tool that allows reading files.

**vulnerable_file_tool.py:**

```python
def read_file_tool(filepath: str) -> str:
    """MCP tool to read files"""
    # ❌ Vulnerable to path traversal
    with open(filepath) as f:
        return f.read()

# User could pass: "../../../../etc/passwd"
```

**Scan:**

```bash
mcp-sentinel scan vulnerable_file_tool.py
# Finds: [PATH-001] Path traversal vulnerability
```

**Fixed version:**

```python
from pathlib import Path

ALLOWED_BASE_PATH = Path("/workspace/allowed")

def read_file_tool(filepath: str) -> str:
    """MCP tool to read files safely"""
    # ✅ Validate path
    requested_path = (ALLOWED_BASE_PATH / filepath).resolve()

    # Ensure path is within allowed directory
    if not str(requested_path).startswith(str(ALLOWED_BASE_PATH)):
        raise ValueError("Access denied: path outside allowed directory")

    # Check file exists
    if not requested_path.exists():
        raise FileNotFoundError(f"File not found: {filepath}")

    # Check it's a file, not a directory
    if not requested_path.is_file():
        raise ValueError(f"Not a file: {filepath}")

    with open(requested_path) as f:
        return f.read()
```

### Scenario 2: Securing Database Query Tool

**Problem:** MCP tool that queries database with user input.

**vulnerable_db_tool.py:**

```python
import sqlite3

def search_users_tool(search_term: str) -> list:
    """Search users in database"""
    conn = sqlite3.connect('users.db')
    cursor = conn.cursor()

    # ❌ SQL injection vulnerability
    query = f"SELECT * FROM users WHERE name LIKE '%{search_term}%'"
    cursor.execute(query)
    return cursor.fetchall()

# Attacker could pass: "' OR '1'='1' --"
```

**Scan:**

```bash
mcp-sentinel scan vulnerable_db_tool.py --only-detector sql-injection
# Finds: [SQL-001] SQL injection via string formatting
```

**Fixed version:**

```python
import sqlite3
from typing import List, Dict

def search_users_tool(search_term: str) -> List[Dict]:
    """Search users in database safely"""
    conn = sqlite3.connect('users.db')
    cursor = conn.cursor()

    # ✅ Use parameterized query
    query = "SELECT id, name, email FROM users WHERE name LIKE ?"
    cursor.execute(query, (f"%{search_term}%",))

    # Return as list of dicts
    columns = [desc[0] for desc in cursor.description]
    results = [dict(zip(columns, row)) for row in cursor.fetchall()]

    return results
```

### Scenario 3: Securing API Integration Tool

**Problem:** MCP tool that fetches data from external APIs.

**vulnerable_api_tool.py:**

```python
import requests

def fetch_api_data_tool(url: str) -> dict:
    """Fetch data from external API"""
    # ❌ SSRF vulnerability - can access internal services
    response = requests.get(url)
    return response.json()

# Attacker could pass: "http://localhost:8080/admin"
```

**Scan:**

```bash
mcp-sentinel scan vulnerable_api_tool.py --only-detector ssrf
# Finds: [SSRF-001] Server-side request forgery
```

**Fixed version:**

```python
import requests
from urllib.parse import urlparse
import ipaddress

ALLOWED_DOMAINS = [
    'api.github.com',
    'api.openai.com',
    'api.anthropic.com'
]

def is_internal_ip(hostname: str) -> bool:
    """Check if hostname resolves to internal IP"""
    try:
        ip = ipaddress.ip_address(hostname)
        return ip.is_private or ip.is_loopback
    except ValueError:
        # Not an IP address, check DNS resolution
        import socket
        try:
            ip = socket.gethostbyname(hostname)
            return is_internal_ip(ip)
        except socket.gaierror:
            return False

def fetch_api_data_tool(url: str) -> dict:
    """Fetch data from external API safely"""
    parsed = urlparse(url)

    # ✅ Validate scheme
    if parsed.scheme not in ['https']:
        raise ValueError("Only HTTPS URLs allowed")

    # ✅ Whitelist domains
    if parsed.netloc not in ALLOWED_DOMAINS:
        raise ValueError(f"Domain {parsed.netloc} not in whitelist")

    # ✅ Block internal IPs
    if is_internal_ip(parsed.netloc):
        raise ValueError("Internal IP access blocked")

    # ✅ Set timeout
    response = requests.get(url, timeout=5, allow_redirects=False)
    response.raise_for_status()

    return response.json()
```

---

## CI/CD Examples

### GitHub Actions

**.github/workflows/security-scan.yml:**

```yaml
name: Security Scan

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  security-scan:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build MCP Sentinel
        run: |
          git clone https://github.com/beejak/MCP_Sentinel.git
          cd MCP_Sentinel
          cargo build --release
          sudo cp target/release/mcp-sentinel /usr/local/bin/

      - name: Run Security Scan
        run: |
          mcp-sentinel scan . --format json --output results.json

      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: security-scan-results
          path: results.json

      - name: Check for Critical Issues
        run: |
          CRITICAL=$(jq '.summary.by_severity.critical' results.json)
          HIGH=$(jq '.summary.by_severity.high' results.json)

          if [ "$CRITICAL" -gt 0 ]; then
            echo "❌ Found $CRITICAL critical vulnerabilities!"
            jq '.vulnerabilities[] | select(.severity == "critical")' results.json
            exit 1
          fi

          if [ "$HIGH" -gt 5 ]; then
            echo "⚠️ Found $HIGH high severity vulnerabilities (threshold: 5)"
            exit 1
          fi

          echo "✅ Security scan passed!"
```

### GitLab CI

**.gitlab-ci.yml:**

```yaml
stages:
  - security

security_scan:
  stage: security
  image: rust:latest

  before_script:
    - git clone https://github.com/beejak/MCP_Sentinel.git
    - cd MCP_Sentinel && cargo build --release
    - cp target/release/mcp-sentinel /usr/local/bin/
    - cd ..

  script:
    - mcp-sentinel scan . --format json --output results.json
    - |
      CRITICAL=$(jq '.summary.by_severity.critical' results.json)
      if [ "$CRITICAL" -gt 0 ]; then
        echo "Critical vulnerabilities found!"
        exit 1
      fi

  artifacts:
    reports:
      json: results.json
    paths:
      - results.json
    expire_in: 30 days

  only:
    - merge_requests
    - main
```

### Jenkins Pipeline

**Jenkinsfile:**

```groovy
pipeline {
    agent any

    stages {
        stage('Security Scan') {
            steps {
                script {
                    // Install MCP Sentinel
                    sh '''
                        if [ ! -f /usr/local/bin/mcp-sentinel ]; then
                            git clone https://github.com/beejak/MCP_Sentinel.git /tmp/mcp-sentinel
                            cd /tmp/mcp-sentinel
                            cargo build --release
                            sudo cp target/release/mcp-sentinel /usr/local/bin/
                        fi
                    '''

                    // Run scan
                    sh 'mcp-sentinel scan . --format json --output results.json'

                    // Parse results
                    def results = readJSON file: 'results.json'
                    def critical = results.summary.by_severity.critical
                    def high = results.summary.by_severity.high

                    echo "Security Scan Results:"
                    echo "  Critical: ${critical}"
                    echo "  High: ${high}"

                    if (critical > 0) {
                        error("Critical vulnerabilities found!")
                    }
                }
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'results.json', fingerprint: true
        }
    }
}
```

---

## Advanced Scanning Patterns

### Pattern 1: Scan Only Changed Files in PR

```bash
#!/bin/bash
# scan-pr.sh - Scan only files changed in PR

BASE_BRANCH=${1:-main}

echo "Scanning files changed since $BASE_BRANCH..."

# Get list of changed files
CHANGED_FILES=$(git diff --name-only --diff-filter=ACM origin/$BASE_BRANCH...HEAD)

if [ -z "$CHANGED_FILES" ]; then
  echo "No files changed"
  exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES"
echo ""

# Scan changed files
echo "$CHANGED_FILES" | xargs mcp-sentinel scan --format json --output pr-scan.json

# Check results
TOTAL=$(jq '.summary.total_vulnerabilities' pr-scan.json)

if [ "$TOTAL" -gt 0 ]; then
  echo "❌ Found $TOTAL vulnerabilities in changed files"
  jq '.vulnerabilities[]' pr-scan.json
  exit 1
else
  echo "✅ No vulnerabilities found in changed files"
  exit 0
fi
```

### Pattern 2: Progressive Security Gate

```bash
#!/bin/bash
# progressive-gate.sh - Stricter checks for production

BRANCH=$(git rev-parse --abbrev-ref HEAD)

echo "Running security scan on branch: $BRANCH"

if [ "$BRANCH" = "main" ]; then
  # Production: Zero tolerance
  echo "🔒 Production branch - zero tolerance mode"
  mcp-sentinel scan . --min-severity low --format json --output scan.json

  TOTAL=$(jq '.summary.total_vulnerabilities' scan.json)
  if [ "$TOTAL" -gt 0 ]; then
    echo "❌ Cannot merge to main with any vulnerabilities"
    exit 1
  fi

elif [ "$BRANCH" = "develop" ]; then
  # Develop: Block critical and high
  echo "🧪 Develop branch - block critical/high"
  mcp-sentinel scan . --min-severity high --format json --output scan.json

  CRITICAL=$(jq '.summary.by_severity.critical' scan.json)
  HIGH=$(jq '.summary.by_severity.high' scan.json)

  if [ "$CRITICAL" -gt 0 ] || [ "$HIGH" -gt 0 ]; then
    echo "❌ Cannot merge with critical or high vulnerabilities"
    exit 1
  fi

else
  # Feature branches: Block only critical
  echo "🚀 Feature branch - block critical only"
  mcp-sentinel scan . --min-severity critical --format json --output scan.json

  CRITICAL=$(jq '.summary.by_severity.critical' scan.json)
  if [ "$CRITICAL" -gt 0 ]; then
    echo "❌ Fix critical vulnerabilities before merging"
    exit 1
  fi
fi

echo "✅ Security gate passed"
exit 0
```

### Pattern 3: Security Trend Tracking

```bash
#!/bin/bash
# track-security.sh - Track security metrics over time

RESULTS_DIR="security-reports"
mkdir -p "$RESULTS_DIR"

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
REPORT_FILE="$RESULTS_DIR/scan-$TIMESTAMP.json"

# Run scan
mcp-sentinel scan . --format json --output "$REPORT_FILE"

# Extract metrics
jq '{
  timestamp: .timestamp,
  total: .summary.total_vulnerabilities,
  critical: .summary.by_severity.critical,
  high: .summary.by_severity.high,
  medium: .summary.by_severity.medium,
  files_scanned: .summary.total_files
}' "$REPORT_FILE" > "$RESULTS_DIR/metrics-$TIMESTAMP.json"

# Generate trend report
echo "Security Trend Report"
echo "===================="
echo ""

jq -s 'sort_by(.timestamp) | .[] | "\(.timestamp): Total=\(.total) Critical=\(.critical) High=\(.high)"' \
  "$RESULTS_DIR"/metrics-*.json

# Check if improving
LATEST=$(ls -t "$RESULTS_DIR"/metrics-*.json | head -1)
PREVIOUS=$(ls -t "$RESULTS_DIR"/metrics-*.json | head -2 | tail -1)

if [ -f "$PREVIOUS" ]; then
  LATEST_TOTAL=$(jq '.total' "$LATEST")
  PREVIOUS_TOTAL=$(jq '.total' "$PREVIOUS")

  DIFF=$((LATEST_TOTAL - PREVIOUS_TOTAL))

  if [ "$DIFF" -gt 0 ]; then
    echo ""
    echo "⚠️ Security worsened: +$DIFF vulnerabilities"
  elif [ "$DIFF" -lt 0 ]; then
    echo ""
    echo "✅ Security improved: $DIFF vulnerabilities"
  else
    echo ""
    echo "➡️ No change in vulnerability count"
  fi
fi
```

---

## Before and After Fixes

### Example 1: Hardcoded Secrets

**Before:**

```python
# config.py
OPENAI_API_KEY = "sk-proj-xyz123abc"
DATABASE_PASSWORD = "admin123"
JWT_SECRET = "super-secret-key-12345"
```

**Scan:**

```bash
$ mcp-sentinel scan config.py

🔴 CRITICAL (3)
  [SECRETS-001] Hardcoded API key: line 2
  [SECRETS-002] Hardcoded password: line 3
  [SECRETS-003] Hardcoded JWT secret: line 4
```

**After:**

```python
# config.py
import os

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
DATABASE_PASSWORD = os.getenv("DATABASE_PASSWORD")
JWT_SECRET = os.getenv("JWT_SECRET")

# Validate required env vars
if not all([OPENAI_API_KEY, DATABASE_PASSWORD, JWT_SECRET]):
    raise ValueError("Missing required environment variables")
```

```bash
$ mcp-sentinel scan config.py
✅ No vulnerabilities found!
```

### Example 2: Command Injection

**Before:**

```python
def run_command(cmd):
    os.system(f"tool --process {cmd}")
```

**After:**

```python
def run_command(cmd):
    # Validate input
    if not cmd.isalnum():
        raise ValueError("Invalid command")

    # Use subprocess safely
    subprocess.run(["tool", "--process", cmd], check=True)
```

---

**Need more examples?** Check out:
- [User Guide](USER_GUIDE.md) - Comprehensive usage documentation
- [Best Practices](BEST_PRACTICES.md) - Security best practices
- [Advanced Usage](ADVANCED_USAGE.md) - Advanced features

**Happy scanning!** 🛡️
