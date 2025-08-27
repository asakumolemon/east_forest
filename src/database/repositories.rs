use sqlx::PgPool;
use crate::models::user::{User, UserView, CreateUserRequest, UpdateUserRequest, DeleteUserRequest, AuthUserRequest, AuthUserResponse, UserQuery};
use crate::utils::auth_util::{hash_password, verify_password, create_jwt};
use crate::models::{article, prompt::*};
use crate::models::article::*;

use crate::database::connection::create_pool;
use crate::config::database_config::DatabaseConfig;

pub struct UserRepository { 
    pool: PgPool,
}

impl UserRepository { 
    pub async fn new() -> Self {
        let config = DatabaseConfig::default();
        Self {
            pool: create_pool(config).await,
        }
    }

    pub async fn create(&self, user: CreateUserRequest) -> Result<User, sqlx::Error> {
        let hashed_password = hash_password(&user.password.unwrap_or("".to_string()));
        sqlx::query_as("INSERT INTO users (username, email, password_hash, avatar_url, bio) VALUES ($1, $2, $3, $4, $5) RETURNING id, username, email, avatar_url, bio, created_at, updated_at")
            .bind(user.username.unwrap_or("".to_string()))
            .bind(user.email.unwrap_or("".to_string()))
            .bind(hashed_password)
            .bind(user.avatar_url.unwrap_or("".to_string()))
            .bind(user.bio.unwrap_or("".to_string()))
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update(&self, user: UpdateUserRequest) -> Result<User, sqlx::Error> {
        let password_hash = hash_password(&user.password.unwrap_or("".to_string()));
        sqlx::query_as("UPDATE users SET name = $1, email = $2, password_hash = $3 WHERE id = $4 RETURNING id, name, email, created_at, updated_at")
            .bind(user.username.unwrap_or("".to_string()))
            .bind(user.email.unwrap_or("".to_string()))
            .bind(password_hash)
            .bind(user.id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn delete(&self, user: DeleteUserRequest) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn auth(&self, auth_user: AuthUserRequest) -> Result<AuthUserResponse, sqlx::Error> { 
        let user: User = sqlx::query_as("SELECT id, username, email, password_hash, avatar_url, bio FROM users WHERE username = $1")
            .bind(auth_user.username)
            .fetch_one(&self.pool)
            .await?;
        let password = user.password_hash;
        if !verify_password(&auth_user.password, &password) {
            return Err(sqlx::Error::RowNotFound);
        }else {
            return Ok(AuthUserResponse {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                avatar_url: user.avatar_url,
                bio: user.bio,
                token: create_jwt(user.id),
            });
        }
    }

    pub async fn get_all_users(&self, query: UserQuery) -> Result<Vec<UserView>, sqlx::Error> { 

        let mut users = sqlx::query_as("SELECT id, username, email, avatar_url, bio FROM users");
        if let Some(id) = query.id { 
            users = users.bind(id);
        }
        if let Some(username) = query.username {
            users = users.bind(format!("{}%", username));
        }
        if let Some(email) = query.email {
            users = users.bind(format!("{}%", email));
        }
        let users = users.fetch_all(&self.pool).await?;
        Ok(users.into_iter().map(|user: User| UserView {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
            created_at: user.created_at,
            updated_at: user.updated_at
        }).collect())
    }

    pub async fn get_user(&self, query: UserQuery) -> Result<UserView, sqlx::Error> {
        let user = sqlx::query_as("SELECT id, username, email, avatar_url, bio FROM users where id = $1")
            .bind(query.id.unwrap_or("".to_string()))
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

}


pub struct PromptRepository { 
    pool: PgPool,
}

impl PromptRepository { 
    pub async fn new() -> Self {
        let config = DatabaseConfig::default();
        Self {
            pool: create_pool(config).await,
        }
    }

    pub async fn get_all(&self, query: PromptQuery) -> Result<Vec<PromptView>, sqlx::Error> { 
        let mut prompts = sqlx::query_as::<_, PromptView>("SELECT * FROM prompts");
        if let Some(id) = query.id {
            prompts = prompts.bind(id);
        }
        if let Some(title) = query.title {
            prompts = prompts.bind(format!("{}%", title));
        }
        if let Some(category) = query.category {
            prompts = prompts.bind(format!("{}%", category));
        }
        if let Some(content) = query.content {
            prompts = prompts.bind(format!("{}%", content));
        }
        if let Some(difficulty_level) = query.difficulty_lecel {
            prompts = prompts.bind(difficulty_level);
        }
        let prompts = prompts.fetch_all(&self.pool).await?;
        Ok(prompts)
    }

    pub async fn create_prompt(&self, request: CreatePromptRequest) -> Result<PromptView, sqlx::Error> { 
        let prompt = sqlx::query_as::<_, PromptView>("INSERT INTO prompts (title, category, content, difficulty_level) VALUES ($1, $2, $3, $4) RETURNING id, title, category, content, difficulty_level, created_at, updated_at")
            .bind(request.title.unwrap_or("".to_string()))
            .bind(request.category.unwrap_or("".to_string()))
            .bind(request.content.unwrap_or("".to_string()))
            .bind(request.difficulty_level.unwrap_or(0))
            .fetch_one(&self.pool)
            .await?;
        Ok(prompt)
    }

    pub async fn update_prompt(&self, request: UpdatePromptRequest) -> Result<PromptView, sqlx::Error> { 
        let prompt = sqlx::query_as::<_, PromptView>("UPDATE prompts SET title = $1, category = $2, content = $3, difficulty_level = $4 WHERE id = $5 RETURNING id, title, category, content, difficulty_level, created_at, updated_at")
            .bind(request.title.unwrap_or("".to_string()))
            .bind(request.category.unwrap_or("".to_string()))
            .bind(request.content.unwrap_or("".to_string()))
            .bind(request.difficulty_level.unwrap_or(0))
            .bind(request.id)
            .fetch_one(&self.pool)
            .await?;
        Ok(prompt)
    }

    pub async fn delete_prompt(&self, request: DeletePromptRequest) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM prompts WHERE id = $1")
            .bind(request.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_prompt(&self, query: PromptQuery) -> Result<PromptView, sqlx::Error> {
        let prompt = sqlx::query_as::<_, PromptView>("SELECT * FROM prompts WHERE id = $1")
            .bind(query.id.unwrap_or("".to_string()))
            .fetch_one(&self.pool)
            .await?;
        Ok(prompt)
    }
}

pub struct ArticleRepository { 
    pub pool: PgPool
}

impl ArticleRepository { 
    pub async fn new() -> Self {
        let config = DatabaseConfig::default();
        Self {
            pool: create_pool(config).await,
        }
    }

    pub async fn create_article(&self, request: CreateArticleRequest) -> Result<ArticleView, sqlx::Error> { 
        let article = sqlx::query_as::<_, ArticleView>("INSERT INTO articles (user_id, prompt_id, title, word_count, is_public, content, ai_score) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *")
            .bind(request.user_id)
            .bind(request.prompt_id)
            .bind(request.title)
            .bind(request.word_count)
            .bind(request.is_public)
            .bind(request.content)
            .bind(request.ai_score)
            .fetch_one(&self.pool)
            .await?;
        Ok(article)
    }

    pub async fn get_article(&self, query: ArticleQuery) -> Result<Vec<ArticleView>, sqlx::Error> {
        let mut articles = sqlx::query_as::<_, ArticleView>("SELECT * FROM articles");
        if let Some(id) = query.id { 
            articles = articles.bind(id);
        }
        if let Some(user_id) = query.user_id {
            articles = articles.bind(user_id);
        }
        if let Some(prompt_id) = query.prompt_id {
            articles = articles.bind(prompt_id);
        }
        if let Some(title) = query.title {
            articles = articles.bind(title);
        }
        if let Some(is_public) = query.is_public {
            articles = articles.bind(is_public);
        }
        if let Some(ai_score) = query.ai_score {
            articles = articles.bind(ai_score);
        }
        let articles = articles.fetch_all(&self.pool).await?;
        Ok(articles)
    }

    pub async fn update_article(&self, request: UpdateArticleRequest) -> Result<ArticleView, sqlx::Error> {
        let article = sqlx::query_as::<_, ArticleView>("UPDATE articles SET user_id = $1, prompt_id = $2, title = $3, word_count = $4, is_public = $5, content = $6, ai_score = $7 WHERE id = $8 RETURNING *")
            .bind(request.user_id)
            .bind(request.prompt_id)
            .bind(request.title)
            .bind(request.word_count)
            .bind(request.is_public)
            .bind(request.content)
            .bind(request.ai_score)
            .bind(request.id)
            .fetch_one(&self.pool)
            .await?;
        Ok(article)
    }

    pub async fn delete_article(&self, request: DeleteArticleRequest) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM articles WHERE id = $1")
            .bind(request.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }    
}
