use thiserror::Error;

#[derive(Debug, Error, Copy, Clone)]
pub enum EnvError {
    #[error("enviroment already initialized")]
    AlredyInitialized,
    #[error(".env file not provided")]
    NotProvided
}
