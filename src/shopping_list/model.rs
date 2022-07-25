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
    pub state: String,
    pub created_at: String,
    pub completed_at: Option<String>, // mark a final state (e.g. ACQUIRED)
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

    pub async fn fetch_all_by_state(
        state: ShoppingListItemState,
        db_pool: &SqlitePool,
    ) -> Result<FindAllShoppingListItemsResponse> {
        let state_value = state.value();
        let items = sqlx::query_as!(
            ShoppingListItem,
            r#"
                SELECT id, name, quantity, image, state, created_at, completed_at
                FROM shopping_list_item
                WHERE state = $1
                "#,
            state_value
        )
        .fetch_all(&*db_pool)
        .await?;

        Ok(FindAllShoppingListItemsResponse { items })
    }

    pub async fn acquire(id: String, db_pool: &SqlitePool) -> Result<()> {
        let state = ShoppingListItemState::Acquired.value();
        let completed_at = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

        let mut tx = db_pool.begin().await?;

        sqlx::query!(
            r#"
                UPDATE shopping_list_item
                SET state = $1, completed_at = $2
                WHERE id = $3
                "#,
            state,
            completed_at,
            id
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn release(id: String, db_pool: &SqlitePool) -> Result<()> {
        let state = ShoppingListItemState::Pending.value();
        
        let mut tx = db_pool.begin().await?;

        sqlx::query!(
            r#"
                UPDATE shopping_list_item
                SET state = $1, completed_at = null
                WHERE id = $2
                "#,
            state,
            id
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
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

pub enum ShoppingListItemState {
    Pending,
    Acquired,
}

impl ShoppingListItemState {
    pub fn value(&self) -> &str {
        match self {
            ShoppingListItemState::Pending => "PENDING",
            ShoppingListItemState::Acquired => "ACQUIRED",
        }
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
