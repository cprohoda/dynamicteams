struct AppAuth {
    client_id: String, // client here is the webapp, not the user
    client_secret: String,
}

struct ClientAuth {
    redirect: String,
    code: String,
}

struct AuthorizedSession {
    access_token: String,
    expires_in: i32,
    id_token: JWT,
    token_type: String,
}

struct JWT {
    header_alg: String,
    header_typ: String,
    payload: Vec<JWTPayloadClaims>,
    Signature: String,
}

enum JWTPayloadClaims { // names by JWT specifications
    iss(String),
    sub(String),
    aud(String),
    exp(String),
    nbf(String),
    iat(String),
    jti(String),
}
