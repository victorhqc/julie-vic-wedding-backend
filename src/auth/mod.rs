mod facebook;
pub use self::facebook::*;

mod google;
pub use self::google::*;

mod token;
pub use self::token::*;

use std::env;

pub fn get_url() -> String {
    env::var("PUBLIC_API_URL").expect("Missing PUBLIC_API_URL environment variable.")
}
