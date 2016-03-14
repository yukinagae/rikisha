extern crate iron;
extern crate router;

use self::iron::prelude::*;
use self::router::Router;
use self::iron::status;

extern crate handlebars_iron as hbs;
use self::hbs::Template;

extern crate rustc_serialize;
use self::rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub struct Team { name: String, pts: u16 }

impl ToJson for Team {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("name".to_string(), self.name.to_json());
        m.insert("pts".to_string(), self.pts.to_json());
        m.to_json()
    }
}

pub fn make_data () -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();
    data.insert("year".to_string(), "2015".to_json());
    let teams = vec![ Team { name: "Jiangsu Sainty".to_string(),
                             pts: 43u16 },
                      Team { name: "Beijing Guoan".to_string(),
                             pts: 27u16 },
                      Team { name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16 },
                      Team { name: "Shandong Luneng".to_string(),
                             pts: 12u16 } ];

    data.insert("teams".to_string(), teams.to_json());
    data.insert("engine".to_string(), "rustc_serialize".to_json());
    data
}

// hello
pub struct Hello;

impl Hello {

    pub fn index(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "index")))
    }

    pub fn new(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "new")))
    }

    pub fn create(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "create")))
    }

    pub fn show(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("no id");
        Ok(Response::with((status::Ok, *id)))
    }

    pub fn edit(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "edit")))
    }
    
    pub fn update(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "update")))
    }

    pub fn delete(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "delete")))
    }

}
