use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Clone)]
pub struct Token<P> {
    pub claims: RequiredClaims,
    pub payload: P,
}

// https://datatracker.ietf.org/doc/html/rfc7519#section-4.1
#[derive(PartialEq, Deserialize, Debug, Clone)]
pub struct RequiredClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "sub")]
    pub subject: String,

    #[serde(rename = "aud")]
    pub audience: String,

    #[serde(rename = "exp")]
    pub expires_at: u64,

    #[serde(rename = "iat")]
    pub issued_at: u64,
}

// https://developers.google.com/identity/gsi/web/reference/html-reference#credential
#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct IdPayload {
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
    #[serde(rename = "hd")]
    pub domain: Option<String>,
}
