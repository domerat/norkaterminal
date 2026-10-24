use crate::domain::exceptions::DomainException;
use crate::infrastructure::adapters::outbound::dto::{
    DBCreateNewUserRequest,
    DBCreateNewUserResponse,
    DBGetUserRequest,
    DBGetUserResponse
};

use sea_orm::EntityTrait;
use sea_orm::ActiveModelTrait;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

#[derive(Clone)]
pub struct UserAdapter {
    database: sea_orm::DatabaseConnection
}

impl UserAdapter {
    pub fn new(database: sea_orm::DatabaseConnection) -> Self {
        Self {
            database
        }
    }
}

#[async_trait::async_trait]
impl crate::application::ports::OutboundUserPort for UserAdapter {
     async fn create_user(&self, data: DBCreateNewUserRequest) -> Result<DBCreateNewUserResponse, DomainException> {
        let uuid_ = uuid::Uuid::new_v4().to_string();

         let active_model = crate::domain::models::user::ActiveModel{
             id: sea_orm::Set(uuid_),
             username: sea_orm::Set(data.username),
             password: sea_orm::Set(data.password),
             name: sea_orm::Set(data.name),
             email: sea_orm::Set(data.email),
             group: sea_orm::Set(data.group)
         };

         let model_ : crate::domain::models::user::Model = active_model.insert(&self.database)
             .await
             .inspect(|model| {
                 tracing::info!("New user created! ID = {}", model.id);
             })
            .map_err(|err| {
                tracing::error!("Error occurred on the layer-3: {}", err);
                DomainException::DatabaseInsertionError("users".to_string())
            })?;

        Ok(DBCreateNewUserResponse::new(model_.id))
     }

    async fn get_user(&self, data: DBGetUserRequest) -> Result<DBGetUserResponse, DomainException> {
        let user = crate::domain::models::user::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(crate::domain::models::user::Column::Id.eq(data.id))
                    .add(crate::domain::models::user::Column::Username.eq(data.username))
                    .add(crate::domain::models::user::Column::Email.eq(data.email))
            )
            .require_one(&self.database)
            .await
            .map_err(|err| {
                tracing::error!("Cannot get a user: {}", err.to_string());
                DomainException::DatabaseSearchError("users".to_string())}
            )?;

        Ok(DBGetUserResponse::new(user.username, user.password, user.email, user.name, user.group))
    }
}


