use std::sync::OnceLock;

use azure_speech::Connector;

pub fn native_tls_connector() -> &'static Connector {
    static CONNECTOR: OnceLock<Connector> = OnceLock::new();

    CONNECTOR.get_or_init(|| {
        let connector = tokio_native_tls::native_tls::TlsConnector::new()
            .expect("to create native TLS connector");
        Connector::NativeTls(tokio_native_tls::TlsConnector::from(connector))
    })
}
