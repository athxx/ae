use std::io::{BufRead, BufReader};
use std::{collections::HashMap, env, fs::File};

pub fn env_bool(key: &str) -> bool {
    match env::var(key) {
        Ok(v) => v == "true" || v == "TRUE",
        Err(_) => false,
    }
}

pub fn env_str(key: &str) -> String {
    match env::var(key) {
        Ok(v) => v,
        Err(_) => "".to_owned(),
    }
}

pub fn env_i32(key: &str) -> i32 {
    match env::var(key) {
        Ok(v) => v.parse().unwrap_or(0),
        Err(_) => 0,
    }
}

pub fn env_i64(key: &str) -> i64 {
    match env::var(key) {
        // Ok(v) => i64::from_str(v.as_str()).unwrap_or(0),
        Ok(v) => v.parse::<i64>().unwrap_or(0),
        Err(_) => 0,
    }
}

pub fn env_usize(key: &str) -> usize {
    match env::var(key) {
        Ok(v) => v.parse::<usize>().unwrap_or(0),
        Err(_) => 0,
    }
}

/// it just for get all prefix 'ox_' environment variables
/// if necessary just change filter rules
pub fn env_map() -> HashMap<String, String> {
    env::vars().filter(|(k, _)| k.len() > 3 && &k[0..3] == "ox_").collect()
}

pub fn load_env() -> Result<HashMap<String, String>, std::io::Error> {
    let mut env_vars = HashMap::new();

    // Open the .env file
    let file = File::open(".env")?;

    // Read the contents of the file into a BufReader
    let reader = BufReader::new(file);

    // Parse each line of the file and add the variables to the HashMap
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            env_vars.insert(key, value);
        }
    }

    // Set the environment variables
    for (key, val) in env_vars.iter() {
        env::set_var(key, val);
    }

    Ok(env_vars)
}
