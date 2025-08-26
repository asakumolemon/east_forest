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

    pub async fn create(&self, user: CreateUserRequest) -> Result<User, sqlx::Error> {
        let user = self.repository.create(user).await?;
        Ok(user)
    }

    pub async fn update(&self, user: UpdateUserRequest) -> Result<User, sqlx::Error> {
        self.repository.update(user).await
    }

    pub async fn delete(&self, user: DeleteUserRequest) -> Result<(), sqlx::Error> {
        self.repository.delete(user).await
    }

    pub async fn get_all(&self, query: UserQuery) -> Result<Vec<UserView>, sqlx::Error> {
        let user: UserQuery = UserQuery { 
            username: query.username.map(|s| s.to_string()),
            email: query.email.map(|s| s.to_string()), 
            id: query.id.map(|s| s.to_string()) };
        self.repository.get_all_users(user).await
    }

    pub async fn get_user(&self, query: UserQuery) -> Result<UserView, sqlx::Error> { 
        self.repository.get_user(query).await
    }

}
