# MCP Sentinel Architecture

This document describes the technical architecture of MCP Sentinel.

## System Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         MCP Sentinel CLI                            │
│                     (Command Line Interface)                        │
└────────────────┬────────────────────────────────────────────────────┘
                 │
                 ├── scan (Phase 1 ✅)
                 ├── proxy (Phase 3)
                 ├── monitor (Phase 3)
                 ├── audit (Phase 2)
                 ├── init (Phase 2)
                 ├── whitelist (Phase 4)
                 └── rules (Phase 2)

                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        Scanner Orchestrator                         │
│              (Coordinates file discovery and analysis)              │
└───┬─────────────────────────────────────────────────────────────┬───┘
    │                                                             │
    ▼                                                             ▼
┌────────────────────────────┐                    ┌──────────────────────────┐
│   File Discovery Engine    │                    │    Configuration         │
│  - Glob pattern matching   │                    │  - Scan settings         │
│  - .gitignore support      │                    │  - Exclusion patterns    │
│  - File type filtering     │                    │  - Thresholds            │
└────────────┬───────────────┘                    └──────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Detector Pipeline                              │
│              (Sequential execution of all detectors)                │
└─┬──────────┬──────────┬─────────────┬──────────────┬───────────────┘
  │          │          │             │              │
  ▼          ▼          ▼             ▼              ▼
┌────┐    ┌────┐    ┌────┐       ┌────┐        ┌────┐
│ 1  │    │ 2  │    │ 3  │       │ 4  │        │ 5  │
└────┘    └────┘    └────┘       └────┘        └────┘
Secrets   Command  Sensitive    Tool         Prompt
          Injection File Access Poisoning    Injection

             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Vulnerability Aggregation                        │
│       - Collect results from all detectors                          │
│       - Deduplicate similar findings                                │
│       - Calculate risk scores                                       │
│       - Group by severity/type                                      │
└────────────────┬────────────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Output Formatters                              │
│                                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │   Terminal   │  │     JSON     │  │    SARIF     │             │
│  │   (Phase 1)  │  │  (Phase 1)   │  │  (Phase 4)   │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
│                                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │     HTML     │  │     PDF      │  │   Markdown   │             │
│  │  (Phase 2)   │  │  (Phase 4)   │  │  (Phase 2)   │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. CLI Layer (`src/main.rs`, `src/cli/`)

**Responsibility**: Parse commands, validate arguments, initialize logging

**Technology**: `clap` v4 with derive macros

**Commands**:
- `scan` - Scan a directory for vulnerabilities (Phase 1 ✅)
- `proxy` - Runtime monitoring proxy (Phase 3)
- `monitor` - Real-time dashboard (Phase 3)
- `audit` - Deep code analysis (Phase 2)
- `init` - Create configuration file (Phase 2)
- `whitelist` - Manage whitelisted patterns (Phase 4)
- `rules` - Custom detection rules (Phase 2)

### 2. Scanner Orchestrator (`src/scanner.rs`)

**Responsibility**: Coordinate file discovery and detector execution

**Flow**:
1. Validate target directory exists
2. Discover scannable files
3. For each file:
   - Read content (skip if binary/unreadable)
   - Run all detectors
   - Collect vulnerabilities
4. Aggregate results
5. Calculate risk score
6. Return scan result

**Error Handling**:
- Graceful degradation (continue on detector failures)
- Proper logging at appropriate levels
- Context-rich error messages

### 3. File Discovery (`src/utils/file.rs`)

**Responsibility**: Find files to scan, respecting exclusion patterns

**Features**:
- File type filtering (`.py`, `.js`, `.ts`, `.jsx`, `.tsx`, `.json`, `.yaml`)
- Exclusion patterns (`.gitignore` style)
- Default exclusions: `node_modules/`, `.git/`, `target/`, `dist/`, `build/`
- Symlink following (with cycle detection planned)

### 4. Detector Pipeline

Each detector is independent and implements the same interface:

```rust
pub fn detect(content: &str, file_path: &str) -> Result<Vec<Vulnerability>>
```

