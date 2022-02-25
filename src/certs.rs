use rcgen::generate_simple_self_signed;
use std::fs;

pub struct SignedCert {
    pub certificate: String,
    pub private_key: String,
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
