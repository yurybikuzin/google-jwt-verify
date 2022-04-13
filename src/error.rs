use crate::algorithm::Algorithm;
use base64::DecodeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("NoHeader")]
    NoHeader,
    #[error("NoPayload")]
    NoPayload,
    #[error("NoSignature")]
    NoSignature,
    #[error("AudienceDoesNotMeetClientId: {audience} != {client_id}")]
    AudienceDoesNotMeetClientId { audience: String, client_id: String },
    #[error("InappropriateIssuer: {0}")]
    InappropriateIssuer(String),
    #[error("Expired: current: {current}, expires_at: {expires_at}")]
    Expired { current: u64, expires_at: u64 },
    #[error("ExpiredIssued: issued_at: {issued_at}, expires_at: {expires_at}")]
    ExpiredIssued { issued_at: u64, expires_at: u64 },
    #[error("NoKey")]
    NoKey,
    #[error("RetrieveKeyFailure")]
    RetrieveKeyFailure(anyhow::Error),
    #[error("UnsupportedAlgorithm")]
    UnsupportedAlgorithm(Algorithm),
    #[error("DecodeError: {0}")]
    Decode(anyhow::Error),
    #[error("ParseError: {0}")]
    Parse(anyhow::Error),
    #[error("ParseError: {0}")]
    Openssl(anyhow::Error),
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Error::Decode(anyhow::anyhow!(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Parse(anyhow::anyhow!(err))
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Error::Openssl(anyhow::anyhow!(err))
    }
}
