use curl::easy::Easy;
use serde_json::{Value};
use std::env;

fn main() {
    std::process::exit(match translator() {
        Ok(_) => 0,
        Err(err) => err
    });
}

fn translator() -> Result<(), i32> {
    let (src_lang, trg_lang, args) = match parse_args() {
        Ok((s, t, a)) => (s, t, a),
        Err(err) => return Err(err)
    };

    const URL_FRAGMENT: (&str, &str, &str) = ("https://translate.googleapis.com/translate_a/single?client=gtx&sl=", "&tl=", "&hl=en-US&dt=t&dt=bd&dj=1&source=icon&tk=316277.316277&q=");

    let mut buf = Vec::new();
    let mut easy = Easy::new();
    let enc_args = easy.url_encode(args.as_bytes());
    easy.url(&[URL_FRAGMENT.0,
             src_lang.as_str(),
             URL_FRAGMENT.1,
             trg_lang.as_str(),
             URL_FRAGMENT.2,
             enc_args.as_str()].concat()).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
        transfer.perform().unwrap();
    }

    let res: Value = match serde_json::from_slice(&buf) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("source or target language not found");
            return Err(3)
        }
    };
    println!("translate: {}", res["sentences"][0]["trans"].as_str().unwrap());

    if res["dict"][0]["pos"] != serde_json::Value::Null {
        for i in res["dict"].as_array().unwrap().iter() {
            println!("\n{}:", i["pos"].as_str().unwrap());
            for j in i["terms"].as_array().unwrap().iter() {
                print!("{}, ", j.as_str().unwrap());
            }
            println!("");
        }
    }

    Ok(())
}

fn parse_args() -> Result<(String, String, String), i32> {
    if env::args().len() <= 5 {
        usage();
        return Err(1)
    }

    let (mut src_lang, mut trg_lang) = (String::new(), String::new());
    let mut args: Vec<String> = env::args().skip(1).collect();
    match (args[0].as_str(), args[2].as_str()) {
        ("-s", "-t") => {
            src_lang.push_str(args[1].as_str());
            trg_lang.push_str(args[3].as_str());
        },
        ("-t", "-s") => {
            trg_lang.push_str(args[1].as_str());
            src_lang.push_str(args[3].as_str());
        },
        (_, _) => {
            eprintln!("source and target language required");
            return Err(2)
        }
    };

    let args: String = args.drain(4..)
        .map(|mut x| {
            x.push(' ');
            x
        }).collect();

    Ok((src_lang, trg_lang, args))
}

fn usage() {
    println!("USAGE:");
    println!("\tgtranslate <OPTIONS> <word(s)>\n");
    println!("\tOPTIONS:");
    println!("\t-s\tspecify source language");
    println!("\t-t\tspecify target language");
}
