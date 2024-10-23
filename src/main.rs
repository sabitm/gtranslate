mod helper;
mod response;

use std::net::SocketAddr;

use anyhow::{bail, Result};

use crate::response::TransResponse;

fn main() -> Result<()> {
    translator()
}

fn translator() -> Result<()> {
    let flags = xflags::parse_or_exit! {
        /// Source language that the program will translate from
        required -s,--source lang: String
        /// Target language that the program will translate to
        required -t,--target lang: String
        /// Optional DNS configuration, default if empty: 8.8.8.8
        optional --dns dns: String
        /// Word, sentences that will be translated
        repeated words: String
    };

    if flags.words.is_empty() {
        bail!("flag is required: `<words>`");
    }

    let url = [
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=",
        &flags.source,
        "&tl=",
        &flags.target,
        "&hl=en-US&dt=t&dt=bd&dj=1&source=icon&tk=316277.316277&q=",
        &flags.words.join(" "),
    ]
    .concat();

    let mut agent = ureq::builder();

    // Set DNS if given argument
    if let Some(dns) = flags.dns {
        if !dns.is_empty() {
            // Check if dns port is given
            let mut dns = dns;
            if !dns.contains(':') {
                dns += ":53";
            }

            let sock_addr = dns.parse::<SocketAddr>()?;
            if let Err(v) = helper::DNS_ADDR.set(sock_addr) {
                bail!("failed to set DNS addr: {}", v);
            }
        }
        agent = agent.resolver(helper::resolve);
    }

    let agent = agent.build();
    let resp: TransResponse = agent.get(&url).call()?.into_json()?;

    // Debugging purpose
    // let resp: String = agent.get(&url).call()?.into_string()?;
    // println!("{}", &resp);

    print!("translate: ");
    for s in resp.sentences {
        print!("{}", s.trans);
    }
    println!();

    if let Some(dicts) = resp.dict {
        for dict in dicts {
            println!("\n{}:", dict.pos);
            for entry in dict.entry {
                print!("{}, ", entry.word);
            }
            println!();
        }
    }

    Ok(())
}
