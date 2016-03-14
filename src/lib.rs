#[macro_use]
extern crate router;

// local library
pub mod controllers {
    pub mod hello;
}
pub mod config {
    pub mod routes;
    pub mod env;
}


#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
