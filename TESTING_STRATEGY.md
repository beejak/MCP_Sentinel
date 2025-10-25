# MCP Sentinel Testing Strategy

Comprehensive testing strategy for current version (v1.5.0) and future releases.

## Current Testing Status (v1.5.0)

### Phase 1.5 Test Coverage Goals

**Target Coverage**: 80%+

**Current Implementation** (To be completed):
- Unit tests: 80+ test cases planned
- Integration tests: 15+ scenarios
- Property-based tests: 5+ properties
- Performance benchmarks: 8+ benchmarks

## Unit Testing Strategy

### Test Organization

```
tests/
├── unit/
│   ├── detectors/
│   │   ├── test_secrets.rs              (20+ tests)
│   │   ├── test_command_injection.rs     (15+ tests)
│   │   ├── test_code_injection.rs        (20+ tests - NEW v1.5)
│   │   ├── test_deserialization.rs       (15+ tests - NEW v1.5)
│   │   ├── test_path_traversal.rs        (12+ tests - NEW v1.5)
│   │   ├── test_sql_injection.rs         (15+ tests - NEW v1.5)
│   │   ├── test_ssrf.rs                  (10+ tests - NEW v1.5)
│   │   ├── test_tool_poisoning.rs        (10+ tests)
│   │   └── test_prompt_injection.rs      (8+ tests)
│   ├── models/
│   │   ├── test_vulnerability.rs
│   │   ├── test_scan_result.rs
│   │   ├── test_whitelist.rs             (NEW v1.5)
│   │   └── test_config.rs
│   ├── output/
│   │   ├── test_terminal.rs
│   │   ├── test_json.rs
│   │   ├── test_csv.rs                   (NEW v1.5)
│   │   └── test_html.rs                  (NEW v1.5)
│   └── utils/
│       ├── test_file.rs
│       └── test_hash.rs                  (NEW v1.5)
```

### Test Case Categories

#### 1. Positive Tests (Vulnerability Detection)

```rust
#[test]
fn test_detect_aws_key_various_formats() {
    // Test AWS key in variable assignment
    // Test AWS key in string literal
    // Test AWS key in f-string
    // Test AWS key with different quote styles
    // Test AWS key with whitespace variations
}

#[test]
fn test_detect_eval_with_obfuscation() {
    // Test direct eval()
    // Test eval via getattr
    // Test eval in lambda
    // Test base64 encoded eval pattern
}
```

#### 2. Negative Tests (No False Positives)

```rust
#[test]
fn test_no_false_positive_safe_patterns() {
    let content = r#"
        # Safe code - should NOT trigger
        data = json.loads(user_input)
        result = calculate_hash(value)
        db.query("SELECT * FROM users WHERE id = ?", [user_id])
    "#;

    let vulns = detect(content, "test.py").unwrap();
    assert!(vulns.is_empty(), "Safe patterns should not be detected");
}
```

#### 3. Edge Case Tests

```rust
#[test]
fn test_edge_cases() {
    // Empty file
    // Very long lines (>10,000 chars)
    // Binary data
    // Invalid UTF-8
    // Files with only comments
    // Minified code
}
```

#### 4. Language-Specific Tests

```rust
#[test]
fn test_python_specific_patterns() {
    // Python 2 vs Python 3
    // Different string formatting (%, .format(), f-strings)
    // Type hints
}

#[test]
fn test_javascript_typescript_patterns() {
    // ES5 vs ES6+ syntax
    // TypeScript type annotations
    // JSX code
}
```

## Integration Testing Strategy

### End-to-End Scenarios

```rust
tests/
└── integration/
    ├── test_full_scan_workflow.rs
    ├── test_whitelist_integration.rs      (NEW v1.5)
    ├── test_output_formats.rs             (NEW v1.5)
    ├── test_async_jobs.rs                 (NEW v1.5)
    ├── test_policy_enforcement.rs         (NEW v1.5)
    ├── test_cli_commands.rs
    └── test_real_mcp_servers.rs
```

### Test Scenarios

#### Scenario 1: Full Scan Workflow

```rust
#[tokio::test]
async fn test_complete_scan_workflow() {
    // 1. Create test project with known vulnerabilities
    // 2. Run scan with all detectors
    // 3. Verify correct number and types of findings
    // 4. Test different output formats
    // 5. Verify exit codes
}
```

#### Scenario 2: Whitelist Workflow

```rust
#[tokio::test]
async fn test_whitelist_reduces_findings() {
    // 1. Scan and get baseline findings
    // 2. Add some findings to whitelist
    // 3. Rescan with whitelist
    // 4. Verify whitelisted findings not reported
    // 5. Verify non-whitelisted findings still reported
}
```

