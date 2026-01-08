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
pub extern "C" fn create_token(
    room: *const c_char,
    user_name: *const c_char,
    user_identity: *const c_char,
) -> *mut c_char {
    let empty_c_char: *mut i8 = CString::new("").unwrap().into_raw();

    if room.is_null() {
        println!("Error: room is a null pointer.");
        return empty_c_char;
    }

    if user_name.is_null() {
        println!("Error: user_name is a null pointer.");
        return empty_c_char;
    }

    if user_identity.is_null() {
        println!("Error: user_identity is a null pointer.");
        return empty_c_char;
    }

    let c_room = unsafe { CStr::from_ptr(room) };
    let c_user_name = unsafe { CStr::from_ptr(user_name) };
    let c_user_identity = unsafe { CStr::from_ptr(user_identity) };

    let res = match (
        c_room.to_str(),
        c_user_name.to_str(),
        c_user_identity.to_str(),
    ) {
        (Ok(room), Ok(user_name), Ok(user_identity)) => {
            create_token_internal(room, user_name, user_identity)
        }
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

fn create_token_internal(
    room_name: &str,
    user_name: &str,
    user_identity: &str,
) -> Result<String, TokenError> {
    let api_key = env::var(LIVEKIT_API_KEY_ENV)
        .map_err(|_| TokenError::EnvVarNotPresent(LIVEKIT_API_KEY_ENV.to_string()))?;
    let api_secret = env::var(LIVEKIT_API_SECRET_ENV)
        .map_err(|_| TokenError::EnvVarNotPresent(LIVEKIT_API_SECRET_ENV.to_string()))?;

    access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_name(user_name)
        .with_identity(user_identity)
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: room_name.to_string(),
            ..Default::default()
        })
        .to_jwt()
        .map_err(|e| TokenError::AccessTokenError(e))
}
