use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub tag: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl Related<super::url::Entity> for Entity {
    fn to() -> RelationDef {
        super::url_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::url_tag::Relation::Url.def().rev())
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
