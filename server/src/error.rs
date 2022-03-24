use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Username or password is incorrect")]
    LoginIncorrect,

    // Tokens
    #[error("Failed to create token")]
    TokenCreationFailure,
    #[error("Failed to delete token")]
    TokenDeletionFailure,
    #[error("Failed to fetch token")]
    TokenFetchFailure,
    #[error("Token expired")]
    TokenExpired,

    // User
    #[error("Failed to create user")]
    UserCreationFailure,
    #[error("Failed to delete user")]
    UserDeletionFailure,
    #[error("Failed to fetch user")]
    UserFetchFailure,

    // Tiles
    #[error("Failed to create tile")]
    TileCreationFailure,
    #[error("Failed to delete tile")]
    TileDeletionFailure,
    #[error("Failed to fetch tile")]
    TileFetchFailure,
}
