use std::error::Error;
use crate::helpers::make_post_request;
use std::env;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_API_KEY: &str = "ANTHROPIC_API_KEY";

pub trait LlmInterface {
    fn make_call(&self, prompt: String) -> Result<String, Box<dyn Error>>;
}

struct Claude {
    extra_headers: Vec<String>,
    model: String,
    max_tokens: usize,
}

// curl https://api.anthropic.com/v1/messages \
// --header "x-api-key: $ANTHROPIC_API_KEY" \
// --header "anthropic-version: 2023-06-01" \
// --header "content-type: application/json" \
// --data \
// '{
// "model": "claude-3-5-sonnet-20241022",
// "max_tokens": 1024,
// "messages": [
// {"role": "user", "content": "Hello, world"}
// ]
// }'

impl Claude {
    pub fn new() -> Self {
        let api_key = match env::var(ANTHROPIC_API_KEY) {
            Ok(x) => {x}
            Err(_) => {
                panic!("ANTHROPIC API KEY NOT FOUND")
            }
        };

        Self {
            extra_headers: vec![
                format!("x-api-key: {}", api_key),
                "anthropic-version: 2023-06-01".to_string(),
            ],
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 1024,
        }
    }
}

impl LlmInterface for Claude {
    fn make_call(self, prompt: String) -> Result<String, Box<dyn Error>> {
        let data = prompt.as_str();
        let output = make_post_request(ANTHROPIC_API_URL, data, Some(self.extra_headers))?;
        Ok(output)
    }
}