#### Detector 1: Secrets (`src/detectors/secrets.rs`)
- **Patterns**: 15+ regex patterns
- **Types**: AWS keys, API keys, JWT tokens, private keys, connection strings
- **Features**: Secret redaction, high confidence scoring
- **Performance**: ~1ms per file

#### Detector 2: Command Injection (`src/detectors/code_vulns.rs`)
- **Languages**: Python, JavaScript, TypeScript
- **Patterns**: `os.system()`, `subprocess.run()`, `child_process.exec()`
- **Context**: Checks for user input in commands

#### Detector 3: Sensitive File Access (`src/detectors/code_vulns.rs`)
- **Targets**: SSH keys, AWS credentials, `.env` files, browser cookies
- **Patterns**: File paths and access patterns
- **Risk**: Data exfiltration potential

#### Detector 4: Tool Poisoning (`src/detectors/tool_poisoning.rs`)
- **Checks**: Invisible Unicode, malicious keywords
- **MCP-Specific**: Tool description manipulation
- **Detection**: Zero-width characters, direction overrides

#### Detector 5: Prompt Injection (`src/detectors/prompt_injection.rs`)
- **Patterns**: Role manipulation, system prompt references
- **Keywords**: Jailbreak attempts, instruction overrides
- **Context**: LLM-specific attack vectors

### 5. Data Models (`src/models/`)

#### Vulnerability (`vulnerability.rs`)
```rust
pub struct Vulnerability {
    id: String,
    vuln_type: VulnerabilityType,
    severity: Severity,
    confidence: f32,
    location: Option<Location>,
    title: String,
    description: String,
    impact: Option<String>,
    remediation: Option<String>,
    code_snippet: Option<String>,
    evidence: Option<HashMap<String, Value>>,
}
```

#### Scan Result (`scan_result.rs`)
```rust
pub struct ScanResult {
    scan_id: String,
    timestamp: DateTime<Utc>,
    target: String,
    summary: ScanSummary,
    vulnerabilities: Vec<Vulnerability>,
    metadata: ScanMetadata,
}
```

**Risk Scoring Formula**:
```
risk_score = min(100, critical * 40 + high * 20 + medium * 5 + low)
```

### 6. Output Formatters (`src/output/`)

#### Terminal Formatter (`terminal.rs`)
- Colored output with `crossterm`
- Emoji severity indicators
- Hierarchical grouping (by severity → by type)
- Risk score visualization
- Summary statistics

#### JSON Formatter (`json.rs`)
- Machine-readable output
- Full vulnerability details
- Timestamp and metadata
- Schema-versioned

### 7. Configuration (`src/models/config.rs`)

```rust
pub struct ScanConfig {
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub max_file_size_bytes: usize,
    pub follow_symlinks: bool,
}
```

## Data Flow

```
┌──────────┐      ┌──────────┐      ┌──────────┐      ┌──────────┐
│   User   │─────▶│   CLI    │─────▶│ Scanner  │─────▶│   File   │
│  Input   │      │  Parser  │      │ Orchestr.│      │Discovery │
└──────────┘      └──────────┘      └──────────┘      └──────────┘
                                           │
                                           ▼
                  ┌────────────────────────────────────────────┐
                  │          Detector Pipeline                 │
                  │  (Each detector processes independently)   │
                  └────────────────────────────────────────────┘
                                           │
                                           ▼
                  ┌────────────────────────────────────────────┐
                  │      Vulnerability Aggregation             │
                  │  - Collect all vulnerabilities             │
                  │  - Calculate risk scores                   │
                  │  - Generate summary                        │
                  └────────────────────────────────────────────┘
                                           │
                                           ▼
                  ┌────────────────────────────────────────────┐
                  │       Output Formatting                    │
                  │  - Terminal (colored, structured)          │
                  │  - JSON (machine-readable)                 │
                  └────────────────────────────────────────────┘
                                           │
                                           ▼
                  ┌────────────────────────────────────────────┐
                  │         User/CI System                     │
                  │  - Review vulnerabilities                  │
                  │  - Take remediation actions                │
                  │  - Track over time                         │
                  └────────────────────────────────────────────┘
```

## Error Handling Strategy

### Error Categories

