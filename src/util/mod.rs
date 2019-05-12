pub mod database;
pub mod template;
pub mod static_files;

pub use database::{database, Database};
pub use template::template;
pub use static_files::static_files;