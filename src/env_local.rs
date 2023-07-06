use std::{collections::HashMap, env};

pub enum VariableType {
    Str(String),
    USize(usize),
    Int(i64),
}

pub struct Env {
    pub vars: HashMap<String, VariableType>,
}

pub enum EnvErr {
    MissingVar(Vec<String>),
}

const REQUIRED_VARS: [&str; 2] = ["PORT", "POOL_SIZE"];

impl Env {
    pub fn init() -> Result<Env, EnvErr> {
        let mut local_vars: HashMap<String, VariableType> = HashMap::new();
        let mut missing_vars: Vec<String> = Vec::new();

        for var in REQUIRED_VARS {
            let var_to_str = String::from(var);
            let value_result = env::var(var_to_str.clone());
            if let Ok(value) = value_result {
                match var {
                    "POOL_SIZE" => {
                        let value: usize = value.parse().unwrap();
                        local_vars.insert(var_to_str.clone(), VariableType::USize(value));
                    }
                    "PORT" => {
                        let value: i64 = value.parse().unwrap();
                        local_vars.insert(var_to_str.clone(), VariableType::Int(value));
                    }
                    _ => {
                        local_vars.insert(var_to_str.clone(), VariableType::Str(value));
                    }
                }
            } else {
                missing_vars.push(var_to_str.clone());
            }
        }

        if missing_vars.len() > 0 {
            return Err(EnvErr::MissingVar(missing_vars));
        }
        Ok(Env { vars: local_vars })
    }
}
