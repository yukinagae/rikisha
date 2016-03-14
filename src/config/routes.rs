extern crate router;
use router::Router;

use controllers::hello::Hello;

pub fn routes() -> Router { router!(

    // routes
    get    "/hellos"          => Hello::index,  // index
    get    "/hellos/new"      => Hello::new,    // new
    post   "/hellos"          => Hello::create, // create
    get    "/hellos/:id"      => Hello::show,   // show
    get    "/hellos/:id/edit" => Hello::edit,   // edit
    put    "/hellos/:id"      => Hello::update, // update
    delete "/hellos/:id"      => Hello::delete, // delete

)}
