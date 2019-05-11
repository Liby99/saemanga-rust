enum_from_primitive! {
  #[derive(Debug, PartialEq, Clone)]
  pub enum Error {

    // 1000 for basic errors
    UnknownError = 1000,
    NotImplementedError,
    DatabaseError,
    CannotCreateObjectId,
    CannotParseObjectId,

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
    MangaExistedError,
    MangaSerializeError,
    MangaDeserializeError,
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
      Error::UserSerializeError => "Cannot Serialize User",
      Error::UserDeserializeError => "Cannot Deserialize User",
      Error::UserNotFoundError => "User Not Found",
      Error::UserExistedError => "User Already Existed",
      Error::InvalidUsername => "Invalid Username",
      Error::InvalidPassword => "Invalid Password",
      Error::GenreInfoExtractionError => "Cannot Extract Genre Information",
      Error::GenreNotFoundError => "Unable to find Genre",
      Error::DmkFetchError => "Unable to Fetch Cartoonmad Webpage",
      Error::DmkEncodingError => "Unable to Decode Cartoonmad Webpage",
      Error::DmkDomTraverseError => "Unable to Find Data from Cartoonmad Webpage",
      Error::DmkParseError => "Unable to Parse Information from Cartoonmad Webpage",
      Error::DmkRedirectNotFoundError => "Unable to Find Redirect Information for Cartoonmad Image",
      Error::DmkIdBaseParseError => "Unable to Parse Cartoonmad Image Information",
      Error::DmkSearchEncodingError => "Unable to Encode Search Text",
      Error::MangaExistedError => "Manga Already Existed",
      Error::MangaSerializeError => "Unable to Serialize Manga Data",
      Error::MangaDeserializeError => "Unable to Deserialize Manga Data",
    }
  }
}