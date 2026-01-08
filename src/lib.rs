use libc::c_char;
use livekit_api::access_token::{self, AccessTokenError};
use std::env::{self};
use std::ffi::{CStr, CString};

const LIVEKIT_API_KEY_ENV: &'static str = "LIVEKIT_API_KEY";
const LIVEKIT_API_SECRET_ENV: &'static str = "LIVEKIT_API_SECRET";

#[derive(Debug)]
pub enum TokenError {
    EnvVarNotPresent(String),
    AccessTokenError(AccessTokenError),
    InvalidParameter,
}

#[no_mangle]
pub extern "C" fn create_token(room: *const c_char, user: *const c_char) -> *mut c_char {
    let empty_c_char: *mut i8 = CString::new("").unwrap().into_raw();

    if room.is_null() {
        println!("Error: room is a null pointer.");
        return empty_c_char;
    }

    if user.is_null() {
        println!("Error: user is a null pointer.");
        return empty_c_char;
    }

    let c_room = unsafe { CStr::from_ptr(room) };
    let c_user = unsafe { CStr::from_ptr(user) };

    let res = match (c_room.to_str(), c_user.to_str()) {
        (Ok(room), Ok(user)) => create_token_internal(room, user),
        _ => Err(TokenError::InvalidParameter),
    };

    match res {
        Ok(token) => CString::new(token).unwrap().into_raw(),
        Err(e) => {
            print_err(e);
            empty_c_char // Return empty string if an error occurs
        }
    }
}

fn print_err(err: TokenError) {
    match err {
        TokenError::InvalidParameter => {
            eprintln!("Error: invalid room or user string.")
        }
        TokenError::EnvVarNotPresent(e) => {
            eprintln!("Error reading environment variable: {}", e)
        }
        TokenError::AccessTokenError(e) => eprintln!("Error generating access token: {}", e),
    }
}

/// Creates a token using LiveKit API with given room and user identity.
fn create_token_internal(room_name: &str, identity: &str) -> Result<String, TokenError> {
    let api_key = env::var(LIVEKIT_API_KEY_ENV)
        .map_err(|_| TokenError::EnvVarNotPresent(LIVEKIT_API_KEY_ENV.to_string()))?;
    let api_secret = env::var(LIVEKIT_API_SECRET_ENV)
        .map_err(|_| TokenError::EnvVarNotPresent(LIVEKIT_API_SECRET_ENV.to_string()))?;

    access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity(identity)
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: room_name.to_string(),
            ..Default::default()
        })
        .to_jwt()
        .map_err(|e| TokenError::AccessTokenError(e))
}
