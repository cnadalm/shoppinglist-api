use actix_web::{get, post, delete, web, HttpResponse, Responder};
use log::error;
use sqlx::SqlitePool;

use crate::shopping_list::{ShoppingListItem, CreateShoppingListItemRequest};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/shoppinglist/api/v1")
        .service(web::scope("/items")
            .service(create)
            .service(find_all)
            .service(delete)
    ));
}

#[post("")]
async fn create(request: web::Json<CreateShoppingListItemRequest>, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = ShoppingListItem::create(request.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(()) => HttpResponse::Created().json("Item created successfully"),
        Err(err) => {
            error!("error creating shopping list item: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create a shopping list item")
        }
    }
}

#[get("")]
async fn find_all(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = ShoppingListItem::find_all(db_pool.get_ref()).await;
    match result {
        Ok(response) => HttpResponse::Ok().json(response.items),
        Err(err) => {
            error!("error finding all shopping list items: {}", err);
            HttpResponse::InternalServerError().body("Error trying to find all shopping list items")
        }
    }
}

#[delete("/{id}")]
async fn delete(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let result = ShoppingListItem::delete(path.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(()) => HttpResponse::Ok().json("Item deleted successfully"),
        Err(err) => {
            error!("error deleting shopping list item: {}", err);
            HttpResponse::InternalServerError().body("Error trying to delete a shopping list item")
        }
    }
}