#### Scenario 3: Real MCP Server Testing

```rust
#[tokio::test]
async fn test_scan_real_mcp_servers() {
    // Test against:
    // - filesystem MCP server
    // - time MCP server
    // - github MCP server
    // Verify appropriate findings for each
}
```

## Property-Based Testing

Using `proptest` crate for property-based testing.

### Properties to Test

#### Property 1: Secret Redaction Never Leaks

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_secret_redaction_never_leaks_full_secret(
        secret in "[A-Z0-9]{32}"  // Generate random 32-char secrets
    ) {
        let redacted = redact_secret(&secret);
        // Property: Redacted version should never contain full secret
        assert!(!redacted.contains(&secret));
        // Property: Redacted version should be shorter or equal
        assert!(redacted.len() <= secret.len());
    }
}
```

#### Property 2: Risk Score Consistency

```rust
proptest! {
    #[test]
    fn test_risk_score_monotonic(
        critical in 0u32..10,
        high in 0u32..20,
        medium in 0u32..50
    ) {
        let score1 = calculate_risk_score(critical, high, medium, 0);
        let score2 = calculate_risk_score(critical + 1, high, medium, 0);

        // Property: More critical issues = higher risk score
        assert!(score2 >= score1);

        // Property: Risk score capped at 100
        assert!(score1 <= 100);
        assert!(score2 <= 100);
    }
}
```

#### Property 3: File Discovery Respects Exclusions

```rust
proptest! {
    #[test]
    fn test_exclusion_patterns_work(
        pattern in "\\*\\*/[a-z]+/\\*\\*"  // Generate random patterns
    ) {
        let files = discover_files_with_exclusion("/test", &[pattern]);

        // Property: No discovered files should match excluded pattern
        for file in files {
            assert!(!file.to_string_lossy().contains(&pattern));
        }
    }
}
```

## Fuzzing Strategy

Using `cargo-fuzz` for fuzzing critical code paths.

### Fuzz Targets

```rust
fuzz_targets/
├── fuzz_regex_patterns.rs      // Test regex DoS resistance
├── fuzz_file_parsing.rs        // Test file parsing robustness
├── fuzz_json_parsing.rs        // Test JSON parsing
└── fuzz_cli_args.rs            // Test CLI argument handling
```

### Fuzzing Goals

- **No Panics**: Code should never panic on malformed input
- **No ReDoS**: Regex patterns should complete in <100ms
- **Memory Safety**: No buffer overflows or memory leaks
- **Graceful Degradation**: Return errors, don't crash

### Example Fuzz Target

```rust
// fuzz_targets/fuzz_regex_patterns.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[str]| {
    if let Ok(content) = std::str::from_utf8(data) {
        // Fuzz all detectors with random input
        let _ = mcp_sentinel::detectors::secrets::detect(content, "fuzz.txt");
        let _ = mcp_sentinel::detectors::code_injection::detect(content, "fuzz.txt");
        // Should not panic regardless of input
    }
});
```

## Performance Benchmarking

Using `criterion` crate for benchmarking.

### Benchmark Suites

```rust
benches/
├── detector_performance.rs       // Individual detector speed
├── file_discovery_performance.rs // File discovery speed
├── output_formatting.rs          // Output generation speed
└── full_scan_scenarios.rs        // End-to-end scan performance
```

### Performance Targets (v1.5.0)

| Operation | Target | Measured |
|-----------|--------|----------|
| Secrets detector | <1ms per file | TBD |
| Code injection detector | <2ms per file | TBD |
| File discovery (1000 files) | <100ms | TBD |
| Small repo scan (<100 files) | <2s | TBD |
| Medium repo scan (100-1000 files) | <10s | TBD |
| Large repo scan (1000-10000 files) | <60s | TBD |
| JSON output generation | <50ms | TBD |
| HTML report generation | <200ms | TBD |

### Example Benchmark

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_secrets_detector(c: &mut Criterion) {
    let content = include_str!("../tests/fixtures/sample_file.py");

    c.bench_function("secrets_detector", |b| {
        b.iter(|| {
            mcp_sentinel::detectors::secrets::detect(
                black_box(content),
                black_box("test.py")
            )
        })
    });
}

criterion_group!(benches, bench_secrets_detector);
criterion_main!(benches);
```

## Security Testing

### Static Analysis

```bash
# Clippy with all warnings as errors
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit

# Dependency vulnerability check
cargo deny check
```

### ReDoS Prevention

