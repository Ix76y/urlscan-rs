use reqwest::StatusCode;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UrlScanError {
    #[error("You have exceeded one of your quotas (minute, daily or monthly). Daily quotas are reset every day at 00:00 UTC. You may have run out of disk space and/or number of files on your VirusTotal Monitor accounts.")]
    QuotaExceededError,
    #[error("Too many requests.")]
    TooManyRequestsError,
    #[error("The requested resource was not found.")]
    NotFoundError,
    #[error("You are not allowed to perform the requested operation.")]
    ForbiddenError,
    #[error("The provided API key is incorrect.")]
    WrongCredentialsError,
    #[error("Unknown error.")]
    Unknown,
}

