# flux-learner

A personal learning tool built in Rust that transforms documentation into structured study materials. Part of the `flux-` family of crates.

Point it at any web page or local file, choose an LLM, apply a learning template, and get back a study document tailored to how you learn — with progressive examples, common mistakes, compiler errors, and self-test questions.

## How It Works

1. **Source** — Provide a URL (fetched via [Firecrawl](https://firecrawl.dev)) or a local file path
2. **Template** — A system prompt that encodes your learning style and output preferences
3. **Model** — Any LLM available through [OpenRouter](https://openrouter.ai) (defaults to Claude Sonnet 4.6)
4. **Output** — A markdown study document, ready to drop into an [mdbook](https://rust-lang.github.io/mdBook/) project

## Usage

```bash
# From a web page
flux-learner \
  -t templates/deep-dive.md \
  -s https://tokio.rs/blog/2021-05-14-inventing-the-service-trait \
  -m anthropic/claude-sonnet-4.5 \
  -o my-rust-book/src/service-trait.md

# From a local file
flux-learner \
  -t templates/deep-dive.md \
  -s chapters/ownership.md \
  -o my-rust-book/src/ownership.md
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-t, --template` | Path to the system prompt template | *required* |
| `-s, --source` | URL or local file path for source material | *required* |
| `-m, --model` | OpenRouter model identifier | `anthropic/claude-sonnet-4.6` |
| `-o, --output` | Output file path (directories created automatically) | `output.md` |
| `-i, --image` | Path to an image file (requires `vision` feature) | — |

## Templates

Templates are plain markdown files that tell the LLM how to process the source material. They encode *your* learning style. Iterate on them over time as you learn what works for your brain.

### Included Templates

| Template | Purpose | Best For |
|----------|---------|----------|
| `deep-dive.md` | Comprehensive study guide with progressive examples, common mistakes, and self-test questions | Learning a new concept in depth |
| `exercises.md` | Coding exercises scoped to only the concepts in the source material, with solutions | Practicing what you've just studied |
| `architecture.md` | High-level overview of a codebase, entry points, execution flow, and how pieces connect | Understanding a framework or library |
| `quick-ref.md` | Concise reference with key types/traits table, common patterns, and gotchas | Building a cheat sheet for quick lookup |
| `explain-my-code.md` | Step-by-step explanation of what code does, why each piece is needed, and what Rust concepts it uses | Deepening understanding of code you've written |

### Example

```markdown
# templates/deep-dive.md

You are an expert in Rust. Given the following documentation, generate a study document with:

1. A concise summary of the core concept (no more than a paragraph)
2. Why this concept matters in practice
3. Three progressively complex examples, none trivial
4. Show the concept in the context of building a web server
5. Common mistakes and what the compiler errors look like
6. Questions I should be able to answer if I understand this
```

## Setup

### Prerequisites

- Rust (2024 edition)
- An [OpenRouter](https://openrouter.ai) API key
- A [Firecrawl](https://firecrawl.dev) API key (for URL scraping)

### Environment

Create a `.env` file in the project root:

```
OPENROUTER_API_KEY=your-key-here
FIRECRAWL_API_KEY=your-key-here
```

### Build

```bash
cargo build --release
```

For image/vision support:

```bash
cargo build --release --features vision
```

## Integration with mdbook

Output files are markdown, so they drop directly into an mdbook project:

```
my-rust-book/
├── book.toml
└── src/
    ├── SUMMARY.md
    ├── ownership.md      ← generated
    ├── lifetimes.md       ← generated
    └── service-trait.md   ← generated
```

## Built With

- [Rama](https://ramaproxy.org) — HTTP client
- [OpenRouter](https://openrouter.ai) — LLM gateway
- [Firecrawl](https://firecrawl.dev) — Web scraping to markdown
- [Clap](https://docs.rs/clap) — CLI argument parsing
- [Tokio](https://tokio.rs) — Async runtime

## License

MIT — see [Licence.txt](Licence.txt)