# Google ID token verification
[![crates.io](https://img.shields.io/crates/v/google-jwt-signin.svg)](https://crates.io/crates/google-jwt-signin)
[![documentation](https://docs.rs/google-jwt-signin/badge.svg)](https://docs.rs/google-jwt-signin)

A fork of https://crates.io/crates/google-jwt-verify focused on minimal code size.

Given a [client ID](https://developers.google.com/identity/protocols/oauth2#1.-obtain-oauth-2.0-credentials-from-the-dynamic_data.setvar.console_name-.)
and a user authorization token, verifies the token using steps described here:
https://developers.google.com/identity/gsi/web/reference/html-reference#server-side

Google's public keys are automatically fetched and cached according to the returned Cache-Control headers.
Most requests to verify a token through this library will not wait for an HTTP request.

## Features
- `blocking` (default) Uses [ureq](https://github.com/algesten/ureq)
- `async` Uses [tokio](https://tokio.rs/)

For the sake of build simplicity, this crate chooses not to support native TLS.
[ring](https://github.com/briansmith/ring) is used for SSL encryption when fetching certificates from Google
and also for RSA signature verification.
Read about the ring security audit [here](https://jbp.io/2020/06/14/rustls-audit.html).

## Quick Start
```rust
 //If you don't have a client id, get one from here: https://console.developers.google.com/
 let client_id = "37772117408-qjqo9hca513pdcunumt7gk08ii6te8is.apps.googleusercontent.com";
 let token = "...";// Obtain a signed token from Google
 let client = Client::new(&client_id);
 let id_token = client.verify_id_token(&token)?;
 
 //use the token to obtain information about the verified user
 let user_id = id_token.get_claims().get_subject();
 let email = id_token.get_payload().get_email();
 let name = id_token.get_payload().get_name();
```
