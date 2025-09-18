use regex::Regex;

pub struct Tokenizer{}

impl Tokenizer{
    pub fn new() -> Self{
        Self {

        }
    }
    pub fn tokenize(&self, input: &str) -> Vec<String> {
        // Placeholder for tokenization logic
        let tokens = self.split(input);
        let output = self.shunting_yard(tokens);
        return output;
    }

    fn split(&self, input: &str) -> Vec<String> {
        let token_pattern = r#""[^"]*"|'[^']*'|\S+"#;
        let re = Regex::new(token_pattern).unwrap();
        let tokens: Vec<String> = re.find_iter(input)
            .map(|mat| mat.as_str().to_string())
            .collect();
        return tokens;
    }
    fn shunting_yard(&self, tokens: Vec<String>) -> Vec<String> {
        // Placeholder for shunting yard algorithm
        return tokens;
    }
}