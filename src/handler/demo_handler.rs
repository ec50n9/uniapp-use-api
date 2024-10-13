use actix_web::{delete, get, post, put, web, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::responses::CommonResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostCreateVO {
    title: String,
    content: String,
}

pub struct InMemoryDb {
    posts: Vec<Post>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self { posts: Vec::new() }
    }

    pub fn create_post(&mut self, post: PostCreateVO) -> Option<Post> {
        let post = Post {
            id: self.posts.len().to_string(),
            title: post.title,
            content: post.content,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: None,
        };
        self.posts.push(post);
        self.posts.last().cloned()
    }

    pub fn read_post(&self, id: &str) -> Option<&Post> {
        self.posts.iter().find(|p| p.id == id)
    }

    pub fn update_post(&mut self, id: &str, updated_post: Post) -> Option<&Post> {
        if let Some(post) = self.posts.iter_mut().find(|p| p.id == id) {
            *post = updated_post;
            Some(post)
        } else {
            None
        }
    }

    pub fn delete_post(&mut self, id: &str) -> Option<Post> {
        let index = self.posts.iter().position(|p| p.id == id)?;
        Some(self.posts.remove(index))
    }
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    // HttpResponse::Ok().body("Hello, world!")
    CommonResponse::ok("Hello, world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    // HttpResponse::Ok().body(req_body)
    CommonResponse::ok(req_body)
}

#[post("/posts")]
pub async fn create_post(
    db: web::Data<Mutex<InMemoryDb>>,
    new_post: web::Json<PostCreateVO>,
) -> impl Responder {
    let mut db = db.lock().expect("Failed to lock db");
    if let Some(post) = db.create_post(new_post.into_inner()) {
        CommonResponse::ok(post)
    } else {
        CommonResponse::error(-1, "创建文章失败".to_string())
    }
}

#[get("/posts")]
pub async fn list_posts(db: web::Data<Mutex<InMemoryDb>>) -> impl Responder {
    let db = db.lock().expect("Failed to lock db");
    CommonResponse::ok(db.posts.clone())
}

#[get("/posts/{id}")]
pub async fn read_post(db: web::Data<Mutex<InMemoryDb>>, id: web::Path<String>) -> impl Responder {
    let db = db.lock().expect("Failed to lock db");
    if let Some(post) = db.read_post(&id) {
        CommonResponse::ok(post.clone())
    } else {
        CommonResponse::error(-1, "未找到文章".to_string())
    }
}

#[put("/posts/{id}")]
pub async fn update_post(
    db: web::Data<Mutex<InMemoryDb>>,
    id: web::Path<String>,
    updated_post: web::Json<Post>,
) -> impl Responder {
    let mut db = db.lock().expect("Failed to lock db");
    if let Some(post) = db.update_post(&id, updated_post.into_inner()) {
        CommonResponse::ok(post.clone())
    } else {
        CommonResponse::error(-1, "未找到文章".to_string())
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post(
    db: web::Data<Mutex<InMemoryDb>>,
    id: web::Path<String>,
) -> impl Responder {
    let mut db = db.lock().expect("Failed to lock db");
    if db.delete_post(&id).is_some() {
        CommonResponse::ok("删除成功".to_string())
    } else {
        CommonResponse::error(-1, "未找到文章".to_string())
    }
}
