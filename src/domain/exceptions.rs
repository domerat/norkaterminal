#[derive(thiserror::Error, Debug, Clone)]
pub enum DomainException {
    #[error("Your request cannot be processed: {0}")]
    ProcessingError(String),

    #[error("Cannot insert a new value to the table {0}")]
    DatabaseInsertionError(String),

    #[error("Cannot search on table {0}")]
    DatabaseSearchError(String),

    #[error("Authentication failed due to server issues. Try again later.")]
    AuthenticationFailure,

    #[error("Registration failed due to the server issues. Try again later.")]
    RegistrationFailure,

    #[error("Unknown command")]
    UnknownCommand,

    #[error("Module caused an exception: {0}")]
    ModuleError(String)
}
