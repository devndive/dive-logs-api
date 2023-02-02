/*
    /dives
        - GET    list all dives
        - POST   create a new dive
    /dives/{guid}
        - GET    list details for a dive
        - POST   update information for a dive
        - DELETE remove dive
*/

mod models;
mod db;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("Loading dives");
    let db = db::init_db();

    println!("Adding routes");
    let dive_routes = routes::dive_routes(db);

    println!("Starting server");
    warp::serve(dive_routes)
        .run(([0,0,0,0], 3000))
        .await;
}
