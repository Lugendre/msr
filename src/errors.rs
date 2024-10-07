pub mod infra;
pub mod domain;
pub mod usecase;

use thiserror::Error;
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Domain {
        #[from]
        source: domain::DomainError,
    },
    #[error(transparent)]
    Infra {
        #[from]
        source: infra::InfraError,
    }
}
