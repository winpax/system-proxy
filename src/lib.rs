use std::{
    net::{AddrParseError, IpAddr, SocketAddr},
    str::FromStr,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("AddrParse error: {0}")]
    AddrParse(#[from] AddrParseError),
    #[error("Missing system proxy configuration")]
    MissingProxyConfig,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct SystemProxy {
    pub enabled: bool,
    pub address: IpAddr,
    pub port: u16,
}

impl SystemProxy {
    pub fn get_system_proxy() -> Result<Self> {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        let key = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters")?;
        let enabled = key.get_value("EnableProxy").unwrap_or(0u32) == 1;
        let server: String = key.get_value("ProxyServer")?;

        let (host, port) = if server.is_empty() {
            return Err(Error::MissingProxyConfig);
        } else {
            let socket = SocketAddr::from_str(&server)?;

            (socket.ip(), socket.port())
        };

        Ok(Self {
            enabled,
            address: host,
            port,
        })
    }
}
