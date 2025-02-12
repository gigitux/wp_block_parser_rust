use block_parser_rust::WPBlockParser;

fn main() {
    let mut parser = WPBlockParser::new();
    let document = "<!-- wp:paragraph --><p>Hello world</p><!-- /wp:paragraph -->";
    let blocks = parser.parse(document.to_string());
}
