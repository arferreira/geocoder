use thiserror::Error;

/// GeocoderError is an enum that represents errors that can occur while using the Geocoder.
#[derive(Debug, Error)]
pub enum GeocoderError {
    #[error("request failed")]
    RequestFailed(#[from] reqwest::Error),

    #[error("failed to parse response")]
    ParseError(#[from] serde_json::Error),

    #[error("address not found")]
    AddressNotFound,
}
