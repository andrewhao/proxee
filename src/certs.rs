use rcgen::generate_simple_self_signed;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs;
use std::io::BufReader;

pub struct SignedCert {
    pub certificate: String,
    pub private_key: String,
}

pub fn tls_acceptor() -> tokio_rustls::TlsAcceptor {
    use std::sync::Arc;
    use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};

    let keys_file = fs::File::open(".proxee/server.key").unwrap();
    let mut keys_reader = BufReader::new(keys_file);
    let parsed_keys = pkcs8_private_keys(&mut keys_reader).unwrap();
    let key_vec = parsed_keys.first().unwrap();

    let cert_file = fs::File::open(".proxee/server.crt").unwrap();
    let mut cert_reader = BufReader::new(cert_file);
    let parsed_certs = certs(&mut cert_reader).unwrap();
    let cert_vec = parsed_certs.first().unwrap();
    println!("parsed cert: {:?}", cert_vec);

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

pub fn generate_and_save(hostnames: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let result = generate_self_signed_cert(hostnames).unwrap();
    fs::create_dir_all(".proxee").unwrap();
    fs::write(".proxee/server.crt", result.certificate).unwrap();
    fs::write(".proxee/server.key", result.private_key).unwrap();
    Ok(())
}

fn generate_self_signed_cert(
    hostnames: Vec<String>,
) -> Result<SignedCert, Box<dyn std::error::Error>> {
    let cert = generate_simple_self_signed(hostnames).unwrap();
    let result = SignedCert {
        private_key: cert.serialize_private_key_pem(),
        certificate: cert.serialize_pem().unwrap(),
    };
    return Ok(result);
}
