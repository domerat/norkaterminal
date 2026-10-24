use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]     
    pub id: String,
    
    #[sea_orm(unique)]
    pub username: String,

    pub password: String,
    
    pub name: String,

    #[sea_orm(unique)] 
    pub email: String,

    pub group: String
}

// 5. Define table relationships (even if empty)
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// 6. Required hooks for inserts/updates
impl ActiveModelBehavior for ActiveModel {}

