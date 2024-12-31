use monoio::io::{AsyncReadRent, AsyncWriteRent, Split};
use monoio_rustls::ClientTlsStream;
use rustls::{
    pki_types::{DnsName, ServerName},
    ClientConfig, RootCertStore,
};
use std::net::ToSocketAddrs;

fn get_client_config() -> ClientConfig {
    let mut root_cert_store = RootCertStore::empty();

    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let cfg = rustls::ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    cfg
}

pub enum TelnetSock {
    Raw(monoio::net::TcpStream),
    Tls(ClientTlsStream<monoio::net::TcpStream>),
}

unsafe impl Split for TelnetSock {}

impl AsyncReadRent for TelnetSock {
    fn read<T: monoio::buf::IoBufMut>(
        &mut self,
        buf: T,
    ) -> impl std::future::Future<Output = monoio::BufResult<usize, T>> {
        async move {
            match self {
                TelnetSock::Raw(tcp_stream) => tcp_stream.read(buf).await,
                TelnetSock::Tls(stream) => stream.read(buf).await,
            }
        }
    }

    fn readv<T: monoio::buf::IoVecBufMut>(
        &mut self,
        buf: T,
    ) -> impl std::future::Future<Output = monoio::BufResult<usize, T>> {
        async move {
            match self {
                TelnetSock::Raw(tcp_stream) => tcp_stream.readv(buf).await,
                TelnetSock::Tls(stream) => stream.readv(buf).await,
            }
        }
    }
}

impl AsyncWriteRent for TelnetSock {
    fn write<T: monoio::buf::IoBuf>(
        &mut self,
        buf: T,
    ) -> impl std::future::Future<Output = monoio::BufResult<usize, T>> {
        async move {
            match self {
                TelnetSock::Raw(tcp_stream) => tcp_stream.write(buf).await,
                TelnetSock::Tls(stream) => stream.write(buf).await,
            }
        }
    }

    fn writev<T: monoio::buf::IoVecBuf>(
        &mut self,
        buf_vec: T,
    ) -> impl std::future::Future<Output = monoio::BufResult<usize, T>> {
        async move {
            match self {
                TelnetSock::Raw(tcp_stream) => tcp_stream.writev(buf_vec).await,
                TelnetSock::Tls(stream) => stream.writev(buf_vec).await,
            }
        }
    }

    fn flush(&mut self) -> impl std::future::Future<Output = std::io::Result<()>> {
        async move {
            match self {
                TelnetSock::Raw(tcp_stream) => tcp_stream.flush().await,
                TelnetSock::Tls(stream) => stream.flush().await,
            }
        }
    }

    fn shutdown<'a>(&'a mut self) -> impl std::future::Future<Output = std::io::Result<()>> {
        async move {
            match self {
                TelnetSock::Raw(tcp_stream) => tcp_stream.shutdown().await,
                TelnetSock::Tls(stream) => stream.shutdown().await,
            }
        }
    }
}

impl TelnetSock {
    pub async fn connect<A: ToSocketAddrs>(
        addrs: A,
        secure: bool,
    ) -> Result<TelnetSock, Box<dyn std::error::Error>> {
        let raw_tcp_stream = monoio::net::TcpStream::connect(addrs).await?;
        if secure {
            let connector = monoio_rustls::TlsConnector::from(get_client_config());

            let tls_stream = connector
                .connect(
                    ServerName::DnsName(DnsName::try_from("example.com")?),
                    raw_tcp_stream,
                )
                .await?;

            return Ok(TelnetSock::Tls(tls_stream));
        }
        Ok(TelnetSock::Raw(raw_tcp_stream))
    }
}
