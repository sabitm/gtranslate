use std::fmt;
use std::net::{IpAddr, SocketAddr};
use std::sync::OnceLock;

use rsdns::clients::{std::Client, ClientConfig};
use rsdns::{records::data::A, records::Class};
use ureq::Error;
use ureq::config::Config;
use ureq::http::Uri;
use ureq::unversioned::resolver::{Resolver, ResolvedSocketAddrs};
use ureq::unversioned::transport::NextTimeout;

pub static DNS_ADDR: OnceLock<SocketAddr> = OnceLock::new();

#[derive(Debug)]
pub struct CustomResolver;

impl Resolver for CustomResolver {
    fn resolve(
        &self,
        uri: &Uri,
        _config: &Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, Error> {
        let authority = uri.authority().ok_or(Error::HostNotFound)?;
        let host = authority.host();
        let port = authority
            .port_u16()
            .or_else(|| match uri.scheme_str() {
                Some("https") => Some(443),
                Some("http") => Some(80),
                _ => None,
            })
            .ok_or(Error::HostNotFound)?;

        let nameserver = DNS_ADDR.get_or_init(|| ([8, 8, 8, 8], 53).into());
        let mut client =
            Client::new(ClientConfig::with_nameserver(*nameserver)).map_err(to_ioerror)?;
        let rrset = client
            .query_rrset::<A>(host, Class::IN)
            .map_err(to_ioerror)?;

        let placeholder: SocketAddr = "0.0.0.0:0".parse().unwrap();
        let mut result = ResolvedSocketAddrs::from_fn(|_| placeholder);
        for a in &rrset.rdata {
            result.push(SocketAddr::new(IpAddr::V4(a.address), port));
        }

        if result.is_empty() {
            return Err(Error::HostNotFound);
        }

        Ok(result)
    }
}

fn to_ioerror<T>(err: T) -> std::io::Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

impl fmt::Display for CustomResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomResolver")
    }
}
