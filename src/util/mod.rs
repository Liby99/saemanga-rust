pub mod database;
pub mod template;
pub mod static_files;
pub mod collection;
pub mod error;
pub mod cookie_value;

pub use database::{database, Database};
pub use template::template;
pub use static_files::static_files;
pub use error::Error;
pub use self::collection::Collection;