# MCP Sentinel Command Reference v1.5.0

Complete command-line reference for MCP Sentinel.

## Table of Contents

- [Global Flags](#global-flags)
- [scan - Scan for vulnerabilities](#scan-command)
- [whitelist - Manage whitelisted findings](#whitelist-command)
- [jobs - Manage async scan jobs](#jobs-command)
- [proxy - Runtime monitoring (Phase 3)](#proxy-command)
- [monitor - Real-time dashboard (Phase 3)](#monitor-command)
- [audit - Deep analysis (Phase 2)](#audit-command)
- [init - Create config file (Phase 2)](#init-command)
- [rules - Custom detection rules (Phase 2)](#rules-command)
- [Exit Codes](#exit-codes)
- [Environment Variables](#environment-variables)

## Global Flags

These flags work with any command:

```bash
--verbose, -v          Enable verbose logging (DEBUG level)
--quiet, -q            Suppress non-error output
--no-color             Disable colored output
--config PATH          Use custom configuration file
--version              Show version information
--help, -h             Show help information
```

## scan Command

Scan a directory for security vulnerabilities.

### Synopsis

```bash
mcp-sentinel scan [OPTIONS] <PATH>
```

### Arguments

- `<PATH>` - Directory or file to scan (required)

### Options

#### Output Options

```bash
--format FORMAT        Output format
                       Values: terminal (default), json, csv, html
                       Example: --format json

--output FILE          Write output to file instead of stdout
                       Example: --output report.json

--no-color             Disable colored terminal output
```

#### Filtering Options

```bash
--exclude PATTERN      Exclude files matching glob pattern (can be used multiple times)
                       Example: --exclude "*.test.js" --exclude "node_modules/*"

--include PATTERN      Only scan files matching pattern
                       Example: --include "src/**/*.py"

--fail-on SEVERITY     Exit with code 1 if issues of this severity or higher are found
                       Values: critical, high, medium, low
                       Example: --fail-on high

--min-confidence N     Only report issues with confidence >= N (0.0 to 1.0)
                       Example: --min-confidence 0.85
```

#### Detector Options

```bash
--detectors LIST       Comma-separated list of detectors to run
                       Default: all
                       Values: secrets, command_injection, code_injection,
                               deserialization, path_traversal, sql_injection,
                               ssrf, tool_poisoning, prompt_injection,
                               sensitive_files
                       Example: --detectors secrets,code_injection

--skip-detectors LIST  Comma-separated list of detectors to skip
                       Example: --skip-detectors tool_poisoning
```

#### Performance Options

```bash
--async                Run scan as background job (returns job ID)
                       Example: mcp-sentinel scan ./large-project --async

--parallel N           Number of parallel workers (default: CPU count)
                       Example: --parallel 4

--timeout SECONDS      Max scan duration in seconds
                       Example: --timeout 300
```

#### Policy & Whitelist

```bash
--policy FILE          Enforce security policy from file
                       Example: --policy .mcp-sentinel-policy.yaml

--whitelist FILE       Use whitelist from file
                       Example: --whitelist whitelist.json

--ignore-whitelist     Ignore whitelist and report all findings
```

### Examples

#### Basic Scanning

```bash
# Scan current directory
mcp-sentinel scan .

# Scan specific directory with verbose output
mcp-sentinel scan ./my-mcp-server --verbose

# Scan and save JSON output
mcp-sentinel scan ./project --format json --output report.json
```

#### CI/CD Integration

```bash
# Fail build if critical or high severity issues found
mcp-sentinel scan . --fail-on high

# Fail on any issues with high confidence
mcp-sentinel scan . --fail-on low --min-confidence 0.90

# Generate JSON report for further processing
mcp-sentinel scan . --format json --output results.json
if [ $? -eq 1 ]; then
  echo "Security vulnerabilities found!"
  exit 1
fi
```

#### Advanced Filtering

```bash
# Only scan Python files
mcp-sentinel scan . --include "**/*.py"

# Exclude test files and node_modules
mcp-sentinel scan . \
  --exclude "**/*test*.py" \
  --exclude "**/node_modules/**"

# Run only secrets and code injection detectors
mcp-sentinel scan . --detectors secrets,code_injection
```

#### Policy Enforcement

```bash
# Scan with policy file
mcp-sentinel scan . --policy .mcp-sentinel-policy.yaml

# Example policy file content:
# version: 1.5.0
# policies:
#   - name: "No Critical Issues"
#     rule: "critical_count == 0"
#     action: fail
#   - name: "Risk Score Threshold"
#     rule: "risk_score < 50"
#     action: fail
```

#### Async Scanning (Large Codebases)

```bash
# Start async scan
mcp-sentinel scan ./large-project --async
# Output: Started scan job: job-abc123def456

# Check job status
mcp-sentinel jobs status job-abc123def456

# Get results when complete
mcp-sentinel jobs result job-abc123def456 --format json
```

### Exit Codes

- `0` - Success, no vulnerabilities found (or below --fail-on threshold)
- `1` - Vulnerabilities found (at or above --fail-on threshold)
- `2` - Error (invalid arguments, scan failure, etc.)

## whitelist Command

Manage whitelisted findings to reduce false positives.

### Synopsis

```bash
mcp-sentinel whitelist <SUBCOMMAND> [OPTIONS]
```

### Subcommands

#### add - Add to whitelist

```bash
mcp-sentinel whitelist add <VULN_ID> [OPTIONS]

Options:
  --reason TEXT      Reason for whitelisting (required)
  --expires DATE     Expiration date (YYYY-MM-DD)
  --pattern REGEX    Whitelist by regex pattern instead of specific finding

Examples:
  # Whitelist specific finding
  mcp-sentinel whitelist add SEC-001 --reason "Test code, not production"

  # Whitelist with expiration
  mcp-sentinel whitelist add CODE-INJ-003 \
    --reason "Temporary exception for migration" \
    --expires 2025-12-31

  # Whitelist by pattern
  mcp-sentinel whitelist add --pattern "test_.*\.py" \
    --reason "All test files are safe"
```

#### list - List whitelisted items

```bash
mcp-sentinel whitelist list [OPTIONS]

Options:
  --format FORMAT    Output format: terminal (default), json
  --show-expired     Include expired whitelist entries

Example:
  mcp-sentinel whitelist list --format json
```

#### remove - Remove from whitelist

```bash
mcp-sentinel whitelist remove <HASH>

Example:
  mcp-sentinel whitelist remove a1b2c3d4e5f6
```

#### export - Export whitelist

```bash
mcp-sentinel whitelist export <FILE>

Example:
  mcp-sentinel whitelist export whitelist.json
```

#### import - Import whitelist

```bash
mcp-sentinel whitelist import <FILE>

Example:
  mcp-sentinel whitelist import whitelist.json
```

## jobs Command

Manage async scan jobs (for large codebases).

### Synopsis

```bash
mcp-sentinel jobs <SUBCOMMAND> [OPTIONS]
```

### Subcommands

#### status - Check job status

```bash
mcp-sentinel jobs status <JOB_ID>

Example:
  mcp-sentinel jobs status job-abc123def456

Output:
  Job ID: job-abc123def456
  Status: Running
  Progress: 45% (234/520 files scanned)
  Started: 2025-10-25 14:30:15
  Elapsed: 2m 15s
```

#### list - List all jobs

```bash
mcp-sentinel jobs list [OPTIONS]

Options:
  --status STATUS    Filter by status: pending, running, completed, failed
  --limit N          Max number of jobs to show (default: 20)

Example:
  mcp-sentinel jobs list --status running
```

#### result - Get job result

```bash
mcp-sentinel jobs result <JOB_ID> [OPTIONS]

Options:
  --format FORMAT    Output format: terminal, json, csv, html
  --output FILE      Write to file

Example:
  mcp-sentinel jobs result job-abc123 --format json --output result.json
```

#### cancel - Cancel running job

```bash
mcp-sentinel jobs cancel <JOB_ID>

Example:
  mcp-sentinel jobs cancel job-abc123
```

#### clean - Clean old jobs

```bash
mcp-sentinel jobs clean [OPTIONS]

Options:
  --older-than DAYS  Remove jobs older than N days (default: 7)
  --status STATUS    Only clean jobs with this status

Example:
  mcp-sentinel jobs clean --older-than 30
```

## Exit Codes

MCP Sentinel uses the following exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success - No vulnerabilities found or below threshold |
| 1 | Vulnerabilities found - At or above --fail-on threshold |
| 2 | Error - Invalid arguments, file not found, scan failure |
| 3 | Policy violation - Security policy rules violated |

## Environment Variables

Configure MCP Sentinel behavior via environment variables:

```bash
# Logging
RUST_LOG=mcp_sentinel=debug    # Set log level
MCP_SENTINEL_LOG_FILE=/path    # Log to file

# Configuration
MCP_SENTINEL_CONFIG=/path      # Default config file location
MCP_SENTINEL_WHITELIST=/path   # Default whitelist file location

# Performance
MCP_SENTINEL_MAX_FILE_SIZE     # Max file size to scan (bytes, default: 10MB)
MCP_SENTINEL_TIMEOUT           # Default scan timeout (seconds)
MCP_SENTINEL_PARALLEL_WORKERS  # Number of parallel workers

# Output
NO_COLOR=1                     # Disable colored output
FORCE_COLOR=1                  # Force colored output even in non-TTY

# Database
MCP_SENTINEL_DB_PATH           # SQLite database path for jobs
```

### Example Usage

```bash
# Enable debug logging and scan
export RUST_LOG=mcp_sentinel=debug
mcp-sentinel scan ./project

# Increase max file size and parallel workers
export MCP_SENTINEL_MAX_FILE_SIZE=52428800  # 50MB
export MCP_SENTINEL_PARALLEL_WORKERS=8
mcp-sentinel scan ./large-project
```

## Configuration File

Create a `.mcp-sentinel.yaml` config file in your project root:

```yaml
version: 1.5.0

# Scan settings
scan:
  exclude_patterns:
    - "**/node_modules/**"
    - "**/.git/**"
    - "**/target/**"
    - "**/dist/**"
    - "**/*.min.js"

  include_patterns:
    - "**/*.py"
    - "**/*.js"
    - "**/*.ts"

  max_file_size_mb: 10
  follow_symlinks: false

# Detector settings
detectors:
  enabled:
    - secrets
    - code_injection
    - command_injection
    - deserialization
    - path_traversal
    - sql_injection
    - ssrf
    - tool_poisoning
    - prompt_injection

  # Per-detector configuration
  secrets:
    confidence_threshold: 0.90

  code_injection:
    skip_comments: true

# Output settings
output:
  format: terminal
  show_code_snippets: true
  max_snippet_lines: 5

# Thresholds
thresholds:
  fail_on: high
  min_confidence: 0.75
  max_risk_score: 75
```

## Tips & Best Practices

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Security Scan
  run: |
    cargo install mcp-sentinel
    mcp-sentinel scan . \
      --format json \
      --output security-report.json \
      --fail-on high
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

mcp-sentinel scan --fail-on critical --quiet
if [ $? -eq 1 ]; then
  echo "❌ Security scan failed - critical issues found!"
  echo "Run 'mcp-sentinel scan .' for details"
  exit 1
fi
```

### Gradual Rollout

```bash
# Step 1: Scan without failing (awareness)
mcp-sentinel scan .

# Step 2: Fail only on critical (high-priority fixes)
mcp-sentinel scan . --fail-on critical

# Step 3: Fail on high+ (production-ready)
mcp-sentinel scan . --fail-on high

# Step 4: Fail on all with whitelist (zero-tolerance)
mcp-sentinel scan . --fail-on low --whitelist whitelist.json
```

### Performance Optimization

```bash
# For large repos (1000+ files)
mcp-sentinel scan . --async --parallel 8

# Skip less critical detectors for speed
mcp-sentinel scan . \
  --detectors secrets,code_injection,sql_injection

# Exclude common non-risky directories
mcp-sentinel scan . \
  --exclude "**/node_modules/**" \
  --exclude "**/vendor/**" \
  --exclude "**/*.min.js"
```

## Getting Help

```bash
# General help
mcp-sentinel --help

# Command-specific help
mcp-sentinel scan --help
mcp-sentinel whitelist --help

# Version information
mcp-sentinel --version

# Online documentation
https://github.com/beejak/MCP_Scanner
```

## Troubleshooting

### Common Issues

**Issue**: "Permission denied" errors
**Solution**: Ensure you have read permissions for the target directory

**Issue**: "No scannable files found"
**Solution**: Check file extensions (.py, .js, .ts, etc.) and exclude patterns

**Issue**: Scan is slow
**Solution**: Use `--async` for large codebases, or `--parallel N` to increase workers

**Issue**: Too many false positives
**Solution**: Use whitelist functionality or increase `--min-confidence`

**Issue**: "Command not found: mcp-sentinel"
**Solution**: Add `~/.cargo/bin` to your PATH

### Debug Mode

```bash
# Enable verbose logging
export RUST_LOG=mcp_sentinel=debug
mcp-sentinel scan . --verbose

# Save debug log to file
export MCP_SENTINEL_LOG_FILE=debug.log
mcp-sentinel scan .
```

---

**Version**: 1.5.0
**Last Updated**: October 2025
**License**: Apache 2.0
