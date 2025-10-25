# Feature Development Checklist

Complete checklist for adding new features to MCP Sentinel with automated documentation.

## Before You Start

- [ ] Read [DOCUMENTATION_AUTOMATION.md](DOCUMENTATION_AUTOMATION.md)
- [ ] Understand the feature requirements
- [ ] Create feature branch: `git checkout -b feature/my-feature`

---

## Phase 1: Planning & Documentation Setup (Day 1)

### 1.1 Generate Documentation Template

```bash
# For detector
./scripts/docs/generate-doc.sh --type detector --name "xss-detection"

# For feature
./scripts/docs/generate-doc.sh --type feature --name "real-time-monitoring"
```

**Result:** Documentation template created with placeholders.

### 1.2 Fill In Specification

Edit the generated documentation file:

- [ ] Overview section - What does it do?
- [ ] Use cases - Why is it needed?
- [ ] Examples - How will it be used?
- [ ] Configuration - What options will it have?

### 1.3 Review with Team

- [ ] Present specification to team
- [ ] Get feedback on approach
- [ ] Confirm interface/API design
- [ ] Update documentation based on feedback

---

## Phase 2: Implementation (Days 2-4)

### 2.1 Implement Core Functionality

- [ ] Create source files (e.g., `src/detectors/xss.rs`)
- [ ] Implement core logic
- [ ] Add error handling
- [ ] Add logging

**Note:** Write documentation ALONGSIDE code, not after!

### 2.2 Document As You Code

While implementing, fill in documentation:

- [ ] Add real code examples (from your manual testing)
- [ ] Document configuration options (as you add them)
- [ ] Note troubleshooting issues (as you encounter bugs)
- [ ] Add performance notes (from your observations)

### 2.3 Create Examples

- [ ] Add vulnerable code samples to `examples/[feature]/vulnerable.*`
- [ ] Add fixed code samples to `examples/[feature]/fixed.*`
- [ ] Test examples work correctly
- [ ] Add example README

### 2.4 Write Tests

- [ ] Unit tests (`tests/[type]/[name]_test.rs`)
  - [ ] Basic functionality
  - [ ] Edge cases
  - [ ] Error handling
  - [ ] False positives
- [ ] Integration tests
  - [ ] End-to-end scenarios
  - [ ] CLI integration
  - [ ] Output format validation

### 2.5 Validate Continuously

After each coding session:

```bash
# Validate your documentation
./scripts/docs/validate-docs.sh --feature "Your Feature Name"

# Run tests
cargo test [feature_name]

# Manual testing
mcp-sentinel scan examples/[feature]/vulnerable.*
```

---

## Phase 3: Complete Documentation (Day 5)

### 3.1 Complete All Documentation Sections

**Main documentation file** (`[type]-[name].md`):

- [ ] Overview complete
- [ ] All patterns documented (for detectors)
- [ ] All options documented (for features)
- [ ] 3+ code examples (vulnerable + fixed)
- [ ] Configuration section complete
- [ ] False positives documented
- [ ] CWE mappings (for detectors)
- [ ] Troubleshooting section complete
- [ ] No TODO markers remaining

### 3.2 Update Integration Documentation

**USER_GUIDE.md:**
- [ ] Add detector/feature to relevant section
- [ ] Add usage examples
- [ ] Add to table of contents

**EXAMPLES.md:**
- [ ] Add 2+ real-world examples
- [ ] Include before/after code
- [ ] Add CI/CD integration example (if applicable)

**COMMAND_REFERENCE.md:**
- [ ] Document CLI commands (if new command)
- [ ] Document CLI options (if new options)
- [ ] Add examples

**TROUBLESHOOTING.md:**
- [ ] Add common issues section
- [ ] Provide solutions
- [ ] Add debugging tips

**FAQ.md:**
- [ ] Add 3+ questions about the feature
- [ ] Provide clear answers
- [ ] Link to detailed docs

**BEST_PRACTICES.md:**
- [ ] Add best practice section (if applicable)
- [ ] Document recommended usage patterns
- [ ] Add anti-patterns to avoid

**README.md:**
- [ ] Update feature list
- [ ] Update statistics (if major feature)
- [ ] Update detector count (if detector)

**CHANGELOG.md:**
- [ ] Add entry for upcoming version
- [ ] List new features/changes
- [ ] Note any breaking changes
- [ ] Mention migration steps (if needed)

### 3.3 Validate Everything

```bash
# Run full documentation validation
./scripts/docs/validate-docs.sh --all

# Check for broken links
./scripts/docs/check-links.sh

# Run all tests
cargo test

# Build to ensure no compile errors
cargo build --release
```

---

## Phase 4: Review (Day 6)

### 4.1 Self-Review

- [ ] Run the feature following your own documentation
- [ ] Try all examples - do they work?
- [ ] Read documentation as if you're a new user
- [ ] Check for typos and grammar
- [ ] Ensure language is clear and concise
- [ ] Verify all links work
- [ ] Ensure consistent formatting

### 4.2 Peer Review

**Code Review:**
- [ ] Assign reviewer for code changes
- [ ] Address code review feedback
- [ ] Update code and tests as needed

