#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{traits::Vaildate, utils::is_in_scope};

#[derive(Serialize, Deserialize)]
pub struct Role {
    // role uuid
    #[serde(with = "uuid::serde::compact")]
    role_id: Uuid,
    name: String,
    model: ModelConfig,
    prompt: String,
}

impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Role {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        Role {
            role_id: uuid,
            name: uuid.to_string(),
            model: ModelConfig::default(),
            prompt: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModelConfig {
    temperature: f32,
    top_p: f32,
    max_tokens: i32,
    presence_penalty: f32,
    frequency_penalty: f32,
    #[serde(default)]
    logit_bias: HashMap<i64, f32>,
}

impl ModelConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            temperature: 0.8,
            top_p: 1.0,
            max_tokens: 4096,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            logit_bias: HashMap::new(),
        }
    }
}

// private for ModelConfig
impl Vaildate for ModelConfig {
    fn is_vaild(&self) -> Result<bool, String> {
        let mut res: bool = true;
        // check
        res &= is_in_scope(self.temperature, 0.0, 2.0, true, true);
        res &= is_in_scope(self.top_p, 0.0, 1.0, true, true);
        res &= is_in_scope(self.max_tokens, 0, 4096, true, true);
        res &= is_in_scope(self.presence_penalty, -2.0, 2.0, true, true);
        res &= is_in_scope(self.frequency_penalty, -2.0, 2.0, true, true);

        // return result
        Ok(res)
    }

    fn make_vaild(&mut self) -> Result<bool, String> {
        let is_ok: bool = true;
        self.temperature = self.temperature.clamp(0.0, 2.0);
        self.top_p = self.top_p.clamp(0.0, 1.0);
        self.max_tokens = self.max_tokens.clamp(0, 4096);
        self.presence_penalty = self.presence_penalty.clamp(-2.0, 2.0);
        self.frequency_penalty = self.frequency_penalty.clamp(-2.0, 2.0);

        // return result
        Ok(is_ok)
    }
}

#[test]
fn test_modconfig_check() {
    let mut model_config = ModelConfig::new();

    model_config.temperature = 114.0;

    assert!(!model_config.is_vaild().unwrap());

    model_config.make_vaild().unwrap();

    assert!(model_config.is_vaild().unwrap());
}
