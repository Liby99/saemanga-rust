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
    NoneModifiedError,
    NoneDeletedError,
    DecodeUrlError,

    // 1100 for setting related errors
    UnknownLightMode = 1100,
    UnknownHandMode,
    UnknownIndexDisplayMode,

    // 1200 for user related errors
    UserNotFoundError = 1200,
    UserExistedError,
    InvalidUsername,
    InvalidPassword,
    NoSession,
    SessionNotFound,
    SessionExpired,
    UsernameOrPasswordError,
    IncorrectOldPassword,
    NotAuthenticated,

    // 1300 for dmk/manga related errors
    GenreInfoExtractionError = 1300,
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

    // 1400 for latest info related errors

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
      Error::NoneModifiedError => "Nothing is Getting Modified",
      Error::NoneDeletedError => "Nothing is Getting Deleted",
      Error::DecodeUrlError => "Unable to Decode Url",
      Error::UnknownLightMode => "Unknown Light Mode",
      Error::UnknownHandMode => "Unknown Hand Mode",
      Error::UnknownIndexDisplayMode => "Unknown Index Display Mode",
      Error::UserNotFoundError => "User Not Found",
      Error::UserExistedError => "User Already Existed",
      Error::InvalidUsername => "Invalid Username",
      Error::InvalidPassword => "Invalid Password",
      Error::NotAuthenticated => "Not Authenticated",
      Error::NoSession => "No Session Presented",
      Error::SessionNotFound => "Unable to find Session",
      Error::SessionExpired => "Session Expred",
      Error::UsernameOrPasswordError => "Wrong Username or Password",
      Error::IncorrectOldPassword => "Incorrect Old Password",
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