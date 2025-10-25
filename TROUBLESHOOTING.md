# Troubleshooting Guide

Solutions to common issues when using MCP Sentinel.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Scanning Issues](#scanning-issues)
3. [Performance Issues](#performance-issues)
4. [CI/CD Issues](#cicd-issues)
5. [False Positives](#false-positives)
6. [Output and Reporting](#output-and-reporting)
7. [Getting Help](#getting-help)

---

## Installation Issues

### Issue: `cargo build` fails

**Symptoms:**
```
error: failed to compile mcp-sentinel
```

**Solutions:**

1. **Update Rust:**
```bash
rustup update stable
```

2. **Check Rust version:**
```bash
rustc --version  # Should be 1.70+
```

3. **Clean build:**
```bash
cargo clean
cargo build --release
```

4. **Install build dependencies (Linux):**
```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install gcc openssl-devel

# Arch
sudo pacman -S base-devel openssl
```

5. **Install build dependencies (macOS):**
```bash
xcode-select --install
brew install openssl
```

### Issue: Binary not found after installation

**Symptoms:**
```
command not found: mcp-sentinel
```

**Solutions:**

1. **Check if binary exists:**
```bash
ls -l target/release/mcp-sentinel
```

2. **Add to PATH:**
```bash
export PATH="$PATH:$(pwd)/target/release"
```

3. **Or copy to system path:**
```bash
sudo cp target/release/mcp-sentinel /usr/local/bin/
```

4. **Verify installation:**
```bash
which mcp-sentinel
mcp-sentinel --version
```

### Issue: Permission denied

**Symptoms:**
```
Permission denied (os error 13)
```

**Solutions:**

1. **Make binary executable:**
```bash
chmod +x target/release/mcp-sentinel
```

2. **Check file permissions:**
```bash
ls -la target/release/mcp-sentinel
# Should show: -rwxr-xr-x
```

---

## Scanning Issues

### Issue: "No such file or directory"

**Symptoms:**
```
Error: No such file or directory (os error 2)
```

**Solutions:**

1. **Check path exists:**
```bash
ls -la /path/to/scan
```

2. **Use absolute paths:**
```bash
mcp-sentinel scan $(pwd)/my-project
```

3. **Check current directory:**
```bash
pwd
mcp-sentinel scan .
```

### Issue: Files being skipped

**Symptoms:**
```
Scanned 10 files, expected 100
```

**Solutions:**

1. **Check default exclusions:**
```bash
# MCP Sentinel excludes these by default:
# - .git/*
# - node_modules/*
# - target/*
# - __pycache__/*
# - *.pyc
```

2. **Use verbose mode:**
```bash
mcp-sentinel scan . -v
# Shows which files are skipped and why
```

3. **Override exclusions:**
```bash
mcp-sentinel scan . --no-default-excludes
```

4. **Check file permissions:**
```bash
find . -type f ! -readable
```

### Issue: Scan very slow or hangs

**Symptoms:**
```
Scanning... (stuck)
```

**Solutions:**

1. **Check for very large files:**
```bash
find . -type f -size +100M
```

2. **Exclude large directories:**
```bash
mcp-sentinel scan . --exclude "large-data/*" --exclude "videos/*"
```

3. **Reduce parallelism:**
```bash
mcp-sentinel scan . --threads 2
```

4. **Check disk I/O:**
```bash
iostat -x 1
```

5. **Enable timeout:**
```bash
timeout 5m mcp-sentinel scan .
```

### Issue: Detector not finding known vulnerability

**Symptoms:**
```
Known issue not reported
```

**Solutions:**

1. **Check if detector is enabled:**
```bash
mcp-sentinel scan . -v
# Look for "Running detector: <name>"
```

2. **Verify pattern matches:**
```bash
# Check if your code matches expected patterns
# See USER_GUIDE.md for pattern details
```

3. **Try specific detector:**
```bash
mcp-sentinel scan ./file.py --only-detector secrets
```

4. **Check file is being scanned:**
```bash
mcp-sentinel scan ./file.py -v
```

5. **Report false negative:**
```bash
# If it should be detected, file an issue:
# https://github.com/beejak/MCP_Sentinel/issues
```

---

## Performance Issues

### Issue: Scan takes too long

**Symptoms:**
```
Scan duration: 10 minutes for small project
```

**Solutions:**

1. **Use file type filters:**
```bash
mcp-sentinel scan . --include "*.py" --include "*.js"
```

2. **Exclude unnecessary paths:**
```bash
mcp-sentinel scan . \
  --exclude "node_modules/*" \
  --exclude "dist/*" \
  --exclude ".git/*" \
  --exclude "build/*"
```

3. **Increase parallelism:**
```bash
mcp-sentinel scan . --threads 16
```

4. **Scan incrementally:**
```bash
# Only scan changed files
git diff --name-only | xargs mcp-sentinel scan
```

5. **Use caching (if available):**
```bash
mcp-sentinel scan . --cache --cache-dir .mcp-cache
```

### Issue: High memory usage

**Symptoms:**
```
Out of memory error or system slowdown
```

**Solutions:**

1. **Reduce parallelism:**
```bash
mcp-sentinel scan . --threads 1
```

2. **Scan in batches:**
```bash
find . -name "*.py" | xargs -n 10 mcp-sentinel scan
```

3. **Exclude large files:**
```bash
find . -type f -size +10M -name "*.min.js" | \
  xargs -I {} echo "--exclude {}" | \
  xargs mcp-sentinel scan .
```

4. **Monitor memory:**
```bash
# Run scan with memory monitoring
/usr/bin/time -v mcp-sentinel scan .
```

---

## CI/CD Issues

### Issue: CI build fails with mcp-sentinel not found

**Symptoms:**
```
mcp-sentinel: command not found
```

**Solutions:**

1. **Install in CI pipeline:**
```yaml
# GitHub Actions
- name: Install MCP Sentinel
  run: |
    git clone https://github.com/beejak/MCP_Sentinel.git
    cd MCP_Sentinel
    cargo build --release
    sudo cp target/release/mcp-sentinel /usr/local/bin/
```

2. **Use absolute path:**
```yaml
- run: ./target/release/mcp-sentinel scan .
```

3. **Check PATH:**
```yaml
- run: |
    echo $PATH
    which mcp-sentinel || echo "Not in PATH"
```

### Issue: CI cache not working

**Symptoms:**
```
MCP Sentinel rebuilds every time
```

**Solutions:**

**GitHub Actions:**
```yaml
- uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/mcp-sentinel
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-sentinel-${{ hashFiles('**/Cargo.lock') }}
```

**GitLab CI:**
```yaml
cache:
  key: ${CI_COMMIT_REF_SLUG}
  paths:
    - .cargo/
    - target/
```

### Issue: False failures in CI

**Symptoms:**
```
CI fails but local scan passes
```

**Solutions:**

1. **Check for environment differences:**
```bash
# Local
mcp-sentinel scan . --format json --output local.json

# CI - add this step
- run: mcp-sentinel scan . --format json --output ci.json

# Compare
diff local.json ci.json
```

2. **Use same exit code handling:**
```bash
# Local
mcp-sentinel scan . || echo "Exit code: $?"

# CI
mcp-sentinel scan . || exit $?
```

3. **Check file differences:**
```bash
# Ensure CI has all files
git ls-files | wc -l  # vs
find . -type f | wc -l
```

---

## False Positives

### Issue: Legitimate code flagged as vulnerable

**Symptoms:**
```
[SECRETS-001] False positive: Variable name contains "key"
```

**Solutions:**

1. **Use more specific patterns:**
```python
# Instead of generic "key"
api_key_name = "my-service"  # ✅ Not flagged

# vs
api_key = "sk-1234"  # ❌ Flagged (correct)
```

2. **Add comments to suppress:**
```python
password = "test"  # mcp-sentinel-ignore: test credential
```

3. **Whitelist specific findings:**
```bash
# Create .mcp-sentinel-ignore
cat > .mcp-sentinel-ignore << EOF
# Ignore test files
test_*.py:*
*_test.py:*

# Ignore specific line
server.py:42
EOF
```

4. **Report false positive:**
```bash
# If genuinely false, report it:
# https://github.com/beejak/MCP_Sentinel/issues
```

### Issue: Test code being flagged

**Symptoms:**
```
Test fixtures with hardcoded secrets flagged
```

**Solutions:**

1. **Exclude test directories:**
```bash
mcp-sentinel scan . \
  --exclude "tests/*" \
  --exclude "test_*" \
  --exclude "*_test.py"
```

2. **Use environment variables even in tests:**
```python
# tests/test_auth.py
import os

TEST_API_KEY = os.getenv("TEST_API_KEY", "test-key-for-ci")
```

3. **Mark as test data:**
```python
# Test data - not real credentials
TEST_PASSWORD = "password123"  # mcp-sentinel-ignore
```

---

## Output and Reporting

### Issue: No output displayed

**Symptoms:**
```
(scan completes with no output)
```

**Solutions:**

1. **Check exit code:**
```bash
mcp-sentinel scan .
echo "Exit code: $?"
# 0 = no issues found
```

2. **Use verbose mode:**
```bash
mcp-sentinel scan . -v
```

3. **Check if files were scanned:**
```bash
mcp-sentinel scan . --format json | jq '.summary.total_files'
```

4. **Verify you're scanning right path:**
```bash
pwd
ls -la
mcp-sentinel scan .
```

### Issue: JSON output malformed

**Symptoms:**
```
jq: parse error
```

**Solutions:**

1. **Check file was created:**
```bash
ls -lh results.json
cat results.json | jq '.' # Validate JSON
```

2. **Redirect stderr separately:**
```bash
mcp-sentinel scan . --format json --output results.json 2> errors.log
```

3. **Check for mixed output:**
```bash
# Don't mix formats
mcp-sentinel scan . --format json > results.json
# Not:
mcp-sentinel scan . -v --format json > results.json  # -v adds text
```

### Issue: SARIF upload fails

**Symptoms:**
```
Error uploading SARIF to GitHub
```

**Solutions:**

1. **Validate SARIF format:**
```bash
# Install validator
npm install -g @microsoft/sarif-validator

# Validate
sarif-validator results.sarif
```

2. **Check file size:**
```bash
# GitHub limits: 10 MB per file
ls -lh results.sarif
```

3. **Check permissions:**
```yaml
# GitHub Actions
permissions:
  security-events: write
  contents: read
```

---

## Getting Help

### Debugging Steps

1. **Enable verbose output:**
```bash
mcp-sentinel scan . -v 2>&1 | tee debug.log
```

2. **Check version:**
```bash
mcp-sentinel --version
```

3. **Verify installation:**
```bash
which mcp-sentinel
file $(which mcp-sentinel)
ldd $(which mcp-sentinel)  # Linux
otool -L $(which mcp-sentinel)  # macOS
```

4. **Test with known vulnerable file:**
```bash
echo 'API_KEY = "sk-1234567890"' > test.py
mcp-sentinel scan test.py
# Should find SECRETS-001
```

5. **Collect system information:**
```bash
uname -a
rustc --version
mcp-sentinel --version
```

### Reporting Issues

When reporting issues, include:

1. **Version information:**
```bash
mcp-sentinel --version
```

2. **Command used:**
```bash
mcp-sentinel scan . --min-severity high --format json
```

3. **Expected vs actual behavior**

4. **Minimal reproduction:**
```bash
# Create minimal test case
cat > minimal.py << EOF
# Code that reproduces issue
EOF

mcp-sentinel scan minimal.py
```

5. **System information:**
```bash
uname -a
rustc --version
```

### Support Channels

- **GitHub Issues**: https://github.com/beejak/MCP_Sentinel/issues
- **Discussions**: https://github.com/beejak/MCP_Sentinel/discussions
- **Documentation**: See README.md and docs/

### Common Error Messages

| Error | Meaning | Solution |
|-------|---------|----------|
| `No such file or directory` | Path doesn't exist | Check path with `ls` |
| `Permission denied` | Insufficient permissions | Check with `ls -la`, use `chmod` |
| `command not found` | Not in PATH | Add to PATH or use absolute path |
| `failed to compile` | Build error | Update Rust, install dependencies |
| `Scan failed` | Internal error | Check verbose output, report issue |

---

**Still having issues?**

1. Check [FAQ](FAQ.md) for common questions
2. Review [User Guide](USER_GUIDE.md) for usage details
3. See [Examples](EXAMPLES.md) for working code
4. Open an issue on GitHub

**We're here to help!** 🛡️
