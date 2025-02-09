# WordPress Block Parser

A Rust implementation of a parser for WordPress Gutenberg blocks. This project provides a simple and efficient way to parse WordPress block markup (also known as "WordPress Block Grammar") into structured data.

This project was created to learn Rust programming concepts while working with real-world parsing problems. 

**Note: This is not intended for production use.**

## Overview

The parser can process WordPress block markup and extract:

- Block names
- Block attributes
- Inner HTML content

## Example Usage

```rust
use wp_block_parser::{parse_blocks, WPBlock};

let document = "<!-- wp:paragraph --><p>Hello world</p><!-- /wp:paragraph -->";
let mut parser = WPBlockParser::new();
let blocks = parser.parse(document.to_string());

```

## Block Grammar

The parser understands the following WordPress block grammar elements:

- Block delimiters (`<!-- wp:blockname -->`)
- Block closing tags (`<!-- /wp:blockname -->`)
- Block attributes in JSON format
- Inner HTML content

## Project Structure

- `WPBlock`: Represents a parsed WordPress block with its name, attributes, and inner HTML content
- `WPBlockParser`: The main parser implementation that tokenizes and processes block markup
- `GrammarItem`: Enum defining the various grammar elements in WordPress block syntax
- `Token`: Enum representing the different types of tokens that can be parsed

## Current Features

- Parse basic WordPress blocks
- Extract block names and inner HTML
- Support for block attributes

## Todo

- [ ] Implement nested block parsing
- [ ] Improve attribute parsing logic
- [ ] Add more comprehensive error handling
- [ ] Support for more complex block structures

## License

MIT

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

