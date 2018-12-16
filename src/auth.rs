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
    header: JWTHeader,
    payload: JWTPayload,
    signature: JWTSignature,
}

impl fmt::Display for JWT {
    fn fmt(&self, f: &mut fmt:Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.header, self.payload, self.signature)
    }
}

struct JWTHeader {
    header_alg: String,
    header_typ: String,
}

struct JWTPayload {
    payload: Vec<JWTPayloadClaims>,
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

struct JWTSignature {
    secret: String,
    header: JWTHeader,
    payload: JWTPayload,
}
