#!/bin/bash
# generate-doc.sh - Generate documentation from templates
#
# Usage: ./generate-doc.sh [OPTIONS]
#
# Options:
#   --type <detector|feature|command|integration>
#   --name <name>
#   --version <version>
#   --author <name>
#   --output <path>
#   --help

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
TYPE=""
NAME=""
VERSION="1.6.0"
AUTHOR=$(git config user.name 2>/dev/null || echo "Unknown")
OUTPUT=""
TEMPLATE_DIR=".doc-templates"
DATE=$(date +%Y-%m-%d)

# Helper functions
error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Usage information
usage() {
    cat << EOF
Documentation Generator for MCP Sentinel

Usage: $(basename "$0") [OPTIONS]

Options:
    --type TYPE        Type of documentation (detector|feature|command|integration)
    --name NAME        Name of the feature/detector (kebab-case)
    --version VERSION  Version number (default: $VERSION)
    --author AUTHOR    Author name (default: $AUTHOR)
    --output PATH      Output path (default: auto-generated)
    --help             Show this help message

Examples:
    # Generate detector documentation
    $(basename "$0") --type detector --name xss-detection

    # Generate feature documentation
    $(basename "$0") --type feature --name real-time-monitoring

    # Generate with custom version
    $(basename "$0") --type detector --name csrf-detection --version 2.0.0

EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --type)
            TYPE="$2"
            shift 2
            ;;
        --name)
            NAME="$2"
            shift 2
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --author)
            AUTHOR="$2"
            shift 2
            ;;
        --output)
            OUTPUT="$2"
            shift 2
            ;;
        --help)
            usage
            ;;
        *)
            error "Unknown option: $1\nUse --help for usage information"
            ;;
    esac
done

# Validate required arguments
if [ -z "$TYPE" ]; then
    error "Missing required argument: --type"
fi

if [ -z "$NAME" ]; then
    error "Missing required argument: --name"
fi

# Validate type
case $TYPE in
    detector|feature|command|integration)
        ;;
    *)
        error "Invalid type: $TYPE. Must be one of: detector, feature, command, integration"
        ;;
esac

# Convert name to different formats
NAME_KEBAB="$NAME"  # kebab-case (input format)
NAME_SNAKE=$(echo "$NAME" | tr '-' '_')  # snake_case
NAME_PASCAL=$(echo "$NAME" | awk -F'-' '{for(i=1;i<=NF;i++) printf "%s", toupper(substr($i,1,1)) tolower(substr($i,2))}')  # PascalCase
NAME_TITLE=$(echo "$NAME" | sed 's/-/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2))}1')  # Title Case
NAME_UPPER=$(echo "$NAME" | tr '[:lower:]-' '[:upper:]_')  # UPPER_SNAKE_CASE

# Template file
TEMPLATE_FILE="$TEMPLATE_DIR/${TYPE^^}.md.template"

if [ ! -f "$TEMPLATE_FILE" ]; then
    error "Template file not found: $TEMPLATE_FILE"
fi

# Output file
if [ -z "$OUTPUT" ]; then
    OUTPUT="${TYPE}-${NAME_KEBAB}.md"
fi

info "Generating $TYPE documentation for '$NAME_TITLE'..."

# Create output from template
cp "$TEMPLATE_FILE" "$OUTPUT"

# Replace placeholders
sed -i "s/{{DETECTOR_NAME}}/$NAME_TITLE/g" "$OUTPUT"
sed -i "s/{{FEATURE_NAME}}/$NAME_TITLE/g" "$OUTPUT"
sed -i "s/{{COMMAND_NAME}}/$NAME_KEBAB/g" "$OUTPUT"
sed -i "s/{{VERSION}}/$VERSION/g" "$OUTPUT"
sed -i "s/{{AUTHOR}}/$AUTHOR/g" "$OUTPUT"
sed -i "s/{{DATE}}/$DATE/g" "$OUTPUT"
sed -i "s/{{DETECTOR_ID}}/$NAME_KEBAB/g" "$OUTPUT"
sed -i "s/{{FEATURE_ID}}/$NAME_KEBAB/g" "$OUTPUT"
sed -i "s/{{FEATURE_NAME_UPPER}}/$NAME_UPPER/g" "$OUTPUT"
sed -i "s/{{COMMAND}}/$NAME_KEBAB/g" "$OUTPUT"

success "Created documentation template: $OUTPUT"

# Add sections to existing documentation
info "Adding sections to existing documentation..."

# Function to add section to doc
add_section_to_doc() {
    local doc_file=$1
    local section_title=$2
    local section_content=$3

    if [ ! -f "$doc_file" ]; then
        warning "Documentation file not found: $doc_file (skipping)"
        return
    fi

    # Check if section already exists
    if grep -q "### $section_title" "$doc_file" 2>/dev/null; then
        warning "Section already exists in $doc_file (skipping)"
        return
    fi

    # Add section
    echo -e "\n### $section_title\n\n$section_content\n" >> "$doc_file"
    success "Added section to $doc_file"
}

