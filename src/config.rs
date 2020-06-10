use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    configurable: Topic,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Topic {
    keyword_to_url: ConfigValue,
}

#[derive(Deserialize)]
struct ConfigValue {
    value: HashMap<String, String>,
}

impl Config {
    pub fn lowercased(mut self) -> Self {
        self.configurable.keyword_to_url.value = self
            .configurable
            .keyword_to_url
            .value
            .into_iter()
            .map(|(key, value)| (key.to_lowercase(), value))
            .collect();
        self
    }

    pub fn values(&self) -> &HashMap<String, String> {
        &self.configurable.keyword_to_url.value
    }
}
