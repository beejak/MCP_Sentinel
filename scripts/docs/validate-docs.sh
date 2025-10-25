#!/bin/bash
# validate-docs.sh - Validate documentation completeness and quality
#
# Usage: ./validate-docs.sh [OPTIONS]
#
# Options:
#   --feature <name>    Validate specific feature
#   --all               Validate all documentation
#   --strict            Strict mode (fail on warnings)
#   --fix               Auto-fix simple issues
#   --help              Show help

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
WARNINGS=0
FAILED=0

# Options
FEATURE=""
ALL=false
STRICT=false
FIX=false

# Helper functions
error() {
    echo -e "${RED}✗ $1${NC}"
    ((FAILED++))
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
    ((PASSED++))
}

warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
    ((WARNINGS++))
}

info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Usage
usage() {
    cat << EOF
Documentation Validator for MCP Sentinel

Usage: $(basename "$0") [OPTIONS]

Options:
    --feature NAME    Validate specific feature documentation
    --all             Validate all documentation files
    --strict          Fail on warnings (for CI/CD)
    --fix             Auto-fix simple issues
    --help            Show this help message

Examples:
    # Validate all documentation
    $(basename "$0") --all

    # Validate specific feature
    $(basename "$0") --feature "XSS Detection"

    # Strict validation for CI/CD
    $(basename "$0") --all --strict

    # Validate and auto-fix
    $(basename "$0") --all --fix

EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --feature)
            FEATURE="$2"
            shift 2
            ;;
        --all)
            ALL=true
            shift
            ;;
        --strict)
            STRICT=true
            shift
            ;;
        --fix)
            FIX=true
            shift
            ;;
        --help)
            usage
            ;;
        *)
            echo "Unknown option: $1"
            usage
            ;;
    esac
done

# Validation functions

check_file_exists() {
    local file=$1
    if [ ! -f "$file" ]; then
        error "File not found: $file"
        return 1
    fi
    return 0
}

check_required_sections() {
    local file=$1
    local sections=("${@:2}")

    for section in "${sections[@]}"; do
        if ! grep -q "^## $section" "$file"; then
            error "$file: Missing required section '$section'"
        else
            success "$file: Has section '$section'"
        fi
    done
}

check_no_todos() {
    local file=$1

    local todo_count=$(grep -c "TODO\|FIXME\|XXX" "$file" 2>/dev/null || echo "0")

    if [ "$todo_count" -gt 0 ]; then
        if [ "$FIX" = true ]; then
            info "$file: $todo_count TODO markers (use --fix to see them)"
        else
            warning "$file: Contains $todo_count TODO/FIXME markers"
        fi
    else
        success "$file: No TODO markers"
    fi
}

