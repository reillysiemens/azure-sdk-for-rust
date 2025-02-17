//! Access to token credentials through various means
//!
//! Supported means currently include:
//! * The environment
//! * Azure CLI credentials cache
//! * Managed identity
//! * Client secret
mod azure_cli_credentials;
#[cfg(feature = "azureauth-cli")]
mod azureauth_cli_credentials;
mod cache;
#[cfg(feature = "client_certificate")]
mod client_certificate_credentials;
mod client_secret_credentials;
mod default_credentials;
mod environment_credentials;
mod imds_managed_identity_credentials;
mod workload_identity_credentials;

pub use azure_cli_credentials::*;
#[cfg(feature = "azureauth-cli")]
pub use azureauth_cli_credentials::*;
#[cfg(feature = "client_certificate")]
pub use client_certificate_credentials::*;
pub use client_secret_credentials::*;
pub use default_credentials::*;
pub use environment_credentials::*;
pub use imds_managed_identity_credentials::*;
pub use workload_identity_credentials::*;
