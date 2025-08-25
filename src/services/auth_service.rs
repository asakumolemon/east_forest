use crate::database::{repositories::UserRepository};
use crate::models::user::{AuthUserRequest, AuthUserResponse};

pub struct AuthService { 
    pub repository: UserRepository,
}

impl AuthService { 
    pub fn new(repository: UserRepository) -> Self { 
        Self { repository }
    }

    pub async fn auth(&self, auth_user: AuthUserRequest) -> Result<AuthUserResponse, sqlx::Error> { 
        self.repository.auth(auth_user).await
    }
}
