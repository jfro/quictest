use rustls::client::{ServerCertVerified, ServerCertVerifier};
use rustls::internal::msgs::base::PayloadU16;
use rustls::server::{ClientCertVerified, ClientCertVerifier};
use rustls::{Certificate, DistinguishedNames, Error as TLSError, ServerName};
use std::time::SystemTime;

pub struct ServerVerifier;
impl ServerVerifier {
    pub fn new() -> Self {
        ServerVerifier {}
    }
}
impl ServerCertVerifier for ServerVerifier {
    fn verify_server_cert<'a>(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: SystemTime,
    ) -> Result<ServerCertVerified, TLSError> {
        Ok(ServerCertVerified::assertion())
    }
}

pub struct ClientVerifier;
impl ClientVerifier {
    pub fn new() -> Self {
        ClientVerifier {}
    }
}
impl ClientCertVerifier for ClientVerifier {
    fn client_auth_root_subjects(&self) -> Option<DistinguishedNames> {
        let names = b"localhost";
        Some(vec![PayloadU16::new(names.to_vec())])
    }

    fn verify_client_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _now: SystemTime,
    ) -> Result<ClientCertVerified, TLSError> {
        Ok(ClientCertVerified::assertion())
    }
}