```rust
#[test]
fn test_regex_no_redos() {
    let malicious_input = "A".repeat(10000) + "!";

    let start = std::time::Instant::now();
    let _ = SECRET_PATTERNS[0].regex.is_match(&malicious_input);
    let duration = start.elapsed();

    // Regex should complete in <100ms even with pathological input
    assert!(duration.as_millis() < 100, "Potential ReDoS vulnerability");
}
```

### Path Traversal in Tool Itself

```rust
#[test]
fn test_tool_not_vulnerable_to_path_traversal() {
    // Test that our file operations are safe
    let result = discover_files("../../etc/passwd", &[]);
    assert!(result.is_err(), "Should not traverse outside project");
}
```

## Test Data Management

### Test Fixtures

```
tests/fixtures/
├── vulnerable_servers/
│   ├── test-server/              (Existing)
│   ├── python-mcp-server/        (NEW - real Python MCP)
│   ├── typescript-mcp-server/    (NEW - real TS MCP)
│   └── vulnerable-patterns/      (NEW - edge cases)
├── safe_servers/
│   ├── well-secured-server/      (NEW - no vulnerabilities)
│   └── best-practices-server/    (NEW - security best practices)
├── edge_cases/
│   ├── minified-code/
│   ├── obfuscated-code/
│   ├── unicode-edge-cases/
│   └── large-files/
└── config_files/
    ├── valid-configs/
    └── malicious-configs/
```

### Golden File Testing

```rust
#[test]
fn test_output_matches_golden_file() {
    let result = scan_directory("tests/fixtures/test-server");
    let actual_json = serde_json::to_string_pretty(&result).unwrap();

    let golden = fs::read_to_string("tests/golden/test-server-output.json").unwrap();

    assert_eq!(actual_json, golden, "Output differs from golden file");
}
```

## Continuous Integration Testing

### GitHub Actions Workflow

```yaml
name: Tests

on: [push, pull_request]

jobs:
  unit-tests:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@${{ matrix.rust }}
      - run: cargo test --all-features

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --test integration_*

  property-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --features proptest

  benchmarks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo bench --no-fail-fast

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3
```

## Test Automation

### Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Running tests..."
cargo test --quiet || exit 1

echo "Running clippy..."
cargo clippy --all-targets -- -D warnings || exit 1

echo "Checking formatting..."
cargo fmt -- --check || exit 1

echo "All checks passed!"
```

### Test Watchers

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch for changes and run tests
cargo watch -x test

# Watch and run specific test
cargo watch -x "test test_secrets_detector"
```

## Future Testing Enhancements (v2.0+)

### Mutation Testing

```bash
# Install cargo-mutants
cargo install cargo-mutants

# Run mutation testing
cargo mutants

# Goal: >80% mutation score
```

**What is Mutation Testing?**
Mutants are created by modifying code (e.g., changing `==` to `!=`). Tests should fail for mutants, proving tests are effective.

### Chaos Engineering (Runtime Proxy)

For Phase 3's runtime proxy:

- Inject network failures
- Simulate high latency
- Test connection drops
- Memory pressure testing
- CPU throttling

### AI-Powered Test Generation (v2.0)

```rust
// Use AI to generate test cases
#[test]
fn test_ai_generated_edge_cases() {
    // Use GPT-4 to generate edge cases for detectors
    // Validate that detector correctly handles them
}
```

### Compliance Testing

Test against security standards:

- OWASP Top 10
- CWE Top 25
- SANS Top 25
- NIST guidelines

### Load Testing

```rust
#[test]
fn test_scan_10000_files() {
    // Verify scanner can handle large codebases
    // Check memory usage stays < 500MB
    // Verify no memory leaks
}
```

## Test Metrics and Reporting

### Coverage Goals

| Component | Target Coverage |
|-----------|-----------------|
| Detectors | 90%+ |
| Models | 85%+ |
| Output formatters | 80%+ |
| CLI | 75%+ |
| Utils | 85%+ |
| **Overall** | **80%+** |

### Quality Gates

Before merging PRs, all must pass:

- ✅ All tests pass
- ✅ Code coverage >= 80%
- ✅ No clippy warnings
- ✅ Formatted with rustfmt
- ✅ No security vulnerabilities (cargo audit)
- ✅ Performance benchmarks don't regress >10%

## Conclusion

This comprehensive testing strategy ensures MCP Sentinel is:

1. **Reliable**: Catches vulnerabilities consistently
2. **Robust**: Handles edge cases and malformed input
3. **Performant**: Meets speed requirements
4. **Secure**: The security tool itself is secure
5. **Maintainable**: Tests serve as documentation

**Next Steps for v1.5.0**: Implement the 80+ unit tests planned above.

**Next Steps for v2.0**: Add mutation testing, chaos engineering, and AI-powered test generation.

---

**Version**: 1.5.0
**Last Updated**: October 2025
