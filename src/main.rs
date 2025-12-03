use livekit_api::access_token;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::Ipv6Addr;
use warp::{path, query, reply, Filter};

#[tokio::main]
async fn main() {
    let create_token_route = path("requestToken")
        .and(query::<QueryParams>())
        .map(|params: QueryParams| {
            let response = handle_create_token(params);
            println!("Sending /requestToken response.");
            response
        });

    let contact_route = path("requestServerIp")
        .map(|| {
            let response = handle_request_server_ip();
            println!("Sending /request_server_ip response.");
            response
        });

    println!("Starting Tokenserver");

    let port = env::var("TOKENSERVER_PORT")
        .unwrap()
        .trim()
        .parse::<u16>()
        .unwrap();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    warp::serve(create_token_route.or(contact_route).with(cors))
        .run((Ipv6Addr::UNSPECIFIED, port))
        .await;
}

/// Handles the creation of an access token for a user.
fn handle_create_token(params: QueryParams) -> reply::Json {
    match create_token(&params.room_name, &params.identity) {
        Ok(token) => {
            let response = TokenResponse { token };
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
        livekit_server_address: env::var("LIVEKIT_SERVER_ADDRESS").unwrap(),
    };
    reply::json(&response)
}

/// Creates a token using LiveKit API with given room and user identity.
fn create_token(room_name: &str, identity: &str) -> Result<String, String> {
    let api_key = env::var("LIVEKIT_API_KEY").unwrap();
    let api_secret = env::var("LIVEKIT_API_SECRET").unwrap();

    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity(identity)
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: room_name.to_string(),
            ..Default::default()
        })
        .to_jwt();

    println!(
        "Token created: room_name = {}, identity = {}",
        room_name, identity
    );

    token.map_err(|err| format!("Failed to generate token: {}", err))
}

/// Struct to deserialize the query parameters from the URL.
#[derive(Debug, Deserialize)]
struct QueryParams {
    room_name: String,
    identity: String,
}

/// Response structure for server address.
#[derive(Serialize, Deserialize)]
struct ServerResponse {
    fishnet_server_address: String,
    livekit_server_address: String,
}

/// Response structure for token information.
#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

/// Generic error response for failed operations.
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}
