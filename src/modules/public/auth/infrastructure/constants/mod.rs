use std::{env, sync::LazyLock};

use crate::modules::public::auth::infrastructure::jwt::key::Key;

pub static JWT_TOKEN: LazyLock<Key> = LazyLock::new(|| {
    let token = env::var("JWT_TOKEN").expect("Couldn't find JWT_TOKEN key");
    
    Key::new(token.as_bytes())
});