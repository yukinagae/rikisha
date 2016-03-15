// base
extern crate iron;
use iron::prelude::*;
use std::net::ToSocketAddrs;

use std::error::Error;

// hbs
extern crate handlebars_iron as hbs;
use hbs::{Template, HandlebarsEngine, DirectorySource};

// riki
extern crate riki;
use riki::config::routes;
use riki::config::env;

fn main() {

    let mut chain = Chain::new(routes::routes());

    let mut hbse = HandlebarsEngine::new2();
    hbse.add(Box::new(DirectorySource::new("./src/views/", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }

    chain.link_after(hbse);

    let address = [env::HOST, ":", env::PORT].concat();
    Iron::new(chain).http(address.to_socket_addrs().unwrap().next().unwrap()).unwrap();
}
