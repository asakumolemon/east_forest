use crate::database::repositories::PromptRepository;
use crate::models::prompt::{CreatePromptRequest, UpdatePromptRequest, DeletePromptRequest, PromptQuery, PromptView};

pub struct PromptService { 
    pub prompt_repository: PromptRepository,
}

impl PromptService { 
    pub async fn create_prompt(&self, request: CreatePromptRequest) -> Result<PromptView, sqlx::Error> {
        let prompt = self.prompt_repository.create_prompt(request).await?;
        Ok(prompt)
    }

    pub async fn update_prompt(&self, request: UpdatePromptRequest) -> Result<PromptView, sqlx::Error> {
        let prompt = self.prompt_repository.update_prompt(request).await?;
        Ok(prompt)
    }

    pub async fn delete_prompt(&self, request: DeletePromptRequest) -> Result<(), sqlx::Error> {
        self.prompt_repository.delete_prompt(request).await?;
        Ok(())
    }

    pub async fn get_prompt(&self, query: PromptQuery) -> Result<PromptView, sqlx::Error> {
        let prompt = self.prompt_repository.get_prompt(query).await?;
        Ok(prompt)
    }
}