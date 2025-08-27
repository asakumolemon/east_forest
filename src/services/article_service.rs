use crate::database::repositories::ArticleRepository;
use crate::models::article::{ArticleQuery, ArticleView, CreateArticleRequest, UpdateArticleRequest, DeleteArticleRequest};

pub struct ArticleService {
    article_repository: ArticleRepository,
}

impl ArticleService {
    pub fn new(article_repository: ArticleRepository) -> Self {
        Self { article_repository }
    }

    pub async fn create_article(&self, request: CreateArticleRequest) -> Result<ArticleView, sqlx::Error> {
        let article = self.article_repository.create_article(request).await?;
        Ok(article)
    }

    pub async fn get_article(&self, request: ArticleQuery) -> Result<Vec<ArticleView>, sqlx::Error> {
        let articles = self.article_repository.get_article(request).await?;
        Ok(articles)
    }

    pub async fn update_article(&self, request: UpdateArticleRequest) -> Result<ArticleView, sqlx::Error> {
        let article = self.article_repository.update_article(request).await?;
        Ok(article)
    }

    pub async fn delete_article(&self, request: DeleteArticleRequest) -> Result<(), sqlx::Error> {
        self.article_repository.delete_article(request).await?;
        Ok(())
    }
}