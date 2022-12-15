use actix_web::{get, post, web, HttpResponse};
use crate::model::{self, Games, CreateNewGame, JoinGame};
use mongodb::{
    bson::doc,
    options::{IndexOptions, FindOneOptions},
    Client,
    Collection,
    IndexModel
};

const DB_NAME: &str = "chess";
const COLL_NAME: &str = "games";
const MONGO_USER: &str = "admin";
const MONGO_PASS: &str = "admin1234";

/// Adds a new game to the "users" collection in the database.
#[post("/add_new_game")]
async fn add_new_game(client: web::Data<Client>, form: web::Form<CreateNewGame>) -> HttpResponse {
    let collection: Collection<Games> = client.database(DB_NAME).collection(COLL_NAME);

    // Get the latest game id
    let find_options = FindOneOptions::builder().sort(doc! {"id": -1}).build();
    let latest_id = match collection.find_one(None, find_options).await.unwrap() {
        Some(doc) => doc.id,
        None => 0
    };

    let new_game = Games::new(latest_id, form.into_inner().player_name);
    let result = collection.insert_one(new_game.clone(), None).await;
    match result {
        Ok(_res) => HttpResponse::Ok().body(format!("Game Added\n{:#?}", new_game)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Gets the game with the supplied game_id.
#[get("/get_game/{game_id}")]
async fn get_game(client: web::Data<Client>, game_id: web::Path<f32>) -> HttpResponse {
    let collection: Collection<Games> = client.database(DB_NAME).collection(COLL_NAME);
    let game_id = game_id.into_inner();

    match collection
        .find_one(doc! { "id": game_id }, None)
        .await
    {
        Ok(Some(game)) => HttpResponse::Ok().json(game),
        Ok(None) => HttpResponse::NotFound().body(format!("No game found with id {game_id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Join a Game
#[post("/join_game")]
async fn join_game(client: web::Data<Client>, form: web::Form<JoinGame>) -> HttpResponse {
    let collection: Collection<Games> = client.database(DB_NAME).collection(COLL_NAME);
    let form = form.into_inner();

    match collection
        .find_one(doc! { "id": form.game_id }, None)
        .await
    {
        Ok(Some(game)) => {
            if game.status == model::Status::Waiting {
                let filter = doc! { "id": form.game_id };
                let update = doc! { "$set": { "player_black": form.player_name, "status": "Playing" } };
                let update_result = collection.update_one(filter, update, None).await.unwrap();
                HttpResponse::Ok().json(update_result)
            } else {
                HttpResponse::Ok().json(doc! { "message": "Cannot join this game." })
            }
        },
        Ok(None) => HttpResponse::NotFound().body(format!("No game found with id {}", &form.game_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub struct ClientBuilder;

impl ClientBuilder {
    pub async fn new() -> Client {
        let default_uri = format!("mongodb://{}:{}@localhost:27017", MONGO_USER, MONGO_PASS);
        let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| default_uri.into());

        let client = mongodb::Client::with_uri_str(uri)
            .await
            .expect("failed to connect");

        create_game_index(&client).await;

        client
    }
}

/// Creates an index on the "id" field to force the values to be unique.
async fn create_game_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "id": 1 })
        .options(options)
        .build();

    client
        .database(DB_NAME)
        .collection::<Games>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}