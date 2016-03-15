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


pub type Result = IronResult<Response>;

//
pub fn ok(body: &str) -> Result {
    Ok(Response::with((status::Ok, body)))
}

pub fn bad(body: &str) -> Result {
    Ok(Response::with((status::BadRequest, body)))
}

pub fn render(view: &str, data: BTreeMap<String, Json>) -> Result {
    let mut resp = Response::new();
    resp.set_mut(Template::new(view, data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn param(req: &mut Request, name: &str) -> Option<String> {
    req.extensions.get::<Router>().unwrap().find(name).map(|x| Some(x.to_string())).unwrap_or(None)
}

// hello
pub struct Hello;

impl Hello {

    pub fn index(req: &mut Request) -> Result {
        let data = make_data();
        render("index", data)
    }

    pub fn new(req: &mut Request) -> Result {
        let mut data = BTreeMap::new();
        render("new", data)
    }

    pub fn create(req: &mut Request) -> Result {
        let data = make_data();
        render("index", data)
    }

    pub fn show(req: &mut Request) -> Result {
        match param(req, "id") {
            Some(id) => {
                let mut data = BTreeMap::new();
                data.insert("id".to_string(), id.to_string().to_json());
                render("show", data)
            }
            None => bad("id is required!")
        }
    }

    pub fn edit(req: &mut Request) -> Result {
        match param(req, "id") {
            Some(id) => {
                let mut data = BTreeMap::new();
                data.insert("id".to_string(), id.to_string().to_json());
                render("edit", data)
            }
            None => bad("id is required!")
        }
    }
    
    pub fn update(req: &mut Request) -> Result {
        // TODO
        let data = make_data();
        render("index", data)
    }

    pub fn delete(req: &mut Request) -> Result {
        // TODO
        let data = make_data();
        render("index", data)
    }

}
