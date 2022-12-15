use std::io;
use std::net::{IpAddr, SocketAddr};

use anyhow::{bail, Result};
use rsdns::clients::{std::Client, ClientConfig};
use rsdns::{constants::Class, records::data::A};
use ureq;

fn to_ioerror<T>(err: T) -> io::Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    io::Error::new(io::ErrorKind::Other, err)
}

fn resolve(qname: &str) -> io::Result<Vec<SocketAddr>> {
    let qname: Vec<&str> = qname.split(':').collect();
    let port: u16 = qname[1].parse().map_err(to_ioerror)?;
    let qname = qname[0];
    let nameserver: SocketAddr = ([8, 8, 8, 8], 53).into();
    let mut client = Client::new(ClientConfig::with_nameserver(nameserver)).map_err(to_ioerror)?;
    let rrset = client
        .query_rrset::<A>(qname, Class::In)
        .map_err(to_ioerror)?;

    Ok(rrset
        .rdata
        .iter()
        .map(|a| SocketAddr::new(IpAddr::V4(a.address), port))
        .collect())
}

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
        repeated words: String
    };

    if flags.words.is_empty() {
        bail!("flag is required: `<words>`");
    }

    let url = [
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=",
        flags.source.as_str(),
        "&tl=",
        flags.target.as_str(),
        "&hl=en-US&dt=t&dt=bd&dj=1&source=icon&tk=316277.316277&q=",
        flags.words.join(" ").as_str(),
    ]
    .concat();

    let agent = ureq::builder().resolver(resolve).build();
    let resp: serde_json::Value = agent.get(url.as_str()).call()?.into_json()?;

    print!("translate: ");
    for s in resp["sentences"].as_array().unwrap().iter() {
        print!("{}", s["trans"].as_str().unwrap());
    }
    println!();

    if resp["dict"][0]["pos"] != serde_json::Value::Null {
        for i in resp["dict"].as_array().unwrap().iter() {
            println!("\n{}:", i["pos"].as_str().unwrap());
            for j in i["terms"].as_array().unwrap().iter() {
                print!("{}, ", j.as_str().unwrap());
            }
            println!();
        }
    }

    Ok(())
}
