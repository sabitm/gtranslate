use anyhow::{bail, Result};
use std::env;
use ureq;

fn main() -> Result<()> {
    Ok(translator()?)
}

fn translator() -> Result<()> {
    let (src_lang, trg_lang, args) = parse_args()?;

    let url = [
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=",
        src_lang.as_str(),
        "&tl=",
        trg_lang.as_str(),
        "&hl=en-US&dt=t&dt=bd&dj=1&source=icon&tk=316277.316277&q=",
        args.as_str(),
    ]
    .concat();

    let res: serde_json::Value = ureq::get(url.as_str()).call()?.into_json()?;

    print!("translate: ");
    for s in res["sentences"].as_array().unwrap().iter() {
        print!("{}", s["trans"].as_str().unwrap());
    }
    println!();

    if res["dict"][0]["pos"] != serde_json::Value::Null {
        for i in res["dict"].as_array().unwrap().iter() {
            println!("\n{}:", i["pos"].as_str().unwrap());
            for j in i["terms"].as_array().unwrap().iter() {
                print!("{}, ", j.as_str().unwrap());
            }
            println!();
        }
    }

    Ok(())
}

fn parse_args() -> Result<(String, String, String)> {
    if env::args().len() <= 5 {
        usage();
        bail!("Not enough argument!");
    }

    let (mut src_lang, mut trg_lang) = (String::new(), String::new());
    let mut args: Vec<String> = env::args().skip(1).collect();
    match (args[0].as_str(), args[2].as_str()) {
        ("-s", "-t") => {
            src_lang.push_str(args[1].as_str());
            trg_lang.push_str(args[3].as_str());
        }
        ("-t", "-s") => {
            trg_lang.push_str(args[1].as_str());
            src_lang.push_str(args[3].as_str());
        }
        (_, _) => {
            bail!("Source and target language required");
        }
    };

    let args: String = args
        .drain(4..)
        .map(|mut x| {
            x.push(' ');
            x
        })
        .collect();

    Ok((src_lang, trg_lang, args))
}

fn usage() {
    println!("USAGE:");
    println!("\tgtranslate <OPTIONS> <word(s)>\n");
    println!("\tOPTIONS:");
    println!("\t-s\tspecify source language");
    println!("\t-t\tspecify target language");
}
