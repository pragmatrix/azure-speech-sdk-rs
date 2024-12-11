use url::Url;

#[derive(Clone, Debug)]
/// Auth struct, used to authenticate with Azure Speech Services.
pub enum Auth {
    Subscription { region: String, key: String },
    Host { host: Url, key: String },
}
impl Auth {
    /// Create a new Auth instance from a subscription key and a region.
    pub fn from_subscription(region: impl Into<String>, key: impl Into<String>) -> Self {
        Auth::Subscription {
            region: region.into(),
            key: key.into(),
        }
    }

    // TODO: is the key even required here?
    pub fn from_host(host: impl Into<Url>, key: impl Into<String>) -> Self {
        Auth::Host {
            host: host.into(),
            key: key.into(),
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Auth::Subscription { key, .. } => key,
            Auth::Host { key, .. } => key,
        }
    }
}
