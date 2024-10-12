use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
}

pub struct InMemoryDb {
    posts: Vec<Post>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self { posts: Vec::new() }
    }

    pub fn create_post(&mut self, post: Post) -> Option<&Post> {
        self.posts.push(post);
        self.posts.last()
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

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}



#[post("/posts")]
pub async fn create_post(db: web::Data<Mutex<InMemoryDb>>, new_post: web::Json<Post>) -> impl Responder {
    let mut db = db.lock().expect("Failed to lock db");
    if let Some(post) = db.create_post(new_post.into_inner()) {
        HttpResponse::Created().json(post)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/posts")]
pub async fn list_posts(db: web::Data<Mutex<InMemoryDb>>) -> impl Responder {
    let db = db.lock().expect("Failed to lock db");
    HttpResponse::Ok().json(&db.posts)
}

#[get("/posts/{id}")]
pub async fn read_post(db: web::Data<Mutex<InMemoryDb>>, id: web::Path<String>) -> impl Responder {
    let db = db.lock().expect("Failed to lock db");
    if let Some(post) = db.read_post(&id) {
        HttpResponse::Ok().json(post)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[put("/posts/{id}")]
pub async fn update_post(db: web::Data<Mutex<InMemoryDb>>, id: web::Path<String>, updated_post: web::Json<Post>) -> impl Responder {
    let mut db = db.lock().expect("Failed to lock db");
    if let Some(post) = db.update_post(&id, updated_post.into_inner()) {
        HttpResponse::Ok().json(post)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post(db: web::Data<Mutex<InMemoryDb>>, id: web::Path<String>) -> impl Responder {
    let mut db = db.lock().expect("Failed to lock db");
    if db.delete_post(&id).is_some() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
