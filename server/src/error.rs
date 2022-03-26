use actix_web::HttpResponse;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Internal server error")]
    ServerError,
    #[error("Database error")]
    DatabaseError,

    #[error("Username or password is incorrect")]
    LoginIncorrect,

    // Tokens
    #[error("Failed to create token")]
    TokenCreationFailure,
    #[error("Failed to delete token")]
    TokenDeletionFailure,
    #[error("Failed to fetch token")]
    TokenFetchFailure,
    #[error("Token invalid")]
    TokenInvalid, // TODO do we split this into all the diffrent ways it could be invalid?

    // User
    #[error("Failed to create user")]
    UserCreationFailure,
    #[error("Username {0} already exists")]
    UsernameConflict(String),
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

impl Error {
    fn to_response(self) -> HttpResponse {
        macro_rules! to_res {
            ($(($err:ident, $res:ident)),*) => {{
                match self {
                    $( Self::$err => HttpResponse::$res().body(format!("{}", Self::$err)), )*
                    _ => unimplemented!()
                }
            }}
        }

        //TODO basically have one error for db error and have some for fetches etc
        to_res!(
            (LoginIncorrect, Unauthorized),
            (TokenCreationFailure, InternalServerError),
            (TokenDeletionFailure, InternalServerError),
            (TokenFetchFailure, InternalServerError),
            (TokenInvalid, Unauthorized),
            (UserCreationFailure, InternalServerError),
            (UserDeletionFailure, InternalServerError),
            (UserFetchFailure, InternalServerError)
        )
    }
}

pub type Result<T> = std::result::Result<T, Error>;
