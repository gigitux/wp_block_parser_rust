use std::collections::HashMap;

#[derive(Debug)]
pub struct WPBlock {
    name: String,
    attributes: HashMap<String, String>,
    inner_html: Vec<String>,
}

pub struct WPBlockParser {
    cursor: usize,
    blocks: Vec<WPBlock>,
}

#[derive(Debug)]
enum GrammarItem {
    StartBlock,
    End,
    EndBlock,
    StartAttribute,
    EndAttribute,
}

impl GrammarItem {
    pub fn as_str(&self) -> &'static str {
        match self {
            GrammarItem::StartBlock => "<!-- wp:",
            GrammarItem::End => "-->",
            GrammarItem::EndBlock => "<!-- /wp:",
            GrammarItem::StartAttribute => "{",
            GrammarItem::EndAttribute => "}",
        }
    }
}

#[derive(Debug)]
enum Token {
    OpenTag(String),
    CloseTag(String),
    Attribute(String),
    InnerHTML(String),
}

impl WPBlockParser {
    fn new() -> Self {
        Self {
            cursor: 0,
            blocks: Vec::new(),
        }
    }

    fn next_token(&mut self, document: &str) -> Option<Token> {
        if self.cursor >= document.len() {
            return None;
        }

        let doc = &document[self.cursor..];

        if doc.is_empty() {
            return None;
        }

        if let Some(pos) = doc.find(GrammarItem::StartBlock.as_str()) {
            self.cursor += pos + GrammarItem::StartBlock.as_str().len();
            let end = doc[self.cursor..].find(" ").unwrap_or(0);
            let block_name = &doc[self.cursor..self.cursor + end];
            // TODO: Improve this logic
            self.cursor += block_name.len();
            return Some(Token::OpenTag(block_name.to_string()));
        }

        if let Some(pos) = doc.find(GrammarItem::End.as_str()) {
            let content_start = pos + GrammarItem::End.as_str().len();
            if let Some(end_block) = doc[content_start..].find(GrammarItem::EndBlock.as_str()) {
                let inner_html = &doc[content_start..content_start + end_block].trim();
                self.cursor += content_start + end_block;
                return Some(Token::InnerHTML(inner_html.to_string()));
            }
        }

        if let Some(_pos) = doc.find(GrammarItem::EndBlock.as_str()) {
            let end = doc.find("-->").unwrap_or(0);
            // TODO: Improve this logic
            self.cursor += end + 4;
            return Some(Token::CloseTag(String::new()));
        }

        if let Some(pos) = doc.find(GrammarItem::StartAttribute.as_str()) {
            self.cursor += pos + GrammarItem::StartAttribute.as_str().len();
            let end = doc[self.cursor..]
                .find(GrammarItem::EndAttribute.as_str())
                .unwrap_or(0);
            let attribute = &doc[self.cursor..self.cursor + end];
            self.cursor += end + 1;
            return Some(Token::Attribute(attribute.to_string()));
        }

        None
    }

    pub fn parse(&mut self, document: String) -> &Vec<WPBlock> {
        self.cursor = 0;
        self.blocks = Vec::new();

        let mut current_block = WPBlock {
            name: String::new(),
            attributes: HashMap::new(),
            inner_html: Vec::new(),
        };

        while let Some(token) = self.next_token(&document) {
            match token {
                Token::OpenTag(block_name) => {
                    current_block.name = block_name;
                }
                Token::Attribute(attribute) => {
                    println!("attribute: {}", attribute);
                    let key = attribute.split("=").next().unwrap();
                    let value = attribute.split("=").nth(1).unwrap();
                    current_block
                        .attributes
                        .insert(key.to_string(), value.to_string());
                }
                Token::CloseTag(block_name) => {
                    // TODO: Implement logic to implement inner blocks
                    self.blocks.push(current_block);
                    current_block = WPBlock {
                        name: String::new(),
                        attributes: HashMap::new(),
                        inner_html: Vec::new(),
                    };
                }
                Token::InnerHTML(html) => {
                    current_block.inner_html.push(html);
                }
            }
        }
        &self.blocks
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn it_parses_a_simple_block() {
    let document = "<!-- wp:paragraph --><p>Hello world</p><!-- /wp:paragraph -->";
    let mut parser = WPBlockParser::new();
    let res = parser.parse(document.to_string());
    println!("res: {:?}", res);
    assert_eq!(res[0].name, "paragraph");
    assert_eq!(res[0].inner_html[0], "<p>Hello world</p>");
}
