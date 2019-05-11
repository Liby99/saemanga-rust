enum_from_primitive! {
  #[derive(Debug, PartialEq)]
  pub enum Error {

    // 1000 for basic errors
    UnknownError = 1000,
    DatabaseError,
    ObjectIdError,

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
    DmkEncodingError,
    DmkDomTraverseError,
    DmkParseError,
    DmkRedirectNotFoundError,
    DmkIdBaseParseError,
    DmkSearchEncodingError,
  }
}