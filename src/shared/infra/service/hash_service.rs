use bcrypt::{DEFAULT_COST, hash, verify};

pub trait HashService: Send + Sync {
    fn hash(&self, password: &str) -> String;
    fn compare(&self, password: &str, password_hash: &str) -> bool;
}

pub struct HashServiceImpl;

impl HashServiceImpl {
    pub fn new() -> Self {
        HashServiceImpl
    }
}

impl HashService for HashServiceImpl {
    fn hash(&self, password: &str) -> String {
        hash(password, DEFAULT_COST).expect("Couldn't hash the password")
    }

    fn compare(&self, password: &str, password_hash: &str) -> bool {
        verify(password, password_hash).expect("Couldn't compare the password")
    }
}