# Add to USER_GUIDE.md
if [ "$TYPE" = "detector" ]; then
    add_section_to_doc "USER_GUIDE.md" "$NAME_TITLE Detector" \
"**Purpose:** [Add description]

**Patterns detected:**
- [Pattern 1]
- [Pattern 2]

**Severity:** [Critical/High/Medium/Low]

**CWE:** [CWE-XXX]

**Example:**
\`\`\`python
# Example vulnerable code
\`\`\`

See [${type}-${NAME_KEBAB}.md](${type}-${NAME_KEBAB}.md) for complete documentation."
fi

# Add to EXAMPLES.md
add_section_to_doc "EXAMPLES.md" "$NAME_TITLE Example" \
"**Scenario:** [Describe scenario]

**Vulnerable code:**
\`\`\`python
# Add example
\`\`\`

**Fixed code:**
\`\`\`python
# Add fixed version
\`\`\`

See [${TYPE}-${NAME_KEBAB}.md](${TYPE}-${NAME_KEBAB}.md) for more examples."

# Add to TROUBLESHOOTING.md
add_section_to_doc "TROUBLESHOOTING.md" "$NAME_TITLE Issues" \
"**Issue:** [Common issue]

**Symptoms:**
[Description]

**Solution:**
\`\`\`bash
# Fix commands
\`\`\`"

# Add to FAQ.md
add_section_to_doc "FAQ.md" "$NAME_TITLE FAQ" \
"### How does $NAME_TITLE work?

[Answer]

### When should I use $NAME_TITLE?

[Answer]"

# Create example directory
EXAMPLE_DIR="examples/${NAME_KEBAB}"
if [ ! -d "$EXAMPLE_DIR" ]; then
    mkdir -p "$EXAMPLE_DIR"

    # Create example files
    cat > "$EXAMPLE_DIR/vulnerable.py" << 'EOF'
# Vulnerable code example
# TODO: Add actual vulnerable code

def vulnerable_function():
    # This code has security issues
    pass
EOF

    cat > "$EXAMPLE_DIR/fixed.py" << 'EOF'
# Fixed code example
# TODO: Add actual fixed code

def secure_function():
    # This code is secure
    pass
EOF

    cat > "$EXAMPLE_DIR/README.md" << EOF
# $NAME_TITLE Examples

## Vulnerable Code

See [vulnerable.py](vulnerable.py) for examples of vulnerable code that MCP Sentinel detects.

## Fixed Code

See [fixed.py](fixed.py) for secure implementations.

## Scanning

\`\`\`bash
# Scan vulnerable code
mcp-sentinel scan vulnerable.py

# Should find: [X] issues

# Scan fixed code
mcp-sentinel scan fixed.py

# Should find: 0 issues
\`\`\`
EOF

    success "Created example directory: $EXAMPLE_DIR"
fi

# Create test file
TEST_FILE="tests/detectors/${NAME_SNAKE}_test.rs"
if [ "$TYPE" = "detector" ] && [ ! -f "$TEST_FILE" ]; then
    mkdir -p "tests/detectors"

    cat > "$TEST_FILE" << EOF
// Tests for $NAME_TITLE Detector

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_${NAME_SNAKE}_basic() {
        // TODO: Add test
        assert!(true);
    }

    #[test]
    fn test_${NAME_SNAKE}_false_positive() {
        // TODO: Add test for known false positive
        assert!(true);
    }
}
EOF

    success "Created test file: $TEST_FILE"
fi

# Summary
echo ""
info "Documentation generation complete!"
echo ""
echo "📝 Files created/updated:"
echo "  - $OUTPUT (main documentation)"
echo "  - USER_GUIDE.md (section added)"
echo "  - EXAMPLES.md (section added)"
echo "  - TROUBLESHOOTING.md (section added)"
echo "  - FAQ.md (section added)"
if [ -d "$EXAMPLE_DIR" ]; then
    echo "  - $EXAMPLE_DIR/ (examples)"
fi
if [ -f "$TEST_FILE" ]; then
    echo "  - $TEST_FILE (tests)"
fi
echo ""
echo "📋 Next steps:"
echo "  1. Edit $OUTPUT and fill in all TODO sections"
echo "  2. Add real code examples to $EXAMPLE_DIR/"
echo "  3. Implement the feature/detector"
echo "  4. Write tests in $TEST_FILE"
echo "  5. Run: ./scripts/docs/validate-docs.sh --feature '$NAME_TITLE'"
echo "  6. Update CHANGELOG.md"
echo "  7. Update README.md feature list"
echo ""
warning "Remember: Documentation is not optional! Complete all sections before marking as done."
echo ""
