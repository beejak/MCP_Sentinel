# Advanced Usage Guide

Advanced features, techniques, and power-user tips for MCP Sentinel.

## Table of Contents

1. [Advanced Filtering](#advanced-filtering)
2. [Custom Workflows](#custom-workflows)
3. [Integration Patterns](#integration-patterns)
4. [Performance Tuning](#performance-tuning)
5. [Automation Scripts](#automation-scripts)
6. [Enterprise Features](#enterprise-features)

---

## Advanced Filtering

### Complex Include/Exclude Patterns

Combine multiple filters for precise targeting:

```bash
# Scan Python and JavaScript, exclude tests and generated code
mcp-sentinel scan . \
  --include "*.py" \
  --include "*.js" \
  --include "*.ts" \
  --exclude "*test*" \
  --exclude "*_test.*" \
  --exclude "*.spec.*" \
  --exclude "*.generated.*" \
  --exclude "node_modules/*" \
  --exclude "__pycache__/*" \
  --exclude "dist/*" \
  --exclude "build/*"
```

### Selective Detector Usage

Run specific detectors for targeted analysis:

```bash
# Security audit: Only secrets and sensitive files
mcp-sentinel scan . \
  --only-detector secrets \
  --only-detector sensitive-files

# Injection vulnerability audit
mcp-sentinel scan . \
  --only-detector command-injection \
  --only-detector sql-injection \
  --only-detector code-injection \
  --only-detector ssrf

# Pre-production check: Critical detectors only
mcp-sentinel scan . \
  --only-detector secrets \
  --only-detector command-injection \
  --only-detector deserialization
```

### Directory-Specific Scanning

Scan different directories with different policies:

```bash
#!/bin/bash
# directory-scan.sh - Different policies per directory

# Strict for production code
echo "Scanning production code..."
mcp-sentinel scan ./src \
  --min-severity medium \
  --format json \
  --output src-scan.json

# Lenient for tests
echo "Scanning tests..."
mcp-sentinel scan ./tests \
  --min-severity high \
  --format json \
  --output tests-scan.json

# Analysis only for scripts
echo "Scanning scripts..."
mcp-sentinel scan ./scripts \
  --min-severity critical \
  --format json \
  --output scripts-scan.json

# Combine results
jq -s 'reduce .[] as $item ({}; .vulnerabilities += $item.vulnerabilities)' \
  src-scan.json tests-scan.json scripts-scan.json > combined-scan.json

echo "Combined results:"
jq '.vulnerabilities | length' combined-scan.json
```

---

## Custom Workflows

### Differential Scanning

Scan only what changed between branches:

```bash
#!/bin/bash
# diff-scan.sh - Scan only changes

BASE_BRANCH=${1:-main}
TARGET_BRANCH=${2:-$(git rev-parse --abbrev-ref HEAD)}

echo "Scanning differences between $BASE_BRANCH and $TARGET_BRANCH"

# Get changed files
CHANGED_FILES=$(git diff --name-only $BASE_BRANCH...$TARGET_BRANCH)

if [ -z "$CHANGED_FILES" ]; then
  echo "No changes detected"
  exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES"
echo ""

# Create temp file with changes
TEMP_DIR=$(mktemp -d)
echo "$CHANGED_FILES" | while read file; do
  if [ -f "$file" ]; then
    mkdir -p "$TEMP_DIR/$(dirname $file)"
    cp "$file" "$TEMP_DIR/$file"
  fi
done

# Scan only changed files
mcp-sentinel scan "$TEMP_DIR" --format json --output diff-scan.json

# Cleanup
rm -rf "$TEMP_DIR"

# Report
VULNERABILITIES=$(jq '.summary.total_vulnerabilities' diff-scan.json)
echo ""
echo "Found $VULNERABILITIES vulnerabilities in changed files"

if [ "$VULNERABILITIES" -gt 0 ]; then
  jq '.vulnerabilities[]' diff-scan.json
  exit 1
fi

exit 0
```

### Progressive Scanning Strategy

Gradually improve security posture:

```bash
#!/bin/bash
# progressive-scan.sh - Phased security improvement

PHASE=${1:-1}

case $PHASE in
  1)
    echo "Phase 1: Critical issues only"
    mcp-sentinel scan . --min-severity critical --format json --output phase1.json
    CRITICAL=$(jq '.summary.by_severity.critical' phase1.json)

    if [ "$CRITICAL" -eq 0 ]; then
      echo "✅ Phase 1 complete: No critical issues"
      echo "Run with phase 2 next"
    else
      echo "❌ $CRITICAL critical issues found"
      echo "Fix these before moving to phase 2"
      exit 1
    fi
    ;;

  2)
    echo "Phase 2: High severity issues"
    mcp-sentinel scan . --min-severity high --format json --output phase2.json
    HIGH=$(jq '.summary.by_severity.high' phase2.json)

    if [ "$HIGH" -eq 0 ]; then
      echo "✅ Phase 2 complete: No high severity issues"
      echo "Run with phase 3 next"
    else
      echo "❌ $HIGH high severity issues found"
      echo "Fix these before moving to phase 3"
      exit 1
    fi
    ;;

  3)
    echo "Phase 3: Medium severity issues"
    mcp-sentinel scan . --min-severity medium --format json --output phase3.json
    MEDIUM=$(jq '.summary.by_severity.medium' phase3.json)

    if [ "$MEDIUM" -le 10 ]; then
      echo "✅ Phase 3 complete: Only $MEDIUM medium issues (threshold: 10)"
      echo "Run with phase 4 for full audit"
    else
      echo "⚠️  $MEDIUM medium issues found (threshold: 10)"
      echo "Reduce below 10 before phase 4"
      exit 1
    fi
    ;;

  4)
    echo "Phase 4: Complete audit (all severities)"
    mcp-sentinel scan . --format json --output phase4.json
    TOTAL=$(jq '.summary.total_vulnerabilities' phase4.json)

    echo "📊 Final audit: $TOTAL total issues"
    jq '.summary.by_severity' phase4.json
    ;;

  *)
    echo "Usage: $0 [1|2|3|4]"
    echo "  1: Critical only"
    echo "  2: Critical + High"
    echo "  3: Critical + High + Medium"
    echo "  4: Full audit"
    exit 1
    ;;
esac
```

### Risk-Based Scanning

Prioritize high-risk components:

```bash
#!/bin/bash
# risk-scan.sh - Scan based on component risk

# High-risk: Authentication, authorization, payment
echo "🔴 Scanning high-risk components..."
mcp-sentinel scan ./src/auth ./src/payment ./src/api \
  --min-severity medium \
  --format json \
  --output high-risk-scan.json

HIGH_RISK=$(jq '.summary.total_vulnerabilities' high-risk-scan.json)

# Medium-risk: Business logic
echo "🟡 Scanning medium-risk components..."
mcp-sentinel scan ./src/services ./src/models \
  --min-severity high \
  --format json \
  --output medium-risk-scan.json

MEDIUM_RISK=$(jq '.summary.total_vulnerabilities' medium-risk-scan.json)

# Low-risk: UI, utilities
echo "🟢 Scanning low-risk components..."
mcp-sentinel scan ./src/ui ./src/utils \
  --min-severity critical \
  --format json \
  --output low-risk-scan.json

LOW_RISK=$(jq '.summary.total_vulnerabilities' low-risk-scan.json)

# Summary
echo ""
echo "=== Risk-Based Scan Summary ==="
echo "High-risk components: $HIGH_RISK issues"
echo "Medium-risk components: $MEDIUM_RISK issues"
echo "Low-risk components: $LOW_RISK issues"

# Fail if high-risk components have issues
if [ "$HIGH_RISK" -gt 0 ]; then
  echo "❌ High-risk components have vulnerabilities!"
  exit 1
fi

echo "✅ High-risk components are secure"
```

---

## Integration Patterns

### Webhook Integration

Send scan results to external services:

```bash
#!/bin/bash
# webhook-scan.sh - Scan and send to webhook

WEBHOOK_URL="https://your-service.com/api/scans"

# Run scan
mcp-sentinel scan . --format json --output scan-results.json

# Send to webhook
curl -X POST "$WEBHOOK_URL" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $API_TOKEN" \
  -d @scan-results.json

echo "Scan results sent to webhook"
```

### Database Integration

Store scan results in database:

```python
# db-integration.py
import json
import subprocess
import sqlite3
from datetime import datetime

def run_scan(path):
    """Run MCP Sentinel scan"""
    result = subprocess.run(
        ['mcp-sentinel', 'scan', path, '--format', 'json'],
        capture_output=True,
        text=True
    )
    return json.loads(result.stdout)

def store_scan(scan_data, db_path='security.db'):
    """Store scan results in database"""
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    # Create tables
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS scans (
            id INTEGER PRIMARY KEY,
            timestamp TEXT,
            total_vulnerabilities INTEGER,
            critical INTEGER,
            high INTEGER,
            medium INTEGER,
            low INTEGER
        )
    ''')

    cursor.execute('''
        CREATE TABLE IF NOT EXISTS vulnerabilities (
            id INTEGER PRIMARY KEY,
            scan_id INTEGER,
            severity TEXT,
            file TEXT,
            line INTEGER,
            detector TEXT,
            title TEXT,
            FOREIGN KEY (scan_id) REFERENCES scans(id)
        )
    ''')

    # Insert scan summary
    summary = scan_data['summary']
    cursor.execute('''
        INSERT INTO scans (timestamp, total_vulnerabilities, critical, high, medium, low)
        VALUES (?, ?, ?, ?, ?, ?)
    ''', (
        datetime.now().isoformat(),
        summary['total_vulnerabilities'],
        summary['by_severity']['critical'],
        summary['by_severity']['high'],
        summary['by_severity']['medium'],
        summary['by_severity']['low']
    ))

    scan_id = cursor.lastrowid

    # Insert vulnerabilities
    for vuln in scan_data['vulnerabilities']:
        cursor.execute('''
            INSERT INTO vulnerabilities (scan_id, severity, file, line, detector, title)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (
            scan_id,
            vuln['severity'],
            vuln['file'],
            vuln['line'],
            vuln['detector'],
            vuln['title']
        ))

    conn.commit()
    conn.close()

    print(f"Scan {scan_id} stored in database")

if __name__ == '__main__':
    scan_data = run_scan('.')
    store_scan(scan_data)
```

### Jira Integration

Create Jira tickets for vulnerabilities:

```python
# jira-integration.py
import json
import subprocess
from jira import JIRA

def create_jira_tickets(scan_file, jira_url, jira_user, jira_token, project_key):
    """Create Jira tickets for critical/high vulnerabilities"""

    # Connect to Jira
    jira = JIRA(server=jira_url, basic_auth=(jira_user, jira_token))

    # Load scan results
    with open(scan_file) as f:
        scan_data = json.load(f)

    # Create tickets for critical and high severity
    for vuln in scan_data['vulnerabilities']:
        if vuln['severity'] in ['critical', 'high']:

            # Check if ticket already exists
            query = f'project={project_key} AND summary ~ "{vuln["title"]}" AND status != Done'
            existing = jira.search_issues(query)

            if existing:
                print(f"Ticket already exists for: {vuln['title']}")
                continue

            # Create new ticket
            issue_dict = {
                'project': {'key': project_key},
                'summary': f"[Security] {vuln['title']}",
                'description': f"""
                    *File:* {vuln['file']}:{vuln['line']}
                    *Severity:* {vuln['severity'].upper()}
                    *Detector:* {vuln['detector']}

                    h3. Description
                    {vuln['description']}

                    h3. Recommendation
                    {vuln['recommendation']}

                    h3. Context
                    {{code}}
                    {vuln['context']}
                    {{code}}
                """,
                'issuetype': {'name': 'Bug'},
                'priority': {'name': 'Critical' if vuln['severity'] == 'critical' else 'High'},
                'labels': ['security', vuln['severity'], vuln['detector']]
            }

            new_issue = jira.create_issue(fields=issue_dict)
            print(f"Created ticket: {new_issue.key} for {vuln['title']}")

if __name__ == '__main__':
    import os

    create_jira_tickets(
        scan_file='scan-results.json',
        jira_url=os.getenv('JIRA_URL'),
        jira_user=os.getenv('JIRA_USER'),
        jira_token=os.getenv('JIRA_TOKEN'),
        project_key='SEC'
    )
```

### Slack Integration

Send daily security reports to Slack:

```bash
#!/bin/bash
# slack-report.sh - Daily security report to Slack

SLACK_WEBHOOK="$SLACK_WEBHOOK_URL"

# Run scan
mcp-sentinel scan . --format json --output daily-scan.json

# Parse results
TOTAL=$(jq '.summary.total_vulnerabilities' daily-scan.json)
CRITICAL=$(jq '.summary.by_severity.critical' daily-scan.json)
HIGH=$(jq '.summary.by_severity.high' daily-scan.json)
MEDIUM=$(jq '.summary.by_severity.medium' daily-scan.json)

# Determine status emoji
if [ "$CRITICAL" -gt 0 ]; then
  STATUS="🔴"
  COLOR="danger"
elif [ "$HIGH" -gt 0 ]; then
  STATUS="🟠"
  COLOR="warning"
elif [ "$TOTAL" -gt 0 ]; then
  STATUS="🟡"
  COLOR="warning"
else
  STATUS="✅"
  COLOR="good"
fi

# Create Slack message
cat > slack-message.json <<EOF
{
  "text": "${STATUS} Daily Security Scan Report",
  "attachments": [
    {
      "color": "${COLOR}",
      "fields": [
        {
          "title": "Total Vulnerabilities",
          "value": "${TOTAL}",
          "short": true
        },
        {
          "title": "Critical",
          "value": "${CRITICAL}",
          "short": true
        },
        {
          "title": "High",
          "value": "${HIGH}",
          "short": true
        },
        {
          "title": "Medium",
          "value": "${MEDIUM}",
          "short": true
        }
      ],
      "footer": "MCP Sentinel",
      "ts": $(date +%s)
    }
  ]
}
EOF

# Send to Slack
curl -X POST "$SLACK_WEBHOOK" \
  -H "Content-Type: application/json" \
  -d @slack-message.json

echo "Report sent to Slack"
```

---

## Performance Tuning

### Parallel Scanning Strategy

Maximize throughput for large projects:

```bash
#!/bin/bash
# parallel-scan.sh - Parallel scanning for large projects

THREADS=${1:-8}

# Split project into chunks
echo "Splitting project into chunks..."
find . -type f \( -name "*.py" -o -name "*.js" -o -name "*.ts" \) \
  > all-files.txt

split -n l/$THREADS all-files.txt chunk-

# Scan chunks in parallel
echo "Scanning $THREADS chunks in parallel..."
for chunk in chunk-*; do
  (
    cat "$chunk" | xargs mcp-sentinel scan --format json --output "${chunk}.json"
  ) &
done

# Wait for all scans to complete
wait

# Combine results
echo "Combining results..."
jq -s 'reduce .[] as $item (
  {
    vulnerabilities: [],
    summary: {
      total_vulnerabilities: 0,
      by_severity: {critical: 0, high: 0, medium: 0, low: 0, info: 0}
    }
  };
  .vulnerabilities += $item.vulnerabilities |
  .summary.total_vulnerabilities += $item.summary.total_vulnerabilities |
  .summary.by_severity.critical += $item.summary.by_severity.critical |
  .summary.by_severity.high += $item.summary.by_severity.high |
  .summary.by_severity.medium += $item.summary.by_severity.medium |
  .summary.by_severity.low += $item.summary.by_severity.low |
  .summary.by_severity.info += $item.summary.by_severity.info
)' chunk-*.json > combined-results.json

# Cleanup
rm chunk-* all-files.txt

echo "Scan complete:"
jq '.summary' combined-results.json
```

### Caching Strategy

Cache results for unchanged files:

```bash
#!/bin/bash
# cached-scan.sh - Scan with file-level caching

CACHE_DIR=".mcp-cache"
mkdir -p "$CACHE_DIR"

# Function to get file hash
file_hash() {
  sha256sum "$1" | cut -d' ' -f1
}

# Scan with caching
scan_cached() {
  local file="$1"
  local hash=$(file_hash "$file")
  local cache_file="$CACHE_DIR/$hash.json"

  if [ -f "$cache_file" ]; then
    echo "Cache hit: $file"
    cat "$cache_file"
  else
    echo "Cache miss: $file"
    mcp-sentinel scan "$file" --format json --output "$cache_file"
    cat "$cache_file"
  fi
}

# Find all files and scan
find . -type f \( -name "*.py" -o -name "*.js" \) | while read file; do
  scan_cached "$file"
done | jq -s 'reduce .[] as $item (
  {vulnerabilities: []};
  .vulnerabilities += $item.vulnerabilities
)' > final-results.json

echo "Scan complete with caching"
jq '.vulnerabilities | length' final-results.json
```

### Incremental CI/CD Scanning

Only scan changed files in CI/CD:

```yaml
# .github/workflows/incremental-scan.yml
name: Incremental Security Scan

on:
  pull_request:

jobs:
  scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0  # Full history for diff

      - name: Get changed files
        id: changed
        run: |
          git diff --name-only ${{ github.event.pull_request.base.sha }}...${{ github.sha }} \
            > changed-files.txt
          echo "files=$(cat changed-files.txt | tr '\n' ' ')" >> $GITHUB_OUTPUT

      - name: Scan changed files only
        run: |
          if [ -s changed-files.txt ]; then
            cat changed-files.txt | xargs mcp-sentinel scan --format json --output results.json
          else
            echo '{"vulnerabilities": [], "summary": {"total_vulnerabilities": 0}}' > results.json
          fi

      - name: Check results
        run: |
          TOTAL=$(jq '.summary.total_vulnerabilities' results.json)
          if [ "$TOTAL" -gt 0 ]; then
            echo "Found $TOTAL vulnerabilities in changed files"
            exit 1
          fi
```

---

## Automation Scripts

### Continuous Monitoring

Monitor repository continuously:

```bash
#!/bin/bash
# continuous-monitor.sh - Watch for changes and scan

WATCH_DIR=${1:-.}
SCAN_DELAY=60  # seconds

echo "Monitoring $WATCH_DIR for changes..."

while true; do
  # Find recently modified files (last $SCAN_DELAY seconds)
  find "$WATCH_DIR" -type f \
    \( -name "*.py" -o -name "*.js" -o -name "*.ts" \) \
    -mtime -${SCAN_DELAY}s -print0 | \
    xargs -0 -r mcp-sentinel scan --format json --output monitor-scan.json

  # Check if any vulnerabilities found
  if [ -f monitor-scan.json ]; then
    VULNS=$(jq '.summary.total_vulnerabilities' monitor-scan.json)
    if [ "$VULNS" -gt 0 ]; then
      echo "[$(date)] ⚠️  Found $VULNS vulnerabilities in modified files"
      # Send notification
      notify-send "MCP Sentinel" "Found $VULNS vulnerabilities"
    fi
  fi

  sleep "$SCAN_DELAY"
done
```

### Auto-Fix Script (Template)

Template for auto-fixing common issues:

```python
# auto-fix.py - Template for automated fixes
import json
import re
from pathlib import Path

def load_scan_results(scan_file):
    """Load scan results from JSON"""
    with open(scan_file) as f:
        return json.load(f)

def fix_hardcoded_secret(file_path, line_number, pattern):
    """Example: Replace hardcoded secret with env var"""
    content = Path(file_path).read_text()
    lines = content.split('\n')

    # Find the line
    target_line = lines[line_number - 1]

    # Extract variable name
    match = re.search(r'(\w+)\s*=\s*["\']([^"\']+)["\']', target_line)
    if match:
        var_name = match.group(1)
        # Replace with env var
        fixed_line = f'{var_name} = os.getenv("{var_name}")'
        lines[line_number - 1] = fixed_line

        # Write back
        Path(file_path).write_text('\n'.join(lines))
        print(f"Fixed: {file_path}:{line_number}")
        return True

    return False

def auto_fix(scan_results):
    """Attempt to auto-fix vulnerabilities"""
    fixed_count = 0

    for vuln in scan_results['vulnerabilities']:
        if vuln['detector'] == 'secrets':
            if fix_hardcoded_secret(vuln['file'], vuln['line'], vuln['context']):
                fixed_count += 1

    print(f"Auto-fixed {fixed_count} vulnerabilities")
    print("⚠️  WARNING: Review all changes before committing!")

if __name__ == '__main__':
    scan_results = load_scan_results('scan-results.json')
    auto_fix(scan_results)
```

---

## Enterprise Features

### Multi-Repository Scanning

Scan multiple repositories:

```bash
#!/bin/bash
# multi-repo-scan.sh - Scan multiple repositories

REPOS=(
  "git@github.com:company/repo1.git"
  "git@github.com:company/repo2.git"
  "git@github.com:company/repo3.git"
)

SCAN_DIR="/tmp/multi-repo-scan"
mkdir -p "$SCAN_DIR"

for repo in "${REPOS[@]}"; do
  repo_name=$(basename "$repo" .git)
  echo "Scanning $repo_name..."

  # Clone or update
  if [ -d "$SCAN_DIR/$repo_name" ]; then
    cd "$SCAN_DIR/$repo_name" && git pull
  else
    git clone "$repo" "$SCAN_DIR/$repo_name"
  fi

  # Scan
  mcp-sentinel scan "$SCAN_DIR/$repo_name" \
    --format json \
    --output "$SCAN_DIR/$repo_name-scan.json"
done

# Aggregate results
jq -s '
  map({
    repo: input_filename | gsub(".*/(.*)-scan\\.json"; "\\1"),
    vulnerabilities: .summary.total_vulnerabilities,
    critical: .summary.by_severity.critical,
    high: .summary.by_severity.high
  })
' "$SCAN_DIR"/*-scan.json > aggregated-report.json

echo "Multi-repository scan complete:"
jq '.' aggregated-report.json
```

### Compliance Reporting

Generate compliance reports:

```python
# compliance-report.py
import json
from datetime import datetime
from pathlib import Path

def generate_compliance_report(scan_file, output_file='compliance-report.html'):
    """Generate HTML compliance report"""

    with open(scan_file) as f:
        scan_data = json.load(f)

    # Calculate compliance metrics
    total_vulns = scan_data['summary']['total_vulnerabilities']
    critical = scan_data['summary']['by_severity']['critical']
    high = scan_data['summary']['by_severity']['high']

    # Compliance status
    compliant = critical == 0 and high == 0

    html = f"""
    <!DOCTYPE html>
    <html>
    <head>
        <title>Security Compliance Report</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 40px; }}
            .header {{ border-bottom: 3px solid #333; padding-bottom: 20px; }}
            .status {{ font-size: 24px; font-weight: bold; margin: 20px 0; }}
            .compliant {{ color: green; }}
            .non-compliant {{ color: red; }}
            table {{ border-collapse: collapse; width: 100%; margin: 20px 0; }}
            th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
            th {{ background-color: #f2f2f2; }}
            .critical {{ background-color: #ffebee; }}
            .high {{ background-color: #fff3e0; }}
        </style>
    </head>
    <body>
        <div class="header">
            <h1>Security Compliance Report</h1>
            <p>Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
            <p>Scanner: MCP Sentinel v1.5.0</p>
        </div>

        <div class="status {'compliant' if compliant else 'non-compliant'}">
            Status: {'✅ COMPLIANT' if compliant else '❌ NON-COMPLIANT'}
        </div>

        <h2>Executive Summary</h2>
        <table>
            <tr>
                <th>Metric</th>
                <th>Value</th>
                <th>Threshold</th>
                <th>Status</th>
            </tr>
            <tr class="{'compliant' if critical == 0 else 'critical'}">
                <td>Critical Vulnerabilities</td>
                <td>{critical}</td>
                <td>0</td>
                <td>{'✅ Pass' if critical == 0 else '❌ Fail'}</td>
            </tr>
            <tr class="{'compliant' if high == 0 else 'high'}">
                <td>High Severity</td>
                <td>{high}</td>
                <td>0</td>
                <td>{'✅ Pass' if high == 0 else '❌ Fail'}</td>
            </tr>
            <tr>
                <td>Total Vulnerabilities</td>
                <td>{total_vulns}</td>
                <td>-</td>
                <td>-</td>
            </tr>
        </table>

        <h2>Compliance Standards</h2>
        <ul>
            <li>✅ OWASP Top 10 Coverage</li>
            <li>✅ CWE Top 25 Detection</li>
            <li>✅ PCI DSS 6.5 Requirements</li>
            <li>{'✅' if compliant else '❌'} Zero Critical/High Policy</li>
        </ul>

        <h2>Detailed Findings</h2>
        <table>
            <tr>
                <th>Severity</th>
                <th>File</th>
                <th>Line</th>
                <th>Issue</th>
            </tr>
    """

    for vuln in scan_data['vulnerabilities']:
        severity_class = vuln['severity']
        html += f"""
            <tr class="{severity_class}">
                <td>{vuln['severity'].upper()}</td>
                <td>{vuln['file']}</td>
                <td>{vuln['line']}</td>
                <td>{vuln['title']}</td>
            </tr>
        """

    html += """
        </table>
    </body>
    </html>
    """

    Path(output_file).write_text(html)
    print(f"Compliance report generated: {output_file}")

if __name__ == '__main__':
    generate_compliance_report('scan-results.json')
```

---

**You're now a MCP Sentinel power user!** 🚀

**Resources:**
- [User Guide](USER_GUIDE.md) - Complete feature documentation
- [Examples](EXAMPLES.md) - Real-world scenarios
- [Best Practices](BEST_PRACTICES.md) - Security best practices
- [CI/CD Integration](CI_CD_INTEGRATION.md) - Pipeline integration

**Happy scanning!** 🛡️
