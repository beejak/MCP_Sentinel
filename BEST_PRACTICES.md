# Security Scanning Best Practices

Best practices for using MCP Sentinel effectively in your development workflow.

## Table of Contents

1. [Development Workflow](#development-workflow)
2. [CI/CD Integration](#cicd-integration)
3. [Vulnerability Management](#vulnerability-management)
4. [Secure Coding Practices](#secure-coding-practices)
5. [Team Processes](#team-processes)
6. [Organizational Strategy](#organizational-strategy)

---

## Development Workflow

### Shift Left: Early Detection

**Principle:** Find security issues as early as possible in the development cycle.

```
Cost to fix:
Development: $1      (cheapest)
Testing: $10
Production: $100     (most expensive)
Post-incident: $1000+
```

**Implementation:**

```bash
# 1. Pre-commit hook (catches issues before commit)
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
git diff --cached --name-only | xargs mcp-sentinel scan --min-severity high
EOF
chmod +x .git/hooks/pre-commit

# 2. Pre-push hook (comprehensive scan before push)
cat > .git/hooks/pre-push << 'EOF'
#!/bin/bash
mcp-sentinel scan . --min-severity medium
EOF
chmod +x .git/hooks/pre-push

# 3. Editor integration (real-time feedback)
# VSCode task or file watcher
```

### Incremental Scanning

Scan only what changed for fast feedback:

```bash
# Scan files changed since last commit
git diff --name-only HEAD | xargs mcp-sentinel scan

# Scan uncommitted changes
git diff --name-only | xargs mcp-sentinel scan

# Scan files in current PR/MR
git diff --name-only origin/main...HEAD | xargs mcp-sentinel scan
```

###Regular Full Scans

Don't rely only on incremental scans:

```bash
# Daily full scan of main branch
# (cron: 0 0 * * *)
mcp-sentinel scan . --format json --output daily-scan-$(date +%Y%m%d).json

# Weekly comprehensive audit
# (cron: 0 0 * * 0)
mcp-sentinel scan . --format json > weekly-audit.json
```

### Local Development

Integrate security into daily workflow:

```bash
# Alias for quick scans
echo 'alias mcp-scan="mcp-sentinel scan"' >> ~/.bashrc
echo 'alias mcp-quick="mcp-sentinel scan . --min-severity high"' >> ~/.bashrc

# Quick check before committing
mcp-quick

# Full scan before creating PR
mcp-sentinel scan . --format json --output pr-scan.json
```

---

## CI/CD Integration

### Progressive Security Gates

Different standards for different stages:

```bash
#!/bin/bash
# progressive-gate.sh

BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$BRANCH" = "main" ]; then
  # Production: Zero tolerance
  echo "🔒 Production - Zero tolerance for vulnerabilities"
  mcp-sentinel scan . --min-severity low
  EXIT_CODE=$?
  if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ Cannot deploy to production with vulnerabilities"
    exit 1
  fi

elif [ "$BRANCH" = "develop" ]; then
  # Staging: Block critical and high
  echo "🧪 Staging - Block critical/high"
  mcp-sentinel scan . --min-severity high
  EXIT_CODE=$?
  if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ Fix critical/high vulnerabilities before merging"
    exit 1
  fi

else
  # Feature branches: Block only critical
  echo "🚀 Feature - Block critical only"
  mcp-sentinel scan . --min-severity critical
  EXIT_CODE=$?
  if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ Fix critical vulnerabilities"
    exit 1
  fi
fi

echo "✅ Security gate passed"
```

### Fail Fast Strategy

Run security scans early in pipeline:

```yaml
# GitHub Actions
stages:
  - security  # ← First! Fails fast if issues found
  - test
  - build
  - deploy

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Security Scan
        run: mcp-sentinel scan . --min-severity high
```

### Parallel Scanning

Speed up large projects:

```yaml
jobs:
  scan-python:
    runs-on: ubuntu-latest
    steps:
      - run: mcp-sentinel scan . --include "*.py"

  scan-javascript:
    runs-on: ubuntu-latest
    steps:
      - run: mcp-sentinel scan . --include "*.js" --include "*.ts"

  scan-other:
    runs-on: ubuntu-latest
    steps:
      - run: mcp-sentinel scan . --include "*.rb" --include "*.php"
```

### Caching for Performance

```yaml
# Cache MCP Sentinel binary
- uses: actions/cache@v3
  with:
    path: ~/.cargo/bin/mcp-sentinel
    key: mcp-sentinel-${{ runner.os }}-v1.5.0

# Cache scan results for unchanged files
- uses: actions/cache@v3
  with:
    path: .mcp-cache
    key: scan-cache-${{ hashFiles('**/*.py', '**/*.js') }}
```

### Notifications

Alert teams on security issues:

```yaml
- name: Notify on Security Issues
  if: failure()
  uses: 8398a7/action-slack@v3
  with:
    status: custom
    custom_payload: |
      {
        text: `🚨 Security scan failed on ${process.env.GITHUB_REF}`,
        attachments: [{
          color: 'danger',
          text: `View results: ${process.env.GITHUB_SERVER_URL}/${process.env.GITHUB_REPOSITORY}/actions/runs/${process.env.GITHUB_RUN_ID}`
        }]
      }
    webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

---

## Vulnerability Management

### Triaging Vulnerabilities

**Priority framework:**

```
┌─────────────┬──────────────┬────────────────┬─────────────────┐
│  Severity   │ Response Time│     Action     │   Responsible   │
├─────────────┼──────────────┼────────────────┼─────────────────┤
│  Critical   │  Immediate   │ Fix now, block │ On-call engineer│
│             │  (same day)  │ all deploys    │                 │
├─────────────┼──────────────┼────────────────┼─────────────────┤
│  High       │  1 week      │ Schedule fix,  │ Team lead       │
│             │              │ sprint priority│                 │
├─────────────┼──────────────┼────────────────┼─────────────────┤
│  Medium     │  1 month     │ Add to backlog │ Product manager │
│             │              │ plan for sprint│                 │
├─────────────┼──────────────┼────────────────┼─────────────────┤
│  Low        │  Flexible    │ Technical debt │ Team decision   │
│             │              │ fix when       │                 │
│             │              │ convenient     │                 │
└─────────────┴──────────────┴────────────────┴─────────────────┘
```

### Tracking Vulnerabilities

Create systematic tracking:

```bash
# Track over time
mkdir -p security-reports

# Daily scan
mcp-sentinel scan . --format json \
  --output security-reports/scan-$(date +%Y%m%d).json

# Extract trends
cat security-reports/scan-*.json | jq '{
  date: .timestamp,
  total: .summary.total_vulnerabilities,
  critical: .summary.by_severity.critical,
  high: .summary.by_severity.high
}' > security-trends.json

# Visualize trends
cat security-trends.json | jq -r '@csv' > trends.csv
```

### Issue Tickets

Create actionable tickets:

```bash
# Auto-create GitHub issues for critical vulnerabilities
mcp-sentinel scan . --format json | jq -r '
  .vulnerabilities[] |
  select(.severity == "critical") |
  "## \(.title)\n\n**File:** `\(.file):\(.line)`\n**Severity:** \(.severity)\n\n\(.description)\n\n### Recommendation\n\(.recommendation)"
' | while IFS= read -r body; do
  gh issue create \
    --title "[Security] Critical vulnerability found" \
    --body "$body" \
    --label "security,critical"
done
```

### False Positive Management

Document and track false positives:

```bash
# Create false-positive registry
cat > .mcp-false-positives.yml << EOF
# False positives registry
# Format: file:line:detector:reason

false_positives:
  - file: tests/test_auth.py
    line: 42
    detector: secrets
    reason: "Test fixture, not real credentials"
    reported_by: "alice@example.com"
    reviewed_by: "bob@example.com"
    date: "2025-10-25"

  - file: config/sample.py
    line: 10
    detector: secrets
    reason: "Example configuration, documented in README"
    reported_by: "alice@example.com"
    date: "2025-10-25"
EOF
```

---

## Secure Coding Practices

### Handling Secrets

**❌ Never do this:**
```python
# Hardcoded secrets
API_KEY = "sk-1234567890abcdef"
DB_PASSWORD = "admin123"
JWT_SECRET = "my-secret-key"
```

**✅ Always do this:**
```python
# Environment variables
import os

API_KEY = os.getenv("API_KEY")
if not API_KEY:
    raise ValueError("API_KEY environment variable required")

# Or use secrets manager
from cloud_secrets import get_secret
DB_PASSWORD = get_secret("database_password")
```

### Preventing Command Injection

**❌ Never do this:**
```python
# Unsafe command execution
os.system(f"tool {user_input}")
subprocess.call(f"process {data}", shell=True)
```

**✅ Always do this:**
```python
# Safe subprocess usage
subprocess.run(["tool", user_input], check=True, shell=False)

# With validation
if not user_input.isalnum():
    raise ValueError("Invalid input")
subprocess.run(["tool", user_input], check=True)
```

### Preventing SQL Injection

**❌ Never do this:**
```python
# String concatenation in SQL
query = f"SELECT * FROM users WHERE id = {user_id}"
cursor.execute(query)
```

**✅ Always do this:**
```python
# Parameterized queries
query = "SELECT * FROM users WHERE id = ?"
cursor.execute(query, (user_id,))

# Or use ORM
user = User.objects.get(id=user_id)
```

### Preventing Path Traversal

**❌ Never do this:**
```python
# Unsanitized file paths
def read_file(filename):
    return open(f"/data/{filename}").read()
# User can pass: "../../etc/passwd"
```

**✅ Always do this:**
```python
from pathlib import Path

def read_file(filename):
    base_path = Path("/data").resolve()
    file_path = (base_path / filename).resolve()

    # Ensure path is within base directory
    if not str(file_path).startswith(str(base_path)):
        raise ValueError("Invalid path")

    return file_path.read_text()
```

### Preventing SSRF

**❌ Never do this:**
```python
# Unrestricted URL fetching
import requests
def fetch_data(url):
    return requests.get(url).text
# User can access internal services!
```

**✅ Always do this:**
```python
import requests
from urllib.parse import urlparse
import ipaddress

ALLOWED_DOMAINS = ['api.example.com', 'trusted-api.com']

def fetch_data(url):
    parsed = urlparse(url)

    # Whitelist domains
    if parsed.netloc not in ALLOWED_DOMAINS:
        raise ValueError("Domain not allowed")

    # Block internal IPs
    try:
        ip = ipaddress.ip_address(parsed.hostname)
        if ip.is_private or ip.is_loopback:
            raise ValueError("Internal IP blocked")
    except ValueError:
        pass  # Not an IP, check DNS

    return requests.get(url, timeout=5, allow_redirects=False).text
```

### Input Validation

**Always validate user input:**

```python
def validate_input(data, expected_type, max_length=None, allowed_chars=None):
    """Validate and sanitize user input"""

    # Type check
    if not isinstance(data, expected_type):
        raise TypeError(f"Expected {expected_type}, got {type(data)}")

    # Length check
    if max_length and len(data) > max_length:
        raise ValueError(f"Input too long (max: {max_length})")

    # Character whitelist
    if allowed_chars:
        import re
        if not re.match(allowed_chars, data):
            raise ValueError("Input contains invalid characters")

    return data

# Usage
user_id = validate_input(
    request.GET['id'],
    expected_type=str,
    max_length=20,
    allowed_chars=r'^[a-zA-Z0-9_-]+$'
)
```

---

## Team Processes

### Code Review Security Checklist

**Before approving PR, verify:**

- [ ] MCP Sentinel scan passed
- [ ] No hardcoded secrets
- [ ] User input validated
- [ ] SQL queries parameterized
- [ ] File paths sanitized
- [ ] URLs whitelisted
- [ ] Error messages don't leak info
- [ ] Logging doesn't expose secrets
- [ ] Dependencies up to date
- [ ] Security tests added

### Security Champions

Designate security champions in each team:

**Responsibilities:**
- Run weekly security scans
- Triage security findings
- Review security-sensitive code
- Educate team on security best practices
- Liaison with security team

### Security Training

Regular security training for developers:

**Monthly topics:**
- Month 1: Secret management
- Month 2: Injection vulnerabilities
- Month 3: Authentication & authorization
- Month 4: Cryptography basics
- Month 5: Secure API design
- Month 6: Review and assessment

### Blameless Postmortems

When security issues reach production:

**Process:**
1. **Incident response** (stop the bleeding)
2. **Root cause analysis** (how did it happen?)
3. **Process improvements** (how to prevent?)
4. **Tool improvements** (detection gaps?)
5. **Knowledge sharing** (learn together)

**Never blame individuals.** Focus on improving processes and tools.

---

## Organizational Strategy

### Security Metrics

Track meaningful security metrics:

```bash
# Weekly security dashboard
cat > generate-dashboard.sh << 'EOF'
#!/bin/bash

echo "=== Security Dashboard ==="
echo ""

# Current status
mcp-sentinel scan . --format json --output current.json

TOTAL=$(jq '.summary.total_vulnerabilities' current.json)
CRITICAL=$(jq '.summary.by_severity.critical' current.json)
HIGH=$(jq '.summary.by_severity.high' current.json)

echo "Current Status:"
echo "  Total Vulnerabilities: $TOTAL"
echo "  Critical: $CRITICAL"
echo "  High: $HIGH"
echo ""

# Trends
if [ -f previous.json ]; then
  PREV_TOTAL=$(jq '.summary.total_vulnerabilities' previous.json)
  DIFF=$((TOTAL - PREV_TOTAL))

  if [ $DIFF -gt 0 ]; then
    echo "Trend: ⚠️  Increased by $DIFF"
  elif [ $DIFF -lt 0 ]; then
    echo "Trend: ✅ Decreased by ${DIFF#-}"
  else
    echo "Trend: ➡️  No change"
  fi
fi

# Goals
echo ""
echo "Goals:"
echo "  Target Critical: 0"
echo "  Target High: <5"
echo "  Target Total: <20"

# Save for next week
cp current.json previous.json
EOF

chmod +x generate-dashboard.sh
```

### Security Budget

Allocate time for security:

```
Sprint capacity: 100 story points

Allocation:
- Features: 60 points (60%)
- Bugs: 20 points (20%)
- Security: 10 points (10%) ← Dedicated security time
- Tech debt: 10 points (10%)
```

### Continuous Improvement

Regular security process reviews:

**Quarterly review:**
1. **Scan metrics:** Are we improving?
2. **Process gaps:** What slipped through?
3. **Tool effectiveness:** Is MCP Sentinel helping?
4. **Team feedback:** What's working? What's not?
5. **Action items:** Specific improvements

### Security Culture

Foster a security-aware culture:

**Principles:**
- **Security is everyone's responsibility**
- **It's okay to not know** (but learn!)
- **Ask questions** (no stupid questions)
- **Fail fast** (catch early, fix early)
- **Share knowledge** (teach others)
- **Celebrate wins** (acknowledge good security practices)

### Integration with SDLC

Security at every stage:

```
┌──────────────┬────────────────────────────────────────┐
│  SDLC Stage  │        Security Activities             │
├──────────────┼────────────────────────────────────────┤
│  Planning    │ - Threat modeling                      │
│              │ - Security requirements                │
├──────────────┼────────────────────────────────────────┤
│  Development │ - Secure coding                        │
│              │ - Pre-commit scanning                  │
├──────────────┼────────────────────────────────────────┤
│  Testing     │ - Security testing                     │
│              │ - Penetration testing                  │
├──────────────┼────────────────────────────────────────┤
│  Deployment  │ - CI/CD security gates                 │
│              │ - Production scanning                  │
├──────────────┼────────────────────────────────────────┤
│  Operations  │ - Monitoring                           │
│              │ - Incident response                    │
└──────────────┴────────────────────────────────────────┘
```

---

## Quick Reference

### Daily Developer Checklist

```bash
# Before starting work
git pull
mcp-sentinel scan . --min-severity critical

# Before committing
git diff | mcp-sentinel scan

# Before creating PR
mcp-sentinel scan . --format json --output pr-scan.json
# Review results

# After PR approved
git push
# CI/CD runs full scan
```

### Security Review Checklist

**For each PR:**
- [ ] MCP Sentinel scan passed
- [ ] No new critical/high issues
- [ ] Dependencies updated
- [ ] Security tests added
- [ ] Documentation updated

### Monthly Security Tasks

- [ ] Run full security audit
- [ ] Review security metrics
- [ ] Update dependencies
- [ ] Review and close old security issues
- [ ] Schedule security training
- [ ] Update security documentation

---

**Remember:** Security is a journey, not a destination. Continuous improvement is key! 🛡️

**Resources:**
- [User Guide](USER_GUIDE.md) - Comprehensive usage
- [Examples](EXAMPLES.md) - Real-world scenarios
- [CI/CD Integration](CI_CD_INTEGRATION.md) - Pipeline setup
- [FAQ](FAQ.md) - Common questions
