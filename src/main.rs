use livekit_api::access_token;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::Ipv6Addr;
use uuid::Uuid;
use warp::{path, query, reply, Filter};

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").expect("Failed to read .env file");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"]);

    let create_token_route = path("requestToken")
        .and(query::<QueryParams>())
        .map(handle_create_token)
        .with(cors.clone());

    let contact_route = path("requestServerIp")
        .map(handle_request_server_ip)
        .with(cors.clone());

    println!("Starting Tokenserver");
    warp::serve(create_token_route.or(contact_route))
        .run((Ipv6Addr::UNSPECIFIED, 3030))
        .await;
}

/// Handles the creation of an access token for a user.
fn handle_create_token(params: QueryParams) -> reply::Json {
    match create_token(&params.room_name, &params.user_name) {
        Ok(token) => {
            let response = TokenResponse {
                token,
                livekit_server_address: env::var("LIVEKIT_SERVER_ADDRESS").unwrap(),
            };
            reply::json(&response)
        }
        Err(err) => {
            eprintln!("Error creating token: {}", err);
            reply::json(&ErrorResponse {
                error: "Failed to create token".to_string(),
            })
        }
    }
}

/// Handles the request for the server IP address.
fn handle_request_server_ip() -> reply::Json {
    let response = ServerResponse {
        fishnet_server_address: env::var("FISHNET_SERVER_ADDRESS").unwrap(),
    };
    reply::json(&response)
}

/// Creates a token using LiveKit API with given room and user names.
fn create_token(room_name: &str, user_name: &str) -> Result<String, String> {
    let api_key = env::var("LIVEKIT_API_KEY").unwrap();
    let api_secret = env::var("LIVEKIT_API_SECRET").unwrap();

    let user_id = Uuid::new_v4();
    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity(&user_id.to_string())
        .with_name(user_name)
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: room_name.to_string(),
            ..Default::default()
        })
        .to_jwt();

    println!(
        "Token created: user_name = {}, user_id = {}, room_name = {}",
        user_name, user_id, room_name
    );

    token.map_err(|err| format!("Failed to generate token: {}", err))
}

/// Struct to deserialize the query parameters from the URL.
#[derive(Debug, Deserialize)]
struct QueryParams {
    room_name: String,
    user_name: String,
}

/// Response structure for server address.
#[derive(Serialize, Deserialize)]
struct ServerResponse {
    fishnet_server_address: String,
}

/// Response structure for token information.
#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
    livekit_server_address: String,
}

/// Generic error response for failed operations.
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}
