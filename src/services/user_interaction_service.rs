use crate::database::repositories::UserInteractionRepository;
use crate::models::user_interaction::*;

pub struct UserInteractionService { 
    user_interaction_repository: UserInteractionRepository,
}

impl UserInteractionService { 
    pub async fn new(user_interaction_repository: UserInteractionRepository) -> Self {
        Self {
            user_interaction_repository,
        }
    }

    pub async fn create_user_interaction(&self, request: CreateUserInteractionRequest) -> Result<UserInteractionView, sqlx::Error> {
        self.user_interaction_repository.create_user_interaction(request).await
    }

    pub async fn get_user_interaction(&self, query: UserInteractionQuery) -> Result<Vec<UserInteractionView>, sqlx::Error> {
        self.user_interaction_repository.get_user_interaction(query).await
    }

    pub async fn delete_user_interaction(&self, request: DeleteUserInteractionRequest) -> Result<(), sqlx::Error> {
        self.user_interaction_repository.delete_user_interaction(request).await
    }
}