1. **User Input Errors** (Exit code: 2)
   - Invalid path
   - Missing arguments
   - Malformed options

2. **File System Errors** (Graceful degradation)
   - Permission denied → Skip file (debug log)
   - Binary files → Skip file (debug log)
   - Invalid UTF-8 → Skip file (debug log)

3. **Detector Errors** (Graceful degradation)
   - Regex timeout → Log warning, continue
   - Out of memory → Log warning, skip file
   - Unexpected error → Log warning, continue

4. **Output Errors** (Exit code: 2)
   - Cannot write file → Error
   - JSON serialization → Error

### Logging Levels

```
ERROR:   Critical failures that stop execution
         Examples: Can't read directory, can't write output

WARN:    Non-critical issues that may affect results
         Examples: Detector failed, file skipped

INFO:    Progress updates and high-level results
         Examples: "Scanning 142 files", "Found 5 issues"

DEBUG:   Detailed information for troubleshooting
         Examples: "Running detector X", "Skipping binary file"
```

## Performance Characteristics

### Phase 1 (Current)

| Operation | Target Performance |
|-----------|-------------------|
| Small repo (<100 files) | <2 seconds |
| Medium repo (100-1000 files) | <10 seconds |
| Large repo (>1000 files) | <30 seconds |
| Memory usage | <100 MB |
| Binary size (release) | <20 MB |

### Optimization Strategies

1. **Lazy Pattern Compilation**: Regex patterns compiled once at startup
2. **Sequential Scanning**: Simple, predictable, no race conditions
3. **Early Skipping**: Binary/large files detected and skipped early
4. **Minimal Allocations**: Reuse buffers where possible

### Phase 2+ Optimizations (Planned)

- Parallel file scanning with `rayon`
- Incremental scanning (cache previous results)
- Smart file filtering (AST-based type detection)
- Memory-mapped file reading for large files

## Security Considerations

### 1. Secret Redaction

Detected secrets are **never** shown in full:
- Terminal output: Redacted (e.g., `AKIA...MPLE`)
- JSON output: Redacted
- Logs: Redacted

### 2. Sandboxing

- No code execution (static analysis only)
- No network requests
- No file modifications
- Read-only operations

### 3. Dependencies

- Minimal dependency tree
- Regular `cargo audit` checks
- Pin versions in `Cargo.lock`
- Review security advisories

## Testing Strategy

### Unit Tests
- Located in same file as code (`#[cfg(test)]`)
- Test individual detectors
- Test data model behavior
- Test utility functions

### Integration Tests
- Located in `tests/` directory
- Test end-to-end scanning
- Test output formats
- Test error handling

### Test Fixtures
- `tests/fixtures/vulnerable_servers/` contains intentionally vulnerable code
- Used for integration tests
- Covers all detector types

## Future Architecture (Phase 2-4)

### Phase 2: Enhanced Static Analysis

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Tree-sitter Parser                               │
│              (AST-based code understanding)                         │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Semgrep Engine                                 │
│           (Advanced pattern matching on AST)                        │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      AI Analysis Engine                             │
│        (OpenAI/Anthropic/Ollama for complex analysis)               │
└─────────────────────────────────────────────────────────────────────┘
```

### Phase 3: Runtime Monitoring

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Proxy Interceptor                               │
│         (Intercepts MCP protocol messages)                          │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Behavior Analyzer                                │
│           (Detects anomalous runtime behavior)                      │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Guardrails Engine                                │
│              (Enforce security policies)                            │
└─────────────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core
- **Language**: Rust 1.70+
- **Runtime**: Tokio (async)
- **CLI**: clap v4
- **Error Handling**: anyhow
- **Logging**: tracing + tracing-subscriber

### Detectors
- **Pattern Matching**: regex v1.x
- **Lazy Statics**: once_cell
- **JSON**: serde + serde_json

### Output
- **Terminal Colors**: crossterm
- **Timestamps**: chrono
- **File Operations**: std::fs + walkdir

### Development
- **Testing**: Built-in Rust testing
- **Benchmarking**: criterion (planned)
- **Documentation**: rustdoc

## References

- [MCP Specification](https://modelcontextprotocol.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Database](https://cwe.mitre.org/)
