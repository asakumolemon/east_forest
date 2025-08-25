use bcrypt::hash;
use bcrypt::verify;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation, Algorithm, decode};
use serde::{Deserialize, Serialize};


struct JwtConfig { 
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims { 
    pub sub: String,
    pub exp: i64,
}

pub fn hash_password(password: &str) -> String {
    hash(password, 10).unwrap()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap()
}

pub fn create_jwt(user_id: String) -> String {
    let jwt_config = JwtConfig { 
        secret: "secret".to_string(),
    };  
    let claims = Claims { 
        sub: user_id,
        exp: chrono::Utc::now().timestamp() + 3600, // 1 hour
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_config.secret.as_ref())).unwrap();
    token
}

pub fn verify_jwt(token: &str) -> bool {
    let jwt_config = JwtConfig { 
        secret: "secret".to_string(),
    };
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(jwt_config.secret.as_ref()), &validation);
    if token_data.is_ok() {
        if token_data.unwrap().claims.exp > chrono::Utc::now().timestamp() {
            return true;
        } else {
            return false;
        }
    }else {
        false
    }
}
