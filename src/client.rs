#[cfg(feature = "blocking")]
use std::cell::RefCell;
#[cfg(feature = "async")]
use std::sync::Arc;
#[cfg(feature = "async")]
use tokio::sync::Mutex;

use crate::error::Error;
#[cfg(feature = "async")]
use crate::key_provider::AsyncKeyProvider;
use crate::key_provider::GoogleKeyProvider;
#[cfg(feature = "blocking")]
use crate::key_provider::KeyProvider;
use crate::token::IdPayload;
use crate::token::Token;
use crate::unverified_token::UnverifiedToken;
use serde::Deserialize;

#[cfg(feature = "blocking")]
pub type Client = GenericClient<RefCell<GoogleKeyProvider>>;

#[cfg(feature = "async")]
pub type TokioClient = GenericClient<Arc<Mutex<GoogleKeyProvider>>>;

pub struct GenericClientBuilder<KP> {
    client_id: String,
    key_provider: KP,
    check_expiration: bool,
}

#[cfg(feature = "blocking")]
impl<KP: Default> GenericClientBuilder<RefCell<KP>> {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_owned(),
            key_provider: RefCell::new(KP::default()),
            check_expiration: true,
        }
    }
    pub fn custom_key_provider<T>(self, provider: T) -> GenericClientBuilder<RefCell<T>> {
        GenericClientBuilder {
            client_id: self.client_id,
            key_provider: RefCell::new(provider),
            check_expiration: self.check_expiration,
        }
    }
}

#[cfg(feature = "async")]
impl<KP: Default> GenericClientBuilder<Arc<Mutex<KP>>> {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_owned(),
            key_provider: Arc::new(Mutex::new(KP::default())),
            check_expiration: true,
        }
    }
    pub fn custom_key_provider<T>(self, provider: T) -> GenericClientBuilder<Arc<Mutex<T>>> {
        GenericClientBuilder {
            client_id: self.client_id,
            key_provider: Arc::new(Mutex::new(provider)),
            check_expiration: self.check_expiration,
        }
    }
}

impl<KP> GenericClientBuilder<KP> {
    pub fn unsafe_ignore_expiration(mut self) -> Self {
        self.check_expiration = false;
        self
    }
    pub fn build(self) -> GenericClient<KP> {
        GenericClient {
            client_id: self.client_id,
            key_provider: self.key_provider,
            check_expiration: self.check_expiration,
        }
    }
}

pub struct GenericClient<T> {
    client_id: String,
    key_provider: T,
    check_expiration: bool,
}

#[cfg(feature = "blocking")]
impl<KP: Default> GenericClient<RefCell<KP>> {
    pub fn builder(client_id: &str) -> GenericClientBuilder<RefCell<KP>> {
        GenericClientBuilder::<RefCell<KP>>::new(client_id)
    }
    pub fn new(client_id: &str) -> Self {
        Self::builder(client_id).build()
    }
}

#[cfg(feature = "async")]
impl<KP: Default> GenericClient<Arc<Mutex<KP>>> {
    pub fn builder(client_id: &str) -> GenericClientBuilder<Arc<Mutex<KP>>> {
        GenericClientBuilder::<Arc<Mutex<KP>>>::new(client_id)
    }
    pub fn new(client_id: &str) -> Self {
        Self::builder(client_id).build()
    }
}

#[cfg(feature = "blocking")]
impl<KP: KeyProvider> GenericClient<RefCell<KP>> {
    pub fn verify_token_with_payload<P>(&self, token_string: &str) -> Result<Token<P>, Error>
    where
        for<'a> P: Deserialize<'a> + std::fmt::Debug,
    {
        let unverified_token =
            UnverifiedToken::<P>::validate(token_string, self.check_expiration, &self.client_id)?;
        unverified_token.verify(&self.key_provider)
    }

    pub fn verify_token(&self, token_string: &str) -> Result<Token<()>, Error> {
        self.verify_token_with_payload::<()>(token_string)
    }

    pub fn verify_id_token(&self, token_string: &str) -> Result<Token<IdPayload>, Error> {
        self.verify_token_with_payload(token_string)
    }
}

#[cfg(feature = "async")]
impl<KP: AsyncKeyProvider> GenericClient<Arc<Mutex<KP>>> {
    pub async fn verify_token_with_payload_async<P>(
        &self,
        token_string: &str,
    ) -> Result<Token<P>, Error>
    where
        for<'a> P: Deserialize<'a> + std::fmt::Debug,
    {
        let unverified_token =
            UnverifiedToken::<P>::validate(token_string, self.check_expiration, &self.client_id)?;
        unverified_token.verify_async(&self.key_provider).await
    }

    pub async fn verify_token_async(&self, token_string: &str) -> Result<Token<()>, Error> {
        self.verify_token_with_payload_async::<()>(token_string)
            .await
    }

    pub async fn verify_id_token_async(
        &self,
        token_string: &str,
    ) -> Result<Token<IdPayload>, Error> {
        self.verify_token_with_payload_async(token_string).await
    }
}
