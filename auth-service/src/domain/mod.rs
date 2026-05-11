/* The domain module stores data types that are core to the auth service.
 * These data types should be decoupled from external framewroks as much as possible. */

pub mod user;

// re-export items from sub-modules for easy access
pub use user::*;