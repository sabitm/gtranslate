use std::io;
use std::net::{IpAddr, SocketAddr};
use std::sync::OnceLock;

use rsdns::clients::{std::Client, ClientConfig};
use rsdns::{records::data::A, records::Class};

pub static DNS_ADDR: OnceLock<SocketAddr> = OnceLock::new();

fn to_ioerror<T>(err: T) -> io::Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    io::Error::new(io::ErrorKind::Other, err)
}

pub fn resolve(qname: &str) -> io::Result<Vec<SocketAddr>> {
    let qname: Vec<&str> = qname.split(':').collect();
    let port: u16 = qname[1].parse().map_err(to_ioerror)?;
    let qname = qname[0];

    let nameserver = DNS_ADDR.get_or_init(|| ([8, 8, 8, 8], 53).into());
    let mut client = Client::new(ClientConfig::with_nameserver(*nameserver)).map_err(to_ioerror)?;
    let rrset = client
        .query_rrset::<A>(qname, Class::IN)
        .map_err(to_ioerror)?;

    Ok(rrset
        .rdata
        .iter()
        .map(|a| SocketAddr::new(IpAddr::V4(a.address), port))
        .collect())
}
