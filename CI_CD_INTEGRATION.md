# CI/CD Integration Guide

Complete guide to integrating MCP Sentinel into your CI/CD pipelines.

## Table of Contents

1. [Overview](#overview)
2. [GitHub Actions](#github-actions)
3. [GitLab CI](#gitlab-ci)
4. [Jenkins](#jenkins)
5. [Azure DevOps](#azure-devops)
6. [CircleCI](#circleci)
7. [Travis CI](#travis-ci)
8. [Pre-commit Hooks](#pre-commit-hooks)
9. [Docker Integration](#docker-integration)
10. [Best Practices](#best-practices)

---

## Overview

Integrating MCP Sentinel into your CI/CD pipeline ensures every code change is automatically scanned for security vulnerabilities before deployment.

###Benefits:
- **Early Detection**: Find vulnerabilities during development
- **Automated**: No manual security reviews needed
- **Consistent**: Same standards applied to all code
- **Fast Feedback**: Developers get immediate results
- **Prevent Incidents**: Block insecure code from reaching production

### Integration Strategy

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Commit    │────>│   PR/MR     │────>│    Main     │
│             │     │             │     │   Branch    │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │                    │
       │                   │                    │
       ▼                   ▼                    ▼
  Block         Block Critical/High      Zero Tolerance
  Critical      Allow Medium/Low         Block All Issues
```

---

## GitHub Actions

### Basic Security Scan

Create `.github/workflows/security.yml`:

```yaml
name: Security Scan

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  security:
    name: MCP Sentinel Security Scan
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal

      - name: Cache MCP Sentinel
        id: cache-sentinel
        uses: actions/cache@v3
        with:
          path: ~/.cargo/bin/mcp-sentinel
          key: mcp-sentinel-v1.5.0

      - name: Install MCP Sentinel
        if: steps.cache-sentinel.outputs.cache-hit != 'true'
        run: |
          git clone https://github.com/beejak/MCP_Sentinel.git /tmp/sentinel
          cd /tmp/sentinel
          cargo build --release
          cp target/release/mcp-sentinel ~/.cargo/bin/

      - name: Run Security Scan
        run: |
          mcp-sentinel scan . --format json --output security-results.json

      - name: Upload Results
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: security-results
          path: security-results.json

      - name: Check Security Gate
        run: |
          CRITICAL=$(jq '.summary.by_severity.critical' security-results.json)
          HIGH=$(jq '.summary.by_severity.high' security-results.json)

          echo "Security Scan Results:"
          echo "  Critical: $CRITICAL"
          echo "  High: $HIGH"

          if [ "$CRITICAL" -gt 0 ]; then
            echo "::error::Found $CRITICAL critical vulnerabilities!"
            exit 1
          fi

          if [ "$HIGH" -gt 5 ]; then
            echo "::warning::Found $HIGH high vulnerabilities (threshold: 5)"
            exit 1
          fi

          echo "::notice::Security scan passed ✓"
```

### Advanced: SARIF Upload to GitHub Code Scanning

```yaml
name: Security Scan with Code Scanning

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 0 * * 0'  # Weekly scan

jobs:
  security:
    name: Security Analysis
    runs-on: ubuntu-latest
    permissions:
      security-events: write
      contents: read

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build MCP Sentinel
        run: |
          git clone https://github.com/beejak/MCP_Sentinel.git
          cd MCP_Sentinel
          cargo build --release
          sudo cp target/release/mcp-sentinel /usr/local/bin/

      - name: Run Scan
        run: |
          mcp-sentinel scan . --format sarif --output results.sarif

      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: results.sarif
          category: mcp-sentinel

      - name: Security Gate
        run: |
          mcp-sentinel scan . --format json --output results.json
          CRITICAL=$(jq '.summary.by_severity.critical' results.json)
          if [ "$CRITICAL" -gt 0 ]; then
            exit 1
          fi
```

### Matrix Strategy: Multi-Environment Testing

```yaml
name: Multi-Environment Security

on: [push, pull_request]

jobs:
  security:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        include:
          - os: ubuntu-latest
            binary: mcp-sentinel
          - os: macos-latest
            binary: mcp-sentinel
          - os: windows-latest
            binary: mcp-sentinel.exe

    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build MCP Sentinel
        run: |
          git clone https://github.com/beejak/MCP_Sentinel.git
          cd MCP_Sentinel
          cargo build --release

      - name: Run Scan
        run: |
          MCP_Sentinel/target/release/${{ matrix.binary }} scan . --format json
```

---

## GitLab CI

### Basic Pipeline

`.gitlab-ci.yml`:

```yaml
stages:
  - security
  - deploy

variables:
  SENTINEL_VERSION: "1.5.0"

security_scan:
  stage: security
  image: rust:latest

  before_script:
    - apt-get update && apt-get install -y jq
    - |
      if [ ! -f mcp-sentinel ]; then
        git clone https://github.com/beejak/MCP_Sentinel.git
        cd MCP_Sentinel
        cargo build --release
        cp target/release/mcp-sentinel /usr/local/bin/
        cd ..
      fi

  script:
    - echo "Running MCP Sentinel security scan..."
    - mcp-sentinel scan . --format json --output results.json
    - |
      CRITICAL=$(jq '.summary.by_severity.critical' results.json)
      HIGH=$(jq '.summary.by_severity.high' results.json)

      echo "Critical: $CRITICAL"
      echo "High: $HIGH"

      if [ "$CRITICAL" -gt 0 ]; then
        echo "CRITICAL vulnerabilities found!"
        exit 1
      fi

  artifacts:
    reports:
      json: results.json
    paths:
      - results.json
    expire_in: 30 days
    when: always

  only:
    - merge_requests
    - main
    - develop

deploy:
  stage: deploy
  script:
    - echo "Deploying..."
  only:
    - main
  dependencies:
    - security_scan
```

### Advanced: Security Dashboard Integration

```yaml
security_scan:
  stage: security
  image: rust:latest

  script:
    - mcp-sentinel scan . --format json --output gl-sast-report.json

  artifacts:
    reports:
      sast: gl-sast-report.json
    paths:
      - gl-sast-report.json
    expire_in: 1 week

  allow_failure: false

security_dashboard:
  stage: security
  image: curlimages/curl:latest

  script:
    - |
      curl -X POST "${CI_API_V4_URL}/projects/${CI_PROJECT_ID}/vulnerabilities" \
        -H "PRIVATE-TOKEN: ${CI_JOB_TOKEN}" \
        -H "Content-Type: application/json" \
        -d @gl-sast-report.json
```

---

## Jenkins

### Declarative Pipeline

`Jenkinsfile`:

```groovy
pipeline {
    agent any

    environment {
        SENTINEL_PATH = '/usr/local/bin/mcp-sentinel'
        RESULTS_FILE = 'security-results.json'
    }

    stages {
        stage('Setup') {
            steps {
                script {
                    if (!fileExists(env.SENTINEL_PATH)) {
                        sh '''
                            git clone https://github.com/beejak/MCP_Sentinel.git /tmp/sentinel
                            cd /tmp/sentinel
                            cargo build --release
                            sudo cp target/release/mcp-sentinel ${SENTINEL_PATH}
                        '''
                    }
                }
            }
        }

        stage('Security Scan') {
            steps {
                sh """
                    ${SENTINEL_PATH} scan . --format json --output ${RESULTS_FILE}
                """
            }
        }

        stage('Analyze Results') {
            steps {
                script {
                    def results = readJSON file: env.RESULTS_FILE
                    def critical = results.summary.by_severity.critical
                    def high = results.summary.by_severity.high

                    echo "Security Scan Results:"
                    echo "  Critical: ${critical}"
                    echo "  High: ${high}"

                    if (critical > 0) {
                        error("Found ${critical} critical vulnerabilities!")
                    }

                    if (high > 10) {
                        unstable("Found ${high} high severity vulnerabilities (threshold: 10)")
                    }
                }
            }
        }

        stage('Generate Report') {
            steps {
                sh '''
                    cat ${RESULTS_FILE} | jq -r '.vulnerabilities[] |
                        "[\(.severity | ascii_upcase)] \(.file):\(.line) - \(.title)"' \
                        > security-report.txt
                '''
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: "${RESULTS_FILE},security-report.txt",
                             fingerprint: true

            publishHTML([
                reportDir: '.',
                reportFiles: 'security-report.txt',
                reportName: 'Security Scan Report'
            ])
        }

        failure {
            emailext(
                subject: "Security Scan Failed: ${env.JOB_NAME} #${env.BUILD_NUMBER}",
                body: "Security scan found critical vulnerabilities. Check ${env.BUILD_URL}",
                to: "security-team@company.com"
            )
        }

        success {
            echo "Security scan passed!"
        }
    }
}
```

### Scripted Pipeline with Quality Gates

```groovy
node {
    stage('Checkout') {
        checkout scm
    }

    stage('Security Scan') {
        sh 'mcp-sentinel scan . --format json --output results.json'
    }

    stage('Quality Gate') {
        def results = readJSON file: 'results.json'
        def metrics = [
            critical: results.summary.by_severity.critical,
            high: results.summary.by_severity.high,
            medium: results.summary.by_severity.medium
        ]

        // Define thresholds based on branch
        def branch = env.BRANCH_NAME
        def thresholds = [:]

        if (branch == 'main') {
            thresholds = [critical: 0, high: 0, medium: 5]
        } else if (branch == 'develop') {
            thresholds = [critical: 0, high: 5, medium: 20]
        } else {
            thresholds = [critical: 2, high: 10, medium: 50]
        }

        echo "Branch: ${branch}"
        echo "Metrics: ${metrics}"
        echo "Thresholds: ${thresholds}"

        def failed = false

        if (metrics.critical > thresholds.critical) {
            echo "FAIL: Critical vulnerabilities (${metrics.critical} > ${thresholds.critical})"
            failed = true
        }

        if (metrics.high > thresholds.high) {
            echo "FAIL: High vulnerabilities (${metrics.high} > ${thresholds.high})"
            failed = true
        }

        if (metrics.medium > thresholds.medium) {
            echo "WARNING: Medium vulnerabilities (${metrics.medium} > ${thresholds.medium})"
        }

        if (failed) {
            error("Security quality gate failed!")
        }
    }
}
```

---

## Azure DevOps

`azure-pipelines.yml`:

```yaml
trigger:
  branches:
    include:
      - main
      - develop

pr:
  branches:
    include:
      - main

pool:
  vmImage: 'ubuntu-latest'

variables:
  sentinelVersion: '1.5.0'

stages:
  - stage: Security
    displayName: 'Security Scanning'
    jobs:
      - job: ScanCode
        displayName: 'Scan with MCP Sentinel'
        steps:
          - task: Cache@2
            inputs:
              key: 'sentinel | "$(sentinelVersion)"'
              path: $(Pipeline.Workspace)/.sentinel
            displayName: 'Cache MCP Sentinel'

          - bash: |
              if [ ! -f $(Pipeline.Workspace)/.sentinel/mcp-sentinel ]; then
                git clone https://github.com/beejak/MCP_Sentinel.git /tmp/sentinel
                cd /tmp/sentinel
                cargo build --release
                mkdir -p $(Pipeline.Workspace)/.sentinel
                cp target/release/mcp-sentinel $(Pipeline.Workspace)/.sentinel/
              fi
            displayName: 'Install MCP Sentinel'

          - bash: |
              $(Pipeline.Workspace)/.sentinel/mcp-sentinel scan . \
                --format sarif --output results.sarif
            displayName: 'Run Security Scan'

          - task: PublishBuildArtifacts@1
            inputs:
              pathToPublish: 'results.sarif'
              artifactName: 'CodeAnalysisLogs'
            displayName: 'Publish SARIF Results'

          - bash: |
              $(Pipeline.Workspace)/.sentinel/mcp-sentinel scan . \
                --format json --output results.json

              CRITICAL=$(jq '.summary.by_severity.critical' results.json)

              if [ "$CRITICAL" -gt 0 ]; then
                echo "##vso[task.logissue type=error]Found $CRITICAL critical vulnerabilities"
                echo "##vso[task.complete result=Failed;]Critical vulnerabilities found"
                exit 1
              fi

              echo "##vso[task.complete result=Succeeded;]Security scan passed"
            displayName: 'Check Security Gate'

  - stage: Deploy
    displayName: 'Deploy'
    dependsOn: Security
    condition: and(succeeded(), eq(variables['Build.SourceBranch'], 'refs/heads/main'))
    jobs:
      - job: DeployApp
        displayName: 'Deploy Application'
        steps:
          - bash: echo "Deploying..."
```

---

## CircleCI

`.circleci/config.yml`:

```yaml
version: 2.1

executors:
  rust-executor:
    docker:
      - image: rust:latest

jobs:
  security-scan:
    executor: rust-executor
    steps:
      - checkout

      - restore_cache:
          keys:
            - sentinel-v1-{{ checksum "Cargo.lock" }}
            - sentinel-v1-

      - run:
          name: Install MCP Sentinel
          command: |
            if [ ! -f ~/mcp-sentinel ]; then
              git clone https://github.com/beejak/MCP_Sentinel.git /tmp/sentinel
              cd /tmp/sentinel
              cargo build --release
              cp target/release/mcp-sentinel ~/mcp-sentinel
            fi

      - save_cache:
          key: sentinel-v1-{{ checksum "Cargo.lock" }}
          paths:
            - ~/mcp-sentinel

      - run:
          name: Run Security Scan
          command: |
            ~/mcp-sentinel scan . --format json --output results.json

      - run:
          name: Check Results
          command: |
            CRITICAL=$(jq '.summary.by_severity.critical' results.json)
            if [ "$CRITICAL" -gt 0 ]; then
              echo "Critical vulnerabilities found!"
              exit 1
            fi

      - store_artifacts:
          path: results.json
          destination: security-results

      - store_test_results:
          path: results.json

workflows:
  version: 2
  security-workflow:
    jobs:
      - security-scan:
          filters:
            branches:
              only:
                - main
                - develop
```

---

## Travis CI

`.travis.yml`:

```yaml
language: rust
rust:
  - stable

cache:
  directories:
    - $HOME/.cargo
    - target/

before_install:
  - sudo apt-get update
  - sudo apt-get install -y jq

install:
  - |
    if [ ! -f $HOME/.cargo/bin/mcp-sentinel ]; then
      git clone https://github.com/beejak/MCP_Sentinel.git /tmp/sentinel
      cd /tmp/sentinel
      cargo build --release
      cp target/release/mcp-sentinel $HOME/.cargo/bin/
      cd -
    fi

script:
  - mcp-sentinel scan . --format json --output results.json

after_success:
  - |
    CRITICAL=$(jq '.summary.by_severity.critical' results.json)
    if [ "$CRITICAL" -gt 0 ]; then
      echo "Critical vulnerabilities found!"
      exit 1
    fi

notifications:
  email:
    on_success: change
    on_failure: always
```

---

## Pre-commit Hooks

### Git Pre-commit Hook

`.git/hooks/pre-commit`:

```bash
#!/bin/bash
# MCP Sentinel pre-commit hook

echo "🔍 Running MCP Sentinel security scan..."

# Get list of staged files
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)

if [ -z "$STAGED_FILES" ]; then
  echo "No files to scan"
  exit 0
fi

# Create temp file with staged files
TEMP_FILE=$(mktemp)
echo "$STAGED_FILES" > "$TEMP_FILE"

# Scan staged files
cat "$TEMP_FILE" | xargs mcp-sentinel scan --min-severity high --format json --output .git/scan-results.json

# Check results
if [ $? -ne 0 ]; then
  echo "❌ Security scan failed!"
  exit 1
fi

CRITICAL=$(jq '.summary.by_severity.critical' .git/scan-results.json 2>/dev/null || echo "0")
HIGH=$(jq '.summary.by_severity.high' .git/scan-results.json 2>/dev/null || echo "0")

if [ "$CRITICAL" -gt 0 ] || [ "$HIGH" -gt 0 ]; then
  echo "❌ Found security issues:"
  echo "   Critical: $CRITICAL"
  echo "   High: $HIGH"
  echo ""
  echo "Fix these issues before committing, or use --no-verify to skip"
  exit 1
fi

echo "✅ Security scan passed!"
rm "$TEMP_FILE"
exit 0
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

### Pre-commit Framework

`.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: mcp-sentinel
        name: MCP Sentinel Security Scan
        entry: mcp-sentinel scan
        language: system
        pass_filenames: true
        types: [python, javascript, typescript]
```

Install:

```bash
pip install pre-commit
pre-commit install
```

---

## Docker Integration

### Dockerfile for MCP Sentinel

```dockerfile
FROM rust:latest as builder

WORKDIR /app

# Clone and build MCP Sentinel
RUN git clone https://github.com/beejak/MCP_Sentinel.git . && \
    cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mcp-sentinel /usr/local/bin/

ENTRYPOINT ["mcp-sentinel"]
CMD ["--help"]
```

Build:

```bash
docker build -t mcp-sentinel:latest .
```

### Use in CI/CD

```yaml
# GitHub Actions
- name: Run MCP Sentinel
  run: |
    docker run --rm -v $(pwd):/workspace \
      mcp-sentinel:latest scan /workspace --format json

# GitLab CI
security_scan:
  image: mcp-sentinel:latest
  script:
    - mcp-sentinel scan . --format json
```

---

## Best Practices

### 1. Progressive Security Gates

Implement stricter checks as code progresses:

```bash
# Feature branches: Block only critical
if [[ "$BRANCH" =~ ^feature/.* ]]; then
  mcp-sentinel scan . --min-severity critical

# Develop: Block critical + high
elif [ "$BRANCH" = "develop" ]; then
  mcp-sentinel scan . --min-severity high

# Main: Zero tolerance
elif [ "$BRANCH" = "main" ]; then
  mcp-sentinel scan . --min-severity low
fi
```

### 2. Fail Fast

Run security scans early in the pipeline:

```yaml
stages:
  - security  # First!
  - test
  - build
  - deploy
```

### 3. Cache the Binary

Speed up scans by caching MCP Sentinel:

```yaml
- uses: actions/cache@v3
  with:
    path: ~/.cargo/bin/mcp-sentinel
    key: sentinel-${{ runner.os }}-v1.5.0
```

### 4. Scan Only Changed Files

For faster feedback in PRs:

```bash
git diff --name-only origin/main...HEAD | xargs mcp-sentinel scan
```

### 5. Parallel Scanning

Use multiple jobs for large projects:

```yaml
jobs:
  scan-python:
    - mcp-sentinel scan . --include "*.py"

  scan-javascript:
    - mcp-sentinel scan . --include "*.js"
```

### 6. Security Metrics

Track security trends over time:

```bash
mcp-sentinel scan . --format json --output "scan-$(date +%Y%m%d).json"
# Store in artifact storage
```

### 7. Notifications

Alert teams on security issues:

```yaml
- name: Notify on Failure
  if: failure()
  uses: 8398a7/action-slack@v3
  with:
    status: ${{ job.status }}
    text: 'Security scan failed!'
    webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

---

**Next Steps:**
- [User Guide](USER_GUIDE.md) - Complete feature documentation
- [Examples](EXAMPLES.md) - Real-world scanning examples
- [Troubleshooting](TROUBLESHOOTING.md) - Common issues

**Happy scanning!** 🛡️