**Documentation Review:**
- [ ] Assign reviewer for documentation
- [ ] Address documentation feedback
- [ ] Fix any unclear sections
- [ ] Add missing information

**User Testing** (if possible):
- [ ] Have someone follow your guide
- [ ] Note any confusion or issues
- [ ] Update documentation to clarify

### 4.3 CI/CD Validation

```bash
# Run CI checks locally
./.github/workflows/test-locally.sh

# Or push to PR and wait for CI
git push origin feature/my-feature
# CI will run automatically
```

**CI checks that must pass:**
- [ ] Documentation validation
- [ ] No broken links
- [ ] Code examples compile
- [ ] All tests pass
- [ ] No linting errors
- [ ] Changelog updated

---

## Phase 5: Final Steps (Day 7)

### 5.1 Pre-Merge Checklist

- [ ] All tests passing
- [ ] All CI checks green
- [ ] Documentation validation passing
- [ ] Code review approved
- [ ] Documentation review approved
- [ ] No merge conflicts

### 5.2 Merge to Main

```bash
# Update from main
git checkout main
git pull origin main

# Merge your branch
git checkout feature/my-feature
git rebase main

# Push
git push origin feature/my-feature

# Create PR or merge (depends on your workflow)
```

### 5.3 Post-Merge

- [ ] Verify CI/CD passes on main
- [ ] Tag release (if releasing)
- [ ] Generate release notes
- [ ] Update documentation site
- [ ] Announce in changelog/release notes

---

## Complete Checklist Summary

Use this summary checklist to ensure nothing is missed:

### Code
- [ ] Feature/detector implemented
- [ ] Tests written (unit + integration)
- [ ] All tests passing
- [ ] No compiler warnings
- [ ] Code reviewed and approved

### Documentation
- [ ] Main documentation file complete
- [ ] USER_GUIDE.md updated
- [ ] EXAMPLES.md updated
- [ ] TROUBLESHOOTING.md updated
- [ ] FAQ.md updated
- [ ] COMMAND_REFERENCE.md updated (if applicable)
- [ ] BEST_PRACTICES.md updated (if applicable)
- [ ] README.md updated
- [ ] CHANGELOG.md updated
- [ ] No TODO markers
- [ ] No broken links
- [ ] All code examples tested
- [ ] Documentation reviewed and approved

### Examples
- [ ] Vulnerable code examples created
- [ ] Fixed code examples created
- [ ] Examples tested with scanner
- [ ] Example README created

### Testing
- [ ] Unit tests written
- [ ] Integration tests written
- [ ] Manual testing completed
- [ ] CI/CD passing

### Review
- [ ] Self-review completed
- [ ] Code peer review completed
- [ ] Documentation peer review completed
- [ ] User testing completed (if applicable)

---

## Quick Reference: Common Commands

```bash
# Generate documentation
./scripts/docs/generate-doc.sh --type detector --name "my-detector"

# Validate documentation
./scripts/docs/validate-docs.sh --feature "My Feature"
./scripts/docs/validate-docs.sh --all

# Run tests
cargo test my_feature
cargo test --all

# Build
cargo build --release

# Manual scan testing
mcp-sentinel scan examples/my-feature/vulnerable.py

# Check documentation stats
./scripts/docs/doc-stats.sh
```

---

## Tips for Success

### Do's ✅

1. **Write docs alongside code** - Don't leave it for later
2. **Test your examples** - Make sure they actually work
3. **Keep it simple** - Clear language, short sentences
4. **Add visuals** - Code examples, command outputs, diagrams
5. **Think like a user** - What would confuse you?
6. **Validate often** - Catch issues early
7. **Ask for review** - Fresh eyes catch mistakes

### Don'ts ❌

1. **Don't skip documentation** - It's not optional
2. **Don't use jargon** - Explain technical terms
3. **Don't leave TODOs** - Finish sections completely
4. **Don't copy-paste** - Customize examples for your feature
5. **Don't skip validation** - Run checks before submitting
6. **Don't merge without approval** - Wait for reviews
7. **Don't forget CHANGELOG** - Users need to know what changed

---

## Need Help?

- **Documentation System:** Read [DOCUMENTATION_AUTOMATION.md](DOCUMENTATION_AUTOMATION.md)
- **Contributing Guidelines:** Read [CONTRIBUTING.md](CONTRIBUTING.md)
- **Questions:** Open a discussion on GitHub
- **Issues:** File an issue if you find problems

---

## Time Estimates

| Phase | Estimated Time | Activities |
|-------|---------------|------------|
| Phase 1 | 4 hours | Planning, spec writing |
| Phase 2 | 3 days | Implementation, testing |
| Phase 3 | 1 day | Complete documentation |
| Phase 4 | 1 day | Reviews, fixes |
| Phase 5 | 2 hours | Merge, release |
| **Total** | **5-6 days** | Full feature + docs |

**Note:** Times vary based on feature complexity. Simple detectors may take 2-3 days total.

---

**Remember: Documentation is a first-class citizen. No feature is complete without complete documentation!** 📚✨
