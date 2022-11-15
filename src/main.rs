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
use colored::{Colorize, ColoredString};      // for coloured output

const WIDTH: usize = 79;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let word = match env::args().skip(1).next() {
        Some(s) => s,
        None => panic!("Please pass a word to define."),
    };

    let (key_dict, key_thes) = match fs::read_to_string("api_key")
    {
        Ok(str) =>
            match str.split('\n').collect_tuple::<(&str, &str)>()
            {
                Some(lines) => (String::from(lines.0), String::from(lines.1)),
                None => panic!("Key file must contain 2 keys."),
            },
        Err(e) => panic!("Could not read api key file: {}", e),
    };

    // let url = format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}", word, key_dict);
    let url = format!("https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{}?key={}", word, key_thes);

    let resp = reqwest::get(url)
        .await?.json::<Value>().await?;

    let homonyms: Vec<Value> = match resp {
        Value::Array(v) => v,
        _ => panic!("Malformed respose."),
    };

    if homonyms.len() == 0 {
        println!("No definition for `{}`.", word);
    }
    else if homonyms[0].is_string() {
        println!("No definition for `{}`.", word);
        println!("Did you mean...\n\n\t{}\n\n?",
            wrap_text(
                &homonyms.iter().map(|val| val.as_str().unwrap()).join(", "),
                WIDTH - 8
            ).join("\n\t")
        );
    }
    else {
        print!("\n");
        for (i, hom) in homonyms.iter().enumerate() {
            print!("  {}.\t{}\n", i + 1, hom["fl"].as_str().unwrap().bright_purple().italic());

            for (j, def) in hom["def"][0]["sseq"].as_array().expect("Incomplete JSON").iter().enumerate() {
                print!("      -\t{}\n", process_markup(wrap_text(
                    def[0][1]["dt"][0][1].as_str().expect("Bad JSON"),
                    WIDTH - 8
                ).join("\n\t")));

                if def[0][1]["dt"][1][1][0]["t"].is_string() {
                    println!("\t{}", format!("“{}”", 
                        process_markup(wrap_text(
                            def[0][1]["dt"][1][1][0]["t"].as_str().unwrap(),
                            WIDTH - 8
                        ).join("\n\t")))
                        .truecolor(0x80, 0x80, 0x80)
                    );
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

    out.last_mut().unwrap().pop();

    return out;
}

fn process_markup(text: String) -> String
{
    // Only supports {it} tags

    let mut out = String::new();

    let mut old_start: usize = 0;
    let mut new_start: usize;

    loop {
        // Push normal until we reach an opening token
        out = format!("{}{}", out, match text[old_start..].find("{it}") {
            Some(idx) => { new_start = old_start + idx + 4; &text[old_start..old_start+idx] },
            None => { out.push_str(&text[old_start..]); break; },
        });
        old_start = new_start;

        // Push italic until we reach a closing token
        out = format!("{}{}", out, (match text[old_start..].find("{/it}") {
            Some(idx) => { new_start = old_start + idx + 5; &text[old_start..old_start+idx] },
            None => &text[old_start..],
        }).italic());
        old_start = new_start;
    }

    return out;
}