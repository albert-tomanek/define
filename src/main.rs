#[allow(unused_parens)]

// TODO:
//  - Add a simultaneous async call to the thesaurus
//  - Add pronounciation
//  - Add word type
//  - For 'did you mean', highlight spelling differences.

use std::{fs, env};
use reqwest;
use serde_json::{Value};
use itertools::Itertools;   // for join on iterators
use colored::Colorize;      // for coloured output

const WIDTH: usize = 79;

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
            wrap_text(
                &defs.iter().map(|val| val.as_str().unwrap()).join(", "),
                WIDTH - 8
            ).join("\n\t")
        );
    }
    else {
        print!("\n");
        for (i, def) in defs.iter().enumerate() {
            print!("  {}.\t{}\n", i + 1, def["fl"].as_str().unwrap().bright_purple().italic());

            for (j, expl) in def["shortdef"].as_array().unwrap().iter().enumerate() {
                /*if j == 0 {
                    print!("{}\n", wrap_text(
                        expl.as_str().unwrap(),
                        WIDTH - 8
                    ).join("\n\t"));
                }
                else*/ {
                    print!("      -\t{}\n", wrap_text(
                        expl.as_str().unwrap(),
                        WIDTH - 8
                    ).join("\n\t"));
                }
            }

            print!("\n");
        }
    }

    Ok(())
}

fn wrap_text(text: &str, width: usize) -> Vec<String>
{
    let mut out = vec![String::new()];

    for word in text.split(' ')
    {
        if out.last().unwrap().len() + word.len() > width
        {
            out.push(String::new());
        }

        out.last_mut().unwrap().push_str(word);
        out.last_mut().unwrap().push(' ');
    }

    return out;
}
