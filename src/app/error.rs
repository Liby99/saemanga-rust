enum_from_primitive! {
  #[derive(Debug)]
  pub enum Error {

    // 1000 for basic errors
    UnknownError = 1000,
    DatabaseError(mongodb::Error),
    ObjectIdError(mongodb::oid::Error),

    // 1100 for user related errors
    UserSerializeError = 1100,
    UserDeserializeError,
    UserNotFoundError,
    UserExistedError,
    InvalidUsername,
    InvalidPassword,

    // 1200 for dmk/manga related errors
    GenreInfoExtractionError = 1200,
    GenreNotFoundError,
    DmkFetchError,
    DmkHtmlElemNotFound,
  }
}