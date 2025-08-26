use sqlx::query;

use crate::database::{repositories::UserRepository};
use crate::models::user::{self, CreateUserRequest, DeleteUserRequest, UpdateUserRequest, User, UserQuery, UserView};

pub struct UserService { 
    pub repository: UserRepository,
}

impl UserService { 
    pub fn new(repository: UserRepository) -> Self { 
        Self { repository }
    }

    pub async fn create(&self, username: &str, password: &str, email: &str, avatar_url: &str, bio: &str) -> Result<User, sqlx::Error> {
        let user: CreateUserRequest = CreateUserRequest { username: username.to_string(), password: password.to_string(), email: email.to_string(), avatar_url: avatar_url.to_string(), bio: bio.to_string() };
        let user = self.repository.create(user).await?;
        Ok(user)
    }

    pub async fn update(&self, id: i32, username: &str, password: &str, email: &str, avatar_url: &str, bio: &str) -> Result<User, sqlx::Error> {
        let user: UpdateUserRequest = UpdateUserRequest { 
            id: id.to_string(),
            username: Some(username.to_string()), 
            password: Some(password.to_string()), 
            email: Some(email.to_string()), 
            avatar_url: Some(avatar_url.to_string()), 
            bio: Some(bio.to_string()) };

        self.repository.update(user).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        let user: DeleteUserRequest = DeleteUserRequest { id: id.to_string() };
        self.repository.delete(user).await
    }

    pub async fn get_all(&self, query: UserQuery) -> Result<Vec<UserView>, sqlx::Error> {
        let user: UserQuery = UserQuery { 
            username: query.username.map(|s| s.to_string()),
            email: query.email.map(|s| s.to_string()), 
            id: query.id.map(|s| s.to_string()) };
        self.repository.get_all_users(user).await
    }

}