check_no_broken_links() {
    local file=$1

    # Extract markdown links
    local links=$(grep -oP '\[.*?\]\(\K[^)]+' "$file" 2>/dev/null || echo "")

    if [ -z "$links" ]; then
        success "$file: No links to check"
        return
    fi

    local broken=0
    while IFS= read -r link; do
        # Skip external links (http/https)
        if [[ "$link" =~ ^https?:// ]]; then
            continue
        fi

        # Skip anchor links
        if [[ "$link" =~ ^# ]]; then
            continue
        fi

        # Check if file exists
        local link_file=$(echo "$link" | cut -d'#' -f1)
        if [ -n "$link_file" ] && [ ! -f "$link_file" ]; then
            error "$file: Broken link to '$link_file'"
            ((broken++))
        fi
    done <<< "$links"

    if [ "$broken" -eq 0 ]; then
        success "$file: No broken links"
    fi
}

check_code_examples() {
    local file=$1

    # Count code blocks
    local code_blocks=$(grep -c '```' "$file" 2>/dev/null || echo "0")

    # Should have even number (opening and closing)
    if [ $((code_blocks % 2)) -ne 0 ]; then
        error "$file: Unclosed code block"
    else
        local num_examples=$((code_blocks / 2))
        if [ "$num_examples" -eq 0 ]; then
            warning "$file: No code examples found"
        else
            success "$file: Has $num_examples code examples"
        fi
    fi
}

check_version_consistency() {
    local file=$1

    # Extract version numbers
    local versions=$(grep -oP 'v?\d+\.\d+\.\d+' "$file" 2>/dev/null || echo "")

    if [ -z "$versions" ]; then
        return
    fi

    # Get most common version
    local expected_version=$(echo "$versions" | sort | uniq -c | sort -rn | head -1 | awk '{print $2}')

    # Check for inconsistent versions
    local inconsistent=$(echo "$versions" | grep -v "$expected_version" | wc -l)

    if [ "$inconsistent" -gt 0 ]; then
        warning "$file: Inconsistent version numbers (expected: $expected_version)"
    else
        success "$file: Version numbers consistent ($expected_version)"
    fi
}

check_changelog_updated() {
    if [ ! -f "CHANGELOG.md" ]; then
        error "CHANGELOG.md not found"
        return
    fi

    # Get current version from Cargo.toml
    local current_version=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)

    if grep -q "## \[${current_version}\]" CHANGELOG.md; then
        success "CHANGELOG.md: Has entry for v${current_version}"
    else
        error "CHANGELOG.md: Missing entry for v${current_version}"
    fi
}

check_examples_directory() {
    local feature=$1
    local example_dir="examples/$(echo "$feature" | tr '[:upper:] ' '[:lower:]-')"

    if [ -d "$example_dir" ]; then
        # Check for vulnerable and fixed examples
        if [ -f "$example_dir/vulnerable.py" ] || [ -f "$example_dir/vulnerable.js" ]; then
            success "Examples: Has vulnerable code sample"
        else
            warning "Examples: Missing vulnerable code sample"
        fi

        if [ -f "$example_dir/fixed.py" ] || [ -f "$example_dir/fixed.js" ]; then
            success "Examples: Has fixed code sample"
        else
            warning "Examples: Missing fixed code sample"
        fi
    else
        warning "Examples: Directory not found: $example_dir"
    fi
}

validate_file() {
    local file=$1

    info "Validating $file..."

    check_file_exists "$file" || return

    # Check for required sections based on file type
    case "$file" in
        USER_GUIDE.md)
            check_required_sections "$file" "Overview" "Command Line Interface" "Scanning" "Detectors Deep Dive"
            ;;
        GETTING_STARTED.md)
            check_required_sections "$file" "What is MCP Sentinel" "System Requirements" "Installation" "Your First Scan"
            ;;
        EXAMPLES.md)
            check_required_sections "$file" "Quick Start Examples" "Language-Specific Examples" "Real-World Scenarios"
            ;;
        *)
            # Generic checks
            ;;
    esac

    check_no_todos "$file"
    check_no_broken_links "$file"
    check_code_examples "$file"
    check_version_consistency "$file"
}

# Main validation
echo "🔍 MCP Sentinel Documentation Validator"
echo ""

if [ "$ALL" = true ]; then
    info "Validating all documentation files..."
    echo ""

    # Core documentation
    for file in README.md GETTING_STARTED.md USER_GUIDE.md EXAMPLES.md \
                TROUBLESHOOTING.md FAQ.md BEST_PRACTICES.md ADVANCED_USAGE.md \
                CI_CD_INTEGRATION.md COMMAND_REFERENCE.md; do
        if [ -f "$file" ]; then
            validate_file "$file"
            echo ""
        fi
    done

    # Check changelog
    check_changelog_updated
    echo ""

elif [ -n "$FEATURE" ]; then
    info "Validating documentation for feature: $FEATURE"
    echo ""

    # Check if feature is documented in key files
    for file in USER_GUIDE.md EXAMPLES.md FAQ.md; do
        if [ -f "$file" ]; then
            if grep -qi "$FEATURE" "$file"; then
                success "$file: Documents '$FEATURE'"
            else
                error "$file: Does not document '$FEATURE'"
            fi
        fi
    done

    echo ""

    # Check examples directory
    check_examples_directory "$FEATURE"

    echo ""

else
    error "No validation target specified. Use --all or --feature NAME"
    usage
fi

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Validation Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✓ Passed:   $PASSED${NC}"
echo -e "${YELLOW}⚠ Warnings: $WARNINGS${NC}"
echo -e "${RED}✗ Failed:   $FAILED${NC}"
echo ""

# Exit code
if [ "$FAILED" -gt 0 ]; then
    echo -e "${RED}Documentation validation FAILED${NC}"
    exit 1
elif [ "$WARNINGS" -gt 0 ] && [ "$STRICT" = true ]; then
    echo -e "${YELLOW}Documentation validation FAILED (strict mode)${NC}"
    exit 1
else
    echo -e "${GREEN}Documentation validation PASSED${NC}"
    exit 0
fi
