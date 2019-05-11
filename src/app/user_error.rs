enum_from_primitive! {
  #[derive(Debug)]
  pub enum UserError {
    DatabaseError = 1000, // We start the user error from 1000
    UserIdError,
    UserNotFoundError,
    UserDataError,
    UserExistedError,
    InvalidUsername,
    InvalidPassword,
  }
}

impl From<mongodb::oid::Error> for UserError {
  fn from(_: mongodb::oid::Error) -> Self {
    UserError::UserIdError
  }
}

impl From<mongodb::Error> for UserError {
  fn from(_: mongodb::Error) -> Self {
    UserError::DatabaseError
  }
}