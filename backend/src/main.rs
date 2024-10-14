#[macro_use]
extern crate rocket;

use rocket::{
    http::{Header, Status},
    response,
    response::{content::RawJson, Responder},
    Request, Response, State,
};

use game::Game;

use serde_json::json;
use serde_json::{self};
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

type GameState = Arc<Mutex<Option<Game>>>;

#[derive(Responder)]
struct PlainCORSResponse {
    content: RawJson<String>,
    cross_origin_override: Header<'static>,
    server_header: Header<'static>,
}
impl<T: Display> From<T> for PlainCORSResponse {
    fn from(value: T) -> Self {
        Self {
            content: RawJson(format!("{value}")),
            cross_origin_override: Header::new("Access-Control-Allow-Origin", "*"),
            server_header: Header::new("server", "Tic Tac Toe Server"),
        }
    }
}

struct StatusCORSResponse {
    status: Status,
    cross_origin_override: Header<'static>,
    server_header: Header<'static>,
}
impl From<Status> for StatusCORSResponse {
    fn from(value: Status) -> Self {
        Self {
            status: value,
            cross_origin_override: Header::new("Access-Control-Allow-Origin", "*"),
            server_header: Header::new("server", "Tic Tac Toe Server"),
        }
    }
}
impl<'r> Responder<'r, 'static> for StatusCORSResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let mut res = Response::new();
        res.set_status(self.status);
        res.set_header(self.cross_origin_override);
        res.set_header(self.server_header);
        response::Result::Ok(res)
    }
}

#[post("/init")]
fn init(state: &State<GameState>) -> PlainCORSResponse {
    let initiated = Game::init();

    let mut current_state = state.lock().unwrap();
    *current_state = Some(initiated);
    PlainCORSResponse::from(serde_json::to_string("Game initiated").unwrap())
}

#[post("/engine_move")]
fn engine_move(state: &State<GameState>) -> StatusCORSResponse {
    let mut retreived = state.lock().unwrap();
    if let Some(mut current_state) = retreived.take() {
        current_state.engine_move();
        *retreived = Some(current_state);
        StatusCORSResponse::from(Status::Ok)
    } else {
        StatusCORSResponse::from(Status::InternalServerError)
    }
}

#[post("/manual_move", data = "<chosen_move>")]
fn manual_move(chosen_move: &str, state: &State<GameState>) -> StatusCORSResponse {
    let mut retreived = state.lock().unwrap();
    if let Some(mut current_state) = retreived.take() {
        if let Ok(n) = chosen_move.parse::<u8>() {
            current_state.manual_move(n);
            *retreived = Some(current_state);
            StatusCORSResponse::from(Status::Ok)
        } else {
            StatusCORSResponse::from(Status::BadRequest)
        }
    } else {
        StatusCORSResponse::from(Status::InternalServerError)
    }
}

#[get("/state")]
fn state(state: &State<GameState>) -> PlainCORSResponse {
    let mut retreived = state.lock().unwrap();
    if let Some(current_state) = retreived.take() {
        let res = PlainCORSResponse::from(json!(current_state.get_state()));
        *retreived = Some(current_state);
        res
    } else {
        PlainCORSResponse::from(json!("Could not get state. Game does not exist"))
    }
}

#[launch]
fn rocket() -> _ {
    let state: GameState = Arc::new(Mutex::new(None));
    rocket::build()
        .manage(state)
        .configure(rocket::Config::figment().merge(("port", 8884)))
        .mount("/", routes![init, engine_move, manual_move, state])
}
