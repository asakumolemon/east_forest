
pub struct AiConfig {
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    pub temperature: f64,
    pub max_tokens: i64,
    pub top_p: f64,
}

impl AiConfig {
    pub fn new(api_key: String, api_url: String, model: String, temperature: f64, max_tokens: i64, top_p: f64) -> Self {
        Self {
            api_key,
            api_url,
            model,
            temperature,
            max_tokens,
            top_p,
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            api_key: dotenvy::var("AI_API_KEY").unwrap_or("".to_string()),
            api_url: dotenvy::var("AI_API_URL").unwrap_or("".to_string()),
            model: dotenvy::var("AI_MODEL").unwrap_or("".to_string()),
            temperature: 0.7,
            max_tokens: 4096,
            top_p: 0.3,
        }
    }
}

