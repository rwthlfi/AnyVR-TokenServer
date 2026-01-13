# livekit-token-ffi

A simple dynamic library providing an FFI for generating LiveKit tokens.
Uses the official [LiveKit Rust SDK](https://github.com/livekit/rust-sdks).

This library was created to be used in the [AnyVR](https://github.com/rwthlfi/AnyVR) Unity package.

## Build

```bash
cargo build --release
```

## Usage in Unity / AnyVR

The dedicated AnyVR Unity server runs on Linux, so compiling this crate into a Linux library is sufficient.

Import the generated 'liblivekit_tokengen_ffi.so' file into the 'Plugins/Linux' directory and declare an FFI method as follows.

```csharp
[DllImport("livekit_tokengen_ffi", EntryPoint = "create_token")]
public static extern string CreateToken(string room, string userName, string userIdentity);

```

The library reads the two environmental variables 'LIVEKIT_API_KEY' and 'LIVEKIT_API_SECRET' which have to be set in the environment of the Unity server process.

The tokens should only be generated on the Unity server, so the library does not have to be linked in the client builds.
