use anyhow::Result;
use chrono::{SecondsFormat, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Serialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListItem {
    pub id: String,
    pub name: String,
    pub quantity: String,
    pub image: Option<String>, // Base64
    pub created_at: String,
}

impl ShoppingListItem {
    pub async fn create(
        request: CreateShoppingListItemRequest,
        db_pool: &SqlitePool,
    ) -> Result<()> {
        let id = nanoid!();
        let created_at = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

        let mut tx = db_pool.begin().await?;

        sqlx::query!(
            r#"
                INSERT INTO shopping_list_item(id, name, quantity, image, created_at)
                VALUES($1, $2, $3, $4, $5)
                "#,
            id,
            request.name,
            request.quantity,
            request.image,
            created_at
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn find_all(db_pool: &SqlitePool) -> Result<FindAllShoppingListItemsResponse> {
        let items = sqlx::query_as!(
            ShoppingListItem,
            r#"
                SELECT id, name, quantity, image, created_at
                FROM shopping_list_item
                "#
        )
        .fetch_all(&*db_pool)
        .await?;

        Ok(FindAllShoppingListItemsResponse { items })
    }

    pub async fn delete(id: String, db_pool: &SqlitePool) -> Result<()> {
        let mut tx = db_pool.begin().await?;

        sqlx::query!(
            r#"
                DELETE FROM shopping_list_item
                WHERE id = $1
                "#,
            id
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct CreateShoppingListItemRequest {
    name: String,
    quantity: String,
    image: Option<String>, // Base64
}

#[derive(Serialize)]
pub struct FindAllShoppingListItemsResponse {
    pub items: Vec<ShoppingListItem>,
}
