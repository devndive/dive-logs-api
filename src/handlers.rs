use std::convert::Infallible;

use warp::hyper::StatusCode;

use crate::{models::Dive, db::Db};

pub async fn list_dives(db: Db) -> Result<impl warp::Reply, Infallible> {
    println!("Getting dives");

    let dives = db.lock().await;
    let dives: Vec<Dive> = dives.clone();

    Ok(warp::reply::json(&dives))
}

pub async fn create_dive(
    new_dive: Dive,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut dives = db.lock().await;

    for dive in dives.iter() {
        if dive.guid == new_dive.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    dives.push(new_dive);

    Ok(StatusCode::CREATED)
}

pub async fn get_dive(guid: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let dives = db.lock().await;

    for dive in dives.iter() {
        if dive.guid == guid {
            return Ok(Box::new(warp::reply::json(&dive)));
        }
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_dive(
    guid:String,
    updated_dive: Dive,
    db: Db
) -> Result<impl warp::Reply, Infallible> {
    let mut dives = db.lock().await;

    for dive in dives.iter_mut() {
        if dive.guid == guid {
            *dive = updated_dive;
            return Ok(StatusCode::OK)
        }
    }

    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_dive(guid: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut dives = db.lock().await;

    let dive_count = dives.len();

    dives.retain(|dive| dive.guid != guid);

    let deleted = dives.len() != dive_count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}