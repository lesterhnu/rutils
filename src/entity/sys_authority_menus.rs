//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_authority_menus")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_base_menu_id: u64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_authority_authority_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
