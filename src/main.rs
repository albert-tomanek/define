#[allow(unused_parens)]

use std::{fs, env};
use reqwest;
use serde_json::{Value};
use itertools::Itertools;   // for join on iterators

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let word = match env::args().skip(1).next() {
        Some(s) => s,
        None => panic!("Please pass a word to define."),
    };

    let key = match fs::read_to_string("api_key")
    {
        Ok(str) => 
            match str.split('\n').next()
            {
                Some(line) => String::from(line),
                None => panic!("Empty key file."),
            },
        Err(e) => panic!("Could not read api key file: {}", e),
    };

    let url = format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}", word, key);

    let resp = reqwest::get(url)
        .await?.json::<Value>().await?;

    let defs: Vec<Value> = match resp {
        Value::Array(v) => v,
        _ => panic!("Malformed respose."),
    };

    if defs.len() == 0 {
        println!("No definition for `{}`.", word);
    }
    else if defs[0].is_string() {
        println!("No definition for `{}`.", word);
        println!("Did you mean...\n\n\t{}\n\n?",
            defs.iter().map(|val| val.as_str().unwrap()).join(", ")
        );
    }
    else {
        for (i, def) in defs.iter().enumerate() {
            print!(" {}.\t", i + 1);
            
            for (j, expl) in def["shortdef"].as_array().unwrap().iter().enumerate() {
                if j == 0 {
                    print!("{}\n", expl.as_str().unwrap_or_else(|| {panic!("Unexpected JSON format")}));
                }
                else {
                    print!(" \t{}\n", expl.as_str().unwrap_or_else(|| {panic!("Unexpected JSON format")}));
                }
            }

            print!("\n");
        }
    }

    Ok(())
}
