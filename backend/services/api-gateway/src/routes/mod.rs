pub mod auth;
pub mod comments;
pub mod posts;
pub mod users;

pub use auth::router as auth_router;
pub use comments::router as comments_router;
pub use posts::router as posts_router;
pub use users::router as users_router;