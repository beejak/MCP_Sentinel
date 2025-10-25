# Documentation Automation System

Automated documentation generation and maintenance system for MCP Sentinel.

## Table of Contents

1. [Overview](#overview)
2. [Documentation Philosophy](#documentation-philosophy)
3. [Automated Workflows](#automated-workflows)
4. [Templates System](#templates-system)
5. [Doc Generation Scripts](#doc-generation-scripts)
6. [Feature Development Process](#feature-development-process)
7. [CI/CD Integration](#cicd-integration)
8. [Maintenance](#maintenance)

---

## Overview

MCP Sentinel uses an automated documentation system to ensure that **every new feature comes with complete, high-quality documentation**. This system eliminates documentation debt and maintains our world-class documentation standards.

### Goals

- ✅ **Zero documentation debt** - Docs created alongside code
- ✅ **Consistency** - All docs follow same structure and quality
- ✅ **Completeness** - Every feature fully documented
- ✅ **Maintainability** - Easy to update as features evolve
- ✅ **Automation** - Minimal manual work required

### Components

```
MCP_Sentinel/
├── .doc-templates/          # Documentation templates
│   ├── FEATURE.md.template
│   ├── DETECTOR.md.template
│   ├── COMMAND.md.template
│   └── INTEGRATION.md.template
├── scripts/docs/            # Automation scripts
│   ├── generate-doc.sh      # Generate docs from templates
│   ├── validate-docs.sh     # Validate documentation completeness
│   ├── update-all-docs.sh   # Update all docs with new feature
│   └── doc-stats.sh         # Generate documentation statistics
├── .github/workflows/
│   └── docs-validation.yml  # CI/CD doc validation
└── FEATURE_CHECKLIST.md     # Checklist for new features
```

---

## Documentation Philosophy

### The Rule: Feature = Code + Docs

**Every feature must include:**

1. **Code implementation** (obviously!)
2. **User-facing documentation** (how to use it)
3. **Examples** (real-world usage)
4. **Integration guide** (how it fits in workflows)
5. **Troubleshooting** (common issues)
6. **API/CLI reference** (technical details)
7. **Tests** (ensure it works)
8. **Changelog entry** (what changed)

### Quality Standards

**All documentation must:**

- ✅ Be written **before or during** feature development (not after)
- ✅ Include **code examples** that actually work
- ✅ Have **step-by-step instructions** for common use cases
- ✅ Provide **troubleshooting** for predictable issues
- ✅ Be **reviewed** by at least one other person
- ✅ Pass **automated validation** checks
- ✅ Be **consistent** with existing documentation style

### Documentation Types

| Type | Purpose | Example |
|------|---------|---------|
| **User Guide** | How to use features | "Scanning with custom rules" |
| **Examples** | Real-world code samples | "Scanning a Python MCP server" |
| **API Reference** | Technical specifications | "CLI command options" |
| **Integration** | Connecting with other tools | "GitHub Actions integration" |
| **Troubleshooting** | Solving common problems | "Scan fails with error X" |
| **Best Practices** | Recommended usage patterns | "Security scanning workflows" |

---

## Automated Workflows

### Workflow 1: New Feature Documentation

**Trigger:** Developer starts working on a new feature

**Process:**

```bash
# 1. Create feature branch
git checkout -b feature/new-detector

# 2. Generate documentation template
./scripts/docs/generate-doc.sh --type detector --name "xss-detection"

# Generated files:
# - .doc-templates/detector-xss-detection.md (draft)
# - EXAMPLES.md (section added)
# - USER_GUIDE.md (section added)
# - TROUBLESHOOTING.md (section added)

# 3. Implement feature + fill in documentation
# (Developer works on both code and docs)

# 4. Run doc validation
./scripts/docs/validate-docs.sh

# 5. Update all related docs
./scripts/docs/update-all-docs.sh --feature "XSS Detection"

# 6. Commit everything together
git add -A
git commit -m "feat: Add XSS detection detector

- Implement XSS detection with 15+ patterns
- Add comprehensive documentation
- Include real-world examples
- Update all user guides"
```

### Workflow 2: Feature Enhancement Documentation

**Trigger:** Enhancing an existing feature

**Process:**

```bash
# 1. Update implementation
vim src/detectors/secrets.rs

# 2. Update corresponding documentation sections
./scripts/docs/update-all-docs.sh --feature "Secrets Detection" --update

# This automatically:
# - Finds all references to "Secrets Detection"
# - Opens relevant files for editing
# - Updates version numbers
# - Updates examples if needed
# - Regenerates statistics

# 3. Run validation
./scripts/docs/validate-docs.sh --feature "Secrets Detection"

# 4. Commit
git commit -am "feat: Add GitHub fine-grained token detection to secrets detector"
```

### Workflow 3: CI/CD Doc Validation

**Trigger:** Pull request opened

**Process:**

```yaml
# .github/workflows/docs-validation.yml runs automatically

Checks:
✓ All new features have documentation
✓ Code examples compile/run
✓ No broken links
✓ Consistent formatting
✓ Changelog updated
✓ Version numbers match
✓ Examples work with current code
```

**If validation fails:** PR is blocked until docs are fixed.

---

## Templates System

### Template Structure

Each template includes:

1. **Frontmatter** (metadata)
2. **Required sections** (enforced by validation)
3. **Placeholder text** (guidance for authors)
4. **Example code** (copy-paste ready)
5. **Checklist** (ensure completeness)

### Available Templates

#### 1. Detector Template

**File:** `.doc-templates/DETECTOR.md.template`

**Use:** New vulnerability detector

**Sections:**
- Overview (what it detects)
- Patterns (regex/rules)
- Examples (vulnerable code → fixed code)
- Configuration (how to enable/disable)
- False positives (known issues)
- CWE mapping (compliance)

**Generate:**
```bash
./scripts/docs/generate-doc.sh --type detector --name "xss-detection"
```

#### 2. Feature Template

**File:** `.doc-templates/FEATURE.md.template`

**Use:** New major feature (not a detector)

**Sections:**
- Overview
- Use cases
- Step-by-step guide
- Configuration
- Integration examples
- API reference
- Troubleshooting

**Generate:**
```bash
./scripts/docs/generate-doc.sh --type feature --name "real-time-monitoring"
```

#### 3. Command Template

**File:** `.doc-templates/COMMAND.md.template`

**Use:** New CLI command

**Sections:**
- Synopsis
- Description
- Options/flags
- Examples
- Exit codes
- Related commands

**Generate:**
```bash
./scripts/docs/generate-doc.sh --type command --name "report"
```

#### 4. Integration Template

**File:** `.doc-templates/INTEGRATION.md.template`

**Use:** Integration with external tool/platform

**Sections:**
- Overview
- Prerequisites
- Setup instructions
- Configuration
- Usage examples
- Troubleshooting
- Best practices

**Generate:**
```bash
./scripts/docs/generate-doc.sh --type integration --name "datadog"
```

---

## Doc Generation Scripts

### 1. generate-doc.sh

**Purpose:** Generate documentation from templates

**Usage:**
```bash
./scripts/docs/generate-doc.sh [OPTIONS]

Options:
  --type <detector|feature|command|integration>
  --name <feature-name>
  --version <version>
  --author <name>
  --output <path>

Examples:
  # Generate detector documentation
  ./scripts/docs/generate-doc.sh --type detector --name "csrf-detection"

  # Generate feature documentation
  ./scripts/docs/generate-doc.sh --type feature --name "ai-analysis" --version "2.0.0"

  # Generate integration documentation
  ./scripts/docs/generate-doc.sh --type integration --name "slack"
```

**What it does:**
1. Copies template to working directory
2. Replaces placeholders (name, date, version)
3. Creates sections in existing docs
4. Opens files in editor for completion
5. Adds to doc index

### 2. validate-docs.sh

**Purpose:** Validate documentation completeness and quality

**Usage:**
```bash
./scripts/docs/validate-docs.sh [OPTIONS]

Options:
  --feature <name>      # Validate specific feature
  --all                 # Validate all documentation
  --strict              # Strict mode (fail on warnings)
  --fix                 # Auto-fix simple issues

Examples:
  # Validate all docs
  ./scripts/docs/validate-docs.sh --all

  # Validate specific feature
  ./scripts/docs/validate-docs.sh --feature "XSS Detection"

  # Strict validation for CI/CD
  ./scripts/docs/validate-docs.sh --all --strict
```

**Checks:**
- [ ] Required sections present
- [ ] Code examples present
- [ ] No broken links
- [ ] No TODO/FIXME markers
- [ ] Version numbers consistent
- [ ] Changelog updated
- [ ] Examples directory has matching files
- [ ] Cross-references valid
- [ ] Spelling/grammar (optional)
- [ ] Line length limits (optional)

**Output:**
```
Validating documentation...

✓ GETTING_STARTED.md - All checks passed
✓ USER_GUIDE.md - All checks passed
⚠ EXAMPLES.md - Missing example for "XSS Detection"
✗ TROUBLESHOOTING.md - Section "XSS Detection" incomplete

Summary:
  Passed: 2
  Warnings: 1
  Failed: 1

Please fix issues before committing.
```

### 3. update-all-docs.sh

**Purpose:** Update all documentation when feature changes

**Usage:**
```bash
./scripts/docs/update-all-docs.sh [OPTIONS]

Options:
  --feature <name>      # Feature to update
  --update              # Update existing sections
  --add                 # Add new sections
  --version <version>   # Update version numbers
  --interactive         # Interactive mode

Examples:
  # Add new feature to all docs
  ./scripts/docs/update-all-docs.sh --feature "XSS Detection" --add

  # Update existing feature
  ./scripts/docs/update-all-docs.sh --feature "Secrets Detection" --update

  # Update version numbers
  ./scripts/docs/update-all-docs.sh --version "1.6.0"
```

**What it does:**
1. Finds all references to feature
2. Updates version numbers
3. Adds/updates examples
4. Updates command references
5. Regenerates table of contents
6. Updates statistics
7. Validates changes

### 4. doc-stats.sh

**Purpose:** Generate documentation statistics

**Usage:**
```bash
./scripts/docs/doc-stats.sh

Output:
  Documentation Statistics
  ========================

  Total Documentation: 35,000 words
  Files: 25
  Examples: 120
  Code Snippets: 450

  By Type:
    User Guides: 9 files (18,000 words)
    Technical Docs: 8 files (10,000 words)
    Examples: 5 files (5,000 words)
    Integration: 3 files (2,000 words)

  Coverage:
    Features: 100% (25/25)
    Commands: 100% (7/7)
    Detectors: 100% (10/10)
    Integrations: 70% (7/10)
```

---

## Feature Development Process

### Step-by-Step Process

#### Phase 1: Planning (Day 1)

**1. Create Feature Specification**
```bash
# Use feature template
./scripts/docs/generate-doc.sh --type feature --name "xss-detection"
```

**2. Fill in specification:**
- What problem does it solve?
- How will users use it?
- What are the use cases?
- What are the edge cases?

**3. Get approval:**
- Review with team
- Confirm approach
- Agree on interface

#### Phase 2: Implementation (Days 2-4)

**1. Implement feature**
```bash
# Create detector
vim src/detectors/xss.rs

# Add tests
vim tests/detectors/xss_test.rs
```

**2. Write documentation alongside code**

While coding, fill in:
- Examples (as you test manually)
- Troubleshooting (as you debug)
- Configuration (as you add options)

**3. Validate continuously**
```bash
# After each coding session
./scripts/docs/validate-docs.sh --feature "XSS Detection"
```

#### Phase 3: Documentation (Day 5)

**1. Complete all documentation sections**

Ensure these are complete:
- [ ] USER_GUIDE.md - Feature usage
- [ ] EXAMPLES.md - 3+ real-world examples
- [ ] TROUBLESHOOTING.md - Common issues
- [ ] COMMAND_REFERENCE.md - CLI details
- [ ] FAQ.md - Predictable questions
- [ ] CHANGELOG.md - Version entry
- [ ] README.md - Updated feature list

**2. Update all related docs**
```bash
./scripts/docs/update-all-docs.sh --feature "XSS Detection" --add
```

**3. Generate examples**
```bash
# Create example directory
mkdir -p examples/xss-detection

# Add vulnerable and fixed examples
cat > examples/xss-detection/vulnerable.py << 'EOF'
# Vulnerable code example
EOF

cat > examples/xss-detection/fixed.py << 'EOF'
# Fixed code example
EOF
```

**4. Final validation**
```bash
./scripts/docs/validate-docs.sh --all --strict
```

#### Phase 4: Review (Day 6)

**1. Self-review checklist**
- [ ] Run the scanner myself following docs
- [ ] Try all examples - do they work?
- [ ] Can a new user follow the guide?
- [ ] Are there any gaps or confusion?
- [ ] Is the language clear and concise?

**2. Peer review**
- [ ] Another developer reviews code + docs
- [ ] Technical writer reviews docs (if available)
- [ ] User tests following the guide

**3. Final updates**
- Address feedback
- Fix any issues
- Re-validate

#### Phase 5: Release (Day 7)

**1. Merge to main**
```bash
git checkout main
git merge feature/xss-detection
```

**2. Tag release**
```bash
git tag -a v1.6.0 -m "Add XSS detection detector"
git push origin v1.6.0
```

**3. Generate release notes**
```bash
./scripts/docs/generate-release-notes.sh v1.6.0
```

**4. Update documentation site**
- Docs automatically deploy via CI/CD
- GitHub Pages updated
- Documentation index regenerated

---

## CI/CD Integration

### GitHub Actions Workflow

**File:** `.github/workflows/docs-validation.yml`

**Triggers:**
- Every pull request
- Every commit to main
- Manual trigger

**Jobs:**

#### 1. Documentation Validation
```yaml
doc-validation:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3

    - name: Validate Documentation
      run: ./scripts/docs/validate-docs.sh --all --strict

    - name: Check for broken links
      run: ./scripts/docs/check-links.sh

    - name: Validate code examples
      run: ./scripts/docs/validate-examples.sh

    - name: Generate coverage report
      run: ./scripts/docs/doc-stats.sh > doc-coverage.txt

    - name: Comment on PR
      if: failure()
      run: |
        gh pr comment ${{ github.event.pull_request.number }} \
          --body "Documentation validation failed. Please fix issues."
```

#### 2. Documentation Build Test
```yaml
doc-build:
  runs-on: ubuntu-latest
  steps:
    - name: Test example code compiles
      run: ./scripts/docs/test-examples.sh

    - name: Generate documentation site
      run: ./scripts/docs/build-docs-site.sh

    - name: Check for warnings
      run: test $(cat build.log | grep -c WARNING) -eq 0
```

#### 3. Documentation Deploy
```yaml
doc-deploy:
  runs-on: ubuntu-latest
  if: github.ref == 'refs/heads/main'
  steps:
    - name: Deploy to GitHub Pages
      run: ./scripts/docs/deploy-docs.sh
```

### Pull Request Checklist

Automated PR comment includes:

```markdown
## Documentation Checklist

Feature: XSS Detection

- [x] Feature documented in USER_GUIDE.md
- [x] Examples added to EXAMPLES.md
- [ ] Troubleshooting section added (REQUIRED)
- [x] CLI reference updated
- [x] FAQ updated
- [x] Changelog entry added
- [x] README feature list updated
- [ ] Integration guide written (if applicable)

**Status:** ⚠️ 1 item remaining

Please complete all required items before merging.
```

---

## Maintenance

### Regular Maintenance Tasks

#### Weekly
```bash
# Check for broken links
./scripts/docs/check-links.sh

# Validate all docs
./scripts/docs/validate-docs.sh --all

# Update statistics
./scripts/docs/doc-stats.sh > DOCUMENTATION_STATS.md
```

#### Monthly
```bash
# Review and update examples
./scripts/docs/update-examples.sh

# Check for outdated content
./scripts/docs/find-outdated.sh

# Generate what's new blog post
./scripts/docs/generate-whats-new.sh
```

#### Quarterly
```bash
# Comprehensive documentation audit
./scripts/docs/audit-docs.sh

# User feedback review
./scripts/docs/review-feedback.sh

# Documentation restructuring (if needed)
./scripts/docs/restructure.sh
```

### Documentation Metrics

Track these metrics:

| Metric | Target | Current |
|--------|--------|---------|
| Feature coverage | 100% | 100% |
| Example coverage | 100% | 100% |
| Broken links | 0 | 0 |
| Average doc age | <30 days | 15 days |
| User feedback score | >4.5/5 | 4.8/5 |
| Time to find info | <2 min | 1.5 min |

---

## Benefits

### For Developers
- ✅ **Clear process** - Know exactly what docs to write
- ✅ **Templates** - Don't start from scratch
- ✅ **Automation** - Scripts handle repetitive tasks
- ✅ **Validation** - Catch issues early

### For Users
- ✅ **Always up-to-date** - Docs match current version
- ✅ **Comprehensive** - Every feature documented
- ✅ **Consistent** - Same quality everywhere
- ✅ **Accurate** - Examples actually work

### For Project
- ✅ **Zero documentation debt** - Never falls behind
- ✅ **Quality maintained** - Automated checks
- ✅ **Easier onboarding** - New contributors have clear process
- ✅ **Professional image** - World-class documentation

---

## Getting Started with Doc Automation

### For New Features

```bash
# 1. Start feature branch
git checkout -b feature/my-new-feature

# 2. Generate documentation template
./scripts/docs/generate-doc.sh --type feature --name "my-new-feature"

# 3. Implement feature + docs together
# (Edit code and documentation files)

# 4. Validate
./scripts/docs/validate-docs.sh --feature "My New Feature"

# 5. Create PR
git push origin feature/my-new-feature
# CI will validate documentation automatically
```

### For Contributors

See **[FEATURE_CHECKLIST.md](FEATURE_CHECKLIST.md)** for complete step-by-step checklist.

---

**Documentation is not optional. It's a first-class citizen in MCP Sentinel!** 📚✨

**Questions?** Open an issue or discussion on GitHub.
