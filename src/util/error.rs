enum_from_primitive! {
  #[derive(Debug, PartialEq, Clone)]
  pub enum Error {

    // 1000 for basic errors
    UnknownError = 1000,
    NotImplementedError,
    DatabaseError,
    CannotCreateObjectId,
    CannotParseObjectId,
    SerializeError,
    DeserializeError,
    NoneInsertedError,
    NoneDeletedError,

    // 1100 for user related errors
    UserNotFoundError = 1100,
    UserExistedError,
    InvalidUsername,
    InvalidPassword,
    NoSession,
    SessionNotFound,
    SessionExpired,
    UsernameOrPasswordError,

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
    MangaExistedError,
    MangaNotFoundError,
  }
}

impl Error {
  pub fn code(&self) -> i32 {
    let err = self.clone();
    err as i32
  }

  pub fn msg<'a>(&self) -> &'a str {
    match self {
      Error::UnknownError => "Unknown Error",
      Error::NotImplementedError => "Not Implemented Error",
      Error::DatabaseError => "Database Error",
      Error::CannotCreateObjectId => "Cannot Create Object ID",
      Error::CannotParseObjectId => "Cannot Parse Object ID",
      Error::SerializeError => "Unable to Serialize Data",
      Error::DeserializeError => "Unable to Deserialize Data",
      Error::NoneInsertedError => "Nothing is Getting Inserted",
      Error::NoneDeletedError => "Nothing is Getting Deleted",
      Error::UserNotFoundError => "User Not Found",
      Error::UserExistedError => "User Already Existed",
      Error::InvalidUsername => "Invalid Username",
      Error::InvalidPassword => "Invalid Password",
      Error::NoSession => "No Session Presented",
      Error::SessionNotFound => "Unable to find Session",
      Error::SessionExpired => "Session Expred",
      Error::UsernameOrPasswordError => "Wrong Username or Password",
      Error::GenreInfoExtractionError => "Cannot Extract Genre Information",
      Error::GenreNotFoundError => "Unable to find Genre",
      Error::DmkFetchError => "Unable to Fetch Cartoonmad Webpage",
      Error::DmkEncodingError => "Unable to Decode Cartoonmad Webpage",
      Error::DmkDomTraverseError => "Unable to Find Data from Cartoonmad Webpage",
      Error::DmkParseError => "Unable to Parse Information from Cartoonmad Webpage",
      Error::DmkRedirectNotFoundError => "Unable to Find Redirect Information for Cartoonmad Image",
      Error::DmkIdBaseParseError => "Unable to Parse Cartoonmad Image Information",
      Error::DmkSearchEncodingError => "Unable to Encode Search Text",
      Error::MangaNotFoundError => "Manga Not Found",
      Error::MangaExistedError => "Manga Already Existed",
    }
  }
}