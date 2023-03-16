//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "column_types")]
pub enum ColumnTypes {
    #[sea_orm(string_value = "mail")]
    Mail,
    #[sea_orm(string_value = "number")]
    Number,
    #[sea_orm(string_value = "relation")]
    Relation,
    #[sea_orm(string_value = "text")]
    Text,
}
