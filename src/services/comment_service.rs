use crate::{database::repositories::CommentRepository, models::comment::CommentQuery};
use crate::models::comment::*;

pub struct CommentService {
    repo: CommentRepository
}

impl CommentService {
    pub fn new(repo: CommentRepository) -> Self {
        Self {
            repo,
        }
    }
    pub async fn create_comment(&self, request: CreateCommentRequest) -> Result<CommentView, sqlx::Error> {
        self.repo.create_comment(request).await
    }

    pub async fn update_comment(&self, request: UpdateCommentRequest) -> Result<CommentView, sqlx::Error> {
        self.repo.update_comment(request).await
    }

    pub async fn delete_comment(&self, request: DeleteCommentRequest) -> Result<(), sqlx::Error> {
        self.repo.delete_comment(request).await
    }

    pub async fn get_comment(&self, request: CommentQuery) -> Result<Vec<CommentView>, sqlx::Error> {
        self.repo.get_comment(request).await
    }
}
