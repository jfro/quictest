use anyhow::Result;
use rustls::{Certificate, PrivateKey};

pub fn generate_cert() -> Result<(Certificate, PrivateKey)> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    Ok((
        Certificate(cert.serialize_der()?),
        PrivateKey(cert.serialize_private_key_der()),
    ))
}
