# Graphrite

> **Deterministic, AI-friendly diagram syntax with powerful CLI tooling**

Graphrite is a strict subset of Mermaid designed for reliable parsing, validation, and rendering. Perfect for automated workflows, documentation generation, and anywhere you need predictable diagram behavior.

## ✨ Features

- **🎯 Deterministic parsing** - Same input always produces same output
- **🤖 AI-friendly syntax** - Simple, unambiguous grammar perfect for LLMs
- **⚡ Fast validation** - Comprehensive error checking with precise diagnostics
- **🎨 Multiple outputs** - Generate DOT, SVG, or JSON AST
- **📝 Strict subset** - Compatible with Mermaid but removes ambiguity

## 🚀 Quick Start

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.70+)

### Installation

```bash
git clone https://github.com/bwl/graphrite.git
cd graphrite
cargo build --release
```

### Basic Usage

```bash
# Validate a diagram
cargo run -p graphrite-cli -- check samples/valid/simple_flow.mmd

# Generate SVG
cargo run -p graphrite-cli -- render --format svg samples/valid/simple_flow.mmd > output.svg

# Parse to JSON AST
cargo run -p graphrite-cli -- parse samples/valid/simple_flow.mmd
```

## 📖 Syntax Example

```mermaid
%% Diagram: Simple Workflow
direction LR

start["Start Process"]
validate["Validate Input"] 
process["Process Data"]
store["Store Results"]
end_success["Success"]
end_fail["Failed"]

start --> validate
validate --> process
validate --> end_fail
process --> store
store --> end_success
```

## 🛠️ CLI Commands

| Command | Description | Example |
|---------|-------------|---------|
| `check` | Validate diagram syntax | `cargo run -p graphrite-cli -- check file.mmd` |
| `parse` | Output JSON AST | `cargo run -p graphrite-cli -- parse file.mmd` |
| `lint` | Human-readable validation | `cargo run -p graphrite-cli -- lint --pretty file.mmd` |
| `render` | Generate DOT/SVG output | `cargo run -p graphrite-cli -- render --format svg file.mmd` |
| `fmt` | Format diagram (normalize) | `cargo run -p graphrite-cli -- fmt --check file.mmd` |

### Input Sources

```bash
# From file
cargo run -p graphrite-cli -- check diagram.mmd

# From stdin
echo "direction LR\na[\"Start\"]\nb[\"End\"]\na --> b" | cargo run -p graphrite-cli -- ast
```

## 📁 Examples

- **📋 Valid samples**: [`samples/valid/`](samples/valid/) - Correct syntax examples
- **❌ Invalid samples**: [`samples/invalid/`](samples/invalid/) - Common errors and edge cases  
- **🖼️ Rendered output**: [`docs/examples/`](docs/examples/) - Generated SVG examples

## 🔍 Validation Rules

Graphrite enforces strict validation for reliable parsing:

- **Snake_case identifiers**: `my_node`, `process_step` ✅ (not `myNode`, `process-step` ❌)
- **Quoted labels**: `node["My Label"]` ✅ (not `node[My Label]` ❌)
- **Required direction**: First line must be `direction LR` or `direction TD`
- **No orphan nodes**: Every node must have at least one edge
- **No dangling edges**: All edge references must point to declared nodes
- **Line length limit**: 100 characters maximum

## 📚 Documentation

- **[Getting Started](GETTING_STARTED.md)** - Step-by-step tutorial
- **[Syntax Specification](SPEC.md)** - Complete language reference
- **[Error Codes](ERROR_CODES.md)** - Validation error reference
- **[Development Guide](CLAUDE.md)** - Contributing and architecture

## 🎯 Use Cases

- **📖 Documentation** - Generate diagrams in docs with guaranteed consistency
- **🤖 AI Workflows** - LLMs can generate reliable Graphrite syntax
- **⚙️ CI/CD Integration** - Validate diagrams in automated pipelines
- **📊 Data Visualization** - Convert structured data to visual flowcharts

## 🚦 Exit Codes

- `0` - Success
- `1` - Validation errors or parsing failures

## 📄 License

MIT License - see LICENSE file for details.

