//! This crate serves as the CLI for the dot client
//! (which, for the moment, are one in ths same, until
//! a server is established and made accessible through
//! curl/ssh/web/etc.). The dot network project will be
//! an experimental SSH/cURL-based console service which
//! users can sign up for and experience as a contiuous
//! multiuser social and gaming network, all from the
//! comfort of their terminal. To use the client:
//! ```
//! use dot_client::{DotClient, Credentials, ApiKey};
//!
//! let api_key = ApiKey::new("<API KEY>");
//! let credentials = Credentials::from_username("<USERNAME>")
//!     .with_key(api_key);
//!
//! let connection = DotClient::connect(&credentials).await?;
//! ```
//!
//! For now, the CLI, client, and actual service (to be accessed
//! over SSH and/or CURL) are contained in this monocrate as it's
//! being developed and tested locally, but as I start to set
//! this up and deploy to a web server for remote access, the
//! three aforementioned crates will be split up.
//!
pub mod client;
pub mod fmt;
pub mod cli;
pub mod config;
pub mod terminal;
pub mod error;

pub use error::{DotCliError, DotCliResult};
pub use client::{DotClient, DotClientOptions};
