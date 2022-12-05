use anyhow::Result;
use ureq;

fn main() -> Result<()> {
    Ok(translator()?)
}

fn translator() -> Result<()> {
    let flags = xflags::parse_or_exit! {
        /// Source language that the program will translate from
        required -s,--source lang: String
        /// Target language that the program will translate to
        required -t,--target lang: String
        /// Word, sentences that will be translated
        required words: String
    };

    let url = [
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=",
        flags.source.as_str(),
        "&tl=",
        flags.target.as_str(),
        "&hl=en-US&dt=t&dt=bd&dj=1&source=icon&tk=316277.316277&q=",
        flags.words.as_str(),
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
