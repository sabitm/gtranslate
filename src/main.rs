use curl::easy::Easy;
use serde_json::{Value};
use std::env;
use std::process::exit;

fn main() {
    if env::args().len() == 1 {
        println!("USAGE:");
        println!("\tgtranslate <word(s)>");
        exit(1);
    }
    let args: String = env::args().skip(1)
        .map(|mut x| {
            x.push(' ');
            x
        }).collect();
    const URL: &str = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=id&hl=en-US&dt=t&dt=bd&dj=1&source=icon&tk=316277.316277&q=";

    let mut buf = Vec::new();
    let mut easy = Easy::new();
    let enc_args = easy.url_encode(args.as_bytes());
    easy.url(&[URL, enc_args.as_str()].concat()).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
        transfer.perform().unwrap();
    }

    let res: Value = serde_json::from_slice(&buf).unwrap();
    println!("translate: {}", res["sentences"][0]["trans"].as_str().unwrap());

    if res["dict"][0]["pos"] != serde_json::Value::Null {
        for i in res["dict"].as_array().unwrap().into_iter() {
            println!("\n{}:", i["pos"].as_str().unwrap());
            for j in i["terms"].as_array().unwrap().into_iter() {
                print!("{}, ", j.as_str().unwrap());
            }
            println!("");
        }
    }
}
