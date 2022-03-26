use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs;
use std::io::BufReader;
use std::sync::Arc;
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};

pub fn tls_acceptor(key_path: String, cert_path: String) -> tokio_rustls::TlsAcceptor {
    let keys_file = fs::File::open(key_path).unwrap();
    let mut keys_reader = BufReader::new(keys_file);
    let parsed_keys = pkcs8_private_keys(&mut keys_reader).unwrap();
    let key_vec = parsed_keys.first().unwrap();

    let cert_file = fs::File::open(cert_path).unwrap();
    let mut cert_reader = BufReader::new(cert_file);
    let parsed_certs = certs(&mut cert_reader).unwrap();
    let cert_vec = parsed_certs.first().unwrap();

    let key = PrivateKey(key_vec.clone());
    let cert = Certificate(cert_vec.clone());

    Arc::new(
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .unwrap(),
    )
    .into()
}

