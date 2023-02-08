use std::{convert::Infallible};

use warp::Filter;

use crate::{handlers, db::Db, models::Dive};

pub fn dive_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_dive(db.clone())
        .or(update_dive(db.clone()))
        .or(delete_dive(db.clone()))
        .or(create_dive(db.clone()))
        .or(dives_list(db.clone()))
        .or(get_health())
}

/*
pub fn health_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_health()
} */

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Dive,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn dives_list(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("dives")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_dives)
}

fn create_dive(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("dives")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_dive)
}

fn get_dive(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("dives" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_dive)
}

fn update_dive(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("dives" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_dive)
}

fn delete_dive(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("dives" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_dive)
}

fn get_health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("health")
        .and_then(handlers::get_health)
}