use crate::database::{repositories::UserRepository};
use crate::models::user::{self, AuthUserRequest, CreateUserRequest, DeleteUserRequest, UpdateUserRequest, User, UserQuery, UserView};

pub struct UserService { 
    pub repository: UserRepository,
}

impl UserService { 
    pub fn new(repository: UserRepository) -> Self { 
        Self { repository }
    }

    pub async fn create(&self, user: CreateUserRequest) -> Result<User, sqlx::Error> {
        self.repository.create(user).await
    }

    pub async fn update(&self, user: UpdateUserRequest) -> Result<User, sqlx::Error> {
        self.repository.update(user).await
    }

    pub async fn delete(&self, user: DeleteUserRequest) -> Result<(), sqlx::Error> {
        self.repository.delete(user).await
    }

    pub async fn get_all(&self, user: UserQuery) -> Result<Vec<UserView>, sqlx::Error> {
        self.repository.get_all_users(user).await
    }

}
