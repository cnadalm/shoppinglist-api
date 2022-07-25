use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;
use sqlx::SqlitePool;

use crate::shopping_list::{
    CreateShoppingListItemRequest, ShoppingListItem, ShoppingListItemState,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shoppinglist/api/v1").service(
            web::scope("/items")
                .service(create)
                .service(fetch_all_pending)
                .service(fetch_all_acquired)
                .service(acquire)
                .service(release)
                .service(delete),
        ),
    );
}

#[post("")]
async fn create(
    request: web::Json<CreateShoppingListItemRequest>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let result = ShoppingListItem::create(request.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(()) => HttpResponse::Created().json("Item created successfully"),
        Err(err) => {
            error!("error creating shopping list item: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create a shopping list item")
        }
    }
}

#[get("/pending")]
async fn fetch_all_pending(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result =
        ShoppingListItem::fetch_all_by_state(ShoppingListItemState::Pending, db_pool.get_ref())
            .await;
    match result {
        Ok(response) => HttpResponse::Ok().json(response.items),
        Err(err) => {
            error!("error fetching all pending shopping list items: {}", err);
            HttpResponse::InternalServerError()
                .body("Error trying to fetch all pending shopping list items")
        }
    }
}

#[get("/acquired")]
async fn fetch_all_acquired(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result =
        ShoppingListItem::fetch_all_by_state(ShoppingListItemState::Acquired, db_pool.get_ref())
            .await;
    match result {
        Ok(response) => HttpResponse::Ok().json(response.items),
        Err(err) => {
            error!("error fetching all acquired shopping list items: {}", err);
            HttpResponse::InternalServerError()
                .body("Error trying to fetch all acquired shopping list items")
        }
    }
}

#[put("/{id}/acquire")]
async fn acquire(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let result = ShoppingListItem::acquire(path.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(()) => HttpResponse::Ok().json("Item acquired successfully"),
        Err(err) => {
            error!("error acquiring shopping list item: {}", err);
            HttpResponse::InternalServerError().body("Error trying to acquire a shopping list item")
        }
    }
}

#[delete("/{id}/acquire")]
async fn release(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let result = ShoppingListItem::release(path.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(()) => HttpResponse::Ok().json("Item released successfully"),
        Err(err) => {
            error!("error releasing shopping list item: {}", err);
            HttpResponse::InternalServerError().body("Error trying to acquire a shopping list item")
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
