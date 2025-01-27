use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod handlers;
pub mod middlewares;
pub mod utils;
pub mod routes;
pub mod schema;
pub mod models;
pub mod services;

pub use routes::init_routes;
pub use middlewares::SayHi;