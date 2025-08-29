use sqlx::PgPool;

use crate::models::user::*;
use crate::utils::auth_util::{hash_password, verify_password, create_jwt};
use crate::models::prompt::*;
use crate::models::article::*;
use crate::models::comment::*;
use crate::models::user_interaction::*;

use crate::database::connection::create_pool;
use crate::config::database_config::DatabaseConfig;

#[derive(Clone)]
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

    pub async fn create(&self, user: CreateUserRequest) -> Result<UserView, sqlx::Error> {
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

    pub async fn update(&self, user: UpdateUserRequest) -> Result<UserView, sqlx::Error> {
        let password_hash = hash_password(&user.password.unwrap_or("".to_string()));
        sqlx::query_as("UPDATE users SET username = $1, email = $2, password_hash = $3 , avatar_url = $4, bio = $5 
            WHERE id = $6 RETURNING id, username, email, created_at, updated_at, avatar_url, bio")
            .bind(user.username.unwrap_or("".to_string()))
            .bind(user.email.unwrap_or("".to_string()))
            .bind(password_hash)
            .bind(user.avatar_url.unwrap_or("".to_string()))
            .bind(user.bio.unwrap_or("".to_string()))
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
        let password_hash = user.password_hash;
        if !verify_password(&auth_user.password, &password_hash) {
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
        let mut where_conditions = Vec::new();
        let mut bind_params = Vec::new();
        let mut param_index = 1;
    
        if let Some(id) = &query.id {
            where_conditions.push(format!("id LIKE ${}", param_index));
            bind_params.push(format!("%{}%", id));
            param_index += 1;
        }
        
        if let Some(username) = &query.username {
            where_conditions.push(format!("username LIKE ${}", param_index));
            bind_params.push(format!("%{}%", username));
            param_index += 1;
        }
        
        if let Some(email) = &query.email {
            where_conditions.push(format!("email LIKE ${}", param_index));
            bind_params.push(format!("%{}%", email));
        }
    
        let base_query = "SELECT id, username, email, avatar_url, bio, created_at, updated_at FROM users";
        let full_query = if where_conditions.is_empty() {
            base_query.to_string()
        } else {
            format!("{} WHERE {}", base_query, where_conditions.join(" OR "))
        };
    
        let mut query_builder = sqlx::query_as::<_, UserView>(&full_query);
        
        for param in bind_params {
            query_builder = query_builder.bind(param);
        }
    
        let users = query_builder.fetch_all(&self.pool).await?;
        
        Ok(users)
    }

    pub async fn get_user(&self, query: UserQuery) -> Result<UserView, sqlx::Error> {
        let user = sqlx::query_as("SELECT id, username, email, avatar_url, bio, created_at, updated_at FROM users where id = $1")
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
        let mut sql = "SELECT * FROM prompts WHERE 1=1".to_string();
        let mut conditions = Vec::new();
        let mut param_count = 1;
        
        if query.id.is_some() {
            sql.push_str(&format!(" AND id = ${}", param_count));
            conditions.push(query.id);
            param_count += 1;
        }
        if let Some(title) = query.title {
            sql.push_str(&format!(" AND title LIKE ${}", param_count));
            conditions.push(Some(format!("{}%", title)));
            param_count += 1;
        }
        if let Some(category) = query.category {
            sql.push_str(&format!(" AND category LIKE ${}", param_count));
            conditions.push(Some(format!("{}%", category)));
            param_count += 1;
        }
        if let Some(content) = query.content {
            sql.push_str(&format!(" AND content LIKE ${}", param_count));
            conditions.push(Some(format!("{}%", content)));
            param_count += 1;
        }
        if let Some(difficulty_level) = query.difficulty_level {
            sql.push_str(&format!(" AND difficulty_level = ${}", param_count));
            conditions.push(Some(difficulty_level.to_string()));
        }
        
        let mut prompts = sqlx::query_as::<_, PromptView>(&sql);
        for condition in conditions {
            if let Some(value) = condition {
                prompts = prompts.bind(value);
            }
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
        let mut sql = "SELECT * FROM articles WHERE 1=1".to_string();
        let mut param_count = 1;
        
        if query.id.is_some() {
            sql.push_str(&format!(" AND id = ${}", param_count));
            param_count += 1;
        }
        if query.user_id.is_some() {
            sql.push_str(&format!(" AND user_id = ${}", param_count));
            param_count += 1;
        }
        if query.prompt_id.is_some() {
            sql.push_str(&format!(" AND prompt_id = ${}", param_count));
            param_count += 1;
        }
        if query.title.is_some() {
            sql.push_str(&format!(" AND title LIKE ${}", param_count));
            param_count += 1;
        }
        if query.is_public.is_some() {
            sql.push_str(&format!(" AND is_public = ${}", param_count));
            param_count += 1;
        }
        if query.ai_score.is_some() {
            sql.push_str(&format!(" AND ai_score = ${}", param_count));
        }
        
        let mut articles = sqlx::query_as::<_, ArticleView>(&sql);
        
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
            articles = articles.bind(format!("%{}%", title));
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


pub struct CommentRepository { 
    pub pool: PgPool
}

impl CommentRepository { 
    pub async fn new() -> Self {
        let config = DatabaseConfig::default();
        Self {
            pool: create_pool(config).await,
        }
    }

    pub async fn create_comment(&self, request: CreateCommentRequest) -> Result<CommentView, sqlx::Error> { 
        let comment = sqlx::query_as::<_, CommentView>("INSERT INTO comments (article_id, user_id, content) VALUES ($1, $2, $3) RETURNING *")
            .bind(request.article_id)
            .bind(request.user_id)
            .bind(request.content)
            .fetch_one(&self.pool)
            .await?;
        Ok(comment)
    }

    pub async fn update_comment(&self, request: UpdateCommentRequest) -> Result<CommentView, sqlx::Error> { 
        let comment = sqlx::query_as::<_, CommentView>("UPDATE comments SET content = $1 WHERE id = $2 RETURNING *")
            .bind(request.content)
            .bind(request.id)
            .fetch_one(&self.pool)
            .await?;
        Ok(comment)
    }

    pub async fn delete_comment(&self, request: DeleteCommentRequest) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM comments WHERE id = $1")
            .bind(request.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_comment(&self, query: CommentQuery) -> Result<Vec<CommentView>, sqlx::Error> {
        let mut sql = "SELECT * FROM comments".to_string();
        let mut conditions = Vec::new();
        let mut param_count = 1;
        
        if query.article_id.is_some() {
            conditions.push(format!("article_id = ${}", param_count));
            param_count += 1;
        }
        if query.user_id.is_some() {
            conditions.push(format!("user_id = ${}", param_count));
            param_count += 1;
        }
        if query.content.is_some() {
            conditions.push(format!("content = ${}", param_count));
        }
        
        if !conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }
        
        let mut comments = sqlx::query_as::<_, CommentView>(&sql);
        
        if let Some(article_id) = query.article_id {
            comments = comments.bind(article_id);
        }
        if let Some(user_id) = query.user_id {
            comments = comments.bind(user_id);
        }
        if let Some(content) = query.content {
            comments = comments.bind(content);
        }
        
        let comments = comments.fetch_all(&self.pool).await?;
        Ok(comments)
    }
}

pub struct UserInteractionRepository { 
    pool: PgPool
}

impl UserInteractionRepository { 
    pub async fn new() -> Self {
        let config = DatabaseConfig::default();
        Self {
            pool: create_pool(config).await,
        }
    }

    pub async fn create_user_interaction(&self, request: CreateUserInteractionRequest) -> Result<UserInteractionView, sqlx::Error> { 
        let user_interaction = sqlx::query_as::<_, UserInteractionView>(
            "INSERT INTO user_interactions (user_id, prompt_id, article_id, comment_id, interaction_type) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(request.user_id)
        .bind(request.prompt_id)
        .bind(request.article_id)
        .bind(request.comment_id)
        .bind(request.interaction_type)
        .fetch_one(&self.pool)
        .await?;
        Ok(user_interaction)
    }

    pub async fn get_user_interaction(&self, query: UserInteractionQuery) -> Result<Vec<UserInteractionView>, sqlx::Error> {
        let mut sql = "SELECT * FROM user_interactions WHERE 1 = 1".to_string();
        if query.user_id.is_some() {
            sql.push_str(" AND user_id = ?");
        }
        if query.prompt_id.is_some() {
            sql.push_str(" AND prompt_id = ?");
        }
        if query.article_id.is_some() {
            sql.push_str(" AND article_id = ?");
        }
        if query.comment_id.is_some() {
            sql.push_str(" AND comment_id = ?");
        }
        if query.interaction_type.is_some() {
            sql.push_str(" AND interaction_type = ?");
        }
        
        let mut user_interactions = sqlx::query_as::<_, UserInteractionView>(&sql);
        
        if let Some(user_id) = query.user_id {
            user_interactions = user_interactions.bind(user_id);
        }
        if let Some(prompt_id) = query.prompt_id {
            user_interactions = user_interactions.bind(prompt_id);
        }
        if let Some(article_id) = query.article_id {
            user_interactions = user_interactions.bind(article_id);
        }
        if let Some(comment_id) = query.comment_id {
            user_interactions = user_interactions.bind(comment_id);
        }
        if let Some(interaction_type) = query.interaction_type {
            user_interactions = user_interactions.bind(interaction_type);
        }
        
        let user_interactions = user_interactions.fetch_all(&self.pool).await?;
        Ok(user_interactions)
    }

    pub async fn delete_user_interaction(&self, request: DeleteUserInteractionRequest) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_interactions WHERE id = $1")
            .bind(request.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}