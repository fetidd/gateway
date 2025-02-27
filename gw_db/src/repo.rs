use crate::error::Result;

pub trait Repo {
    type Entity;
    type Id;
    type Filter;
    type Updater;

    async fn create(&self, entity: Self::Entity) -> Result<()>;
    async fn read_one(&self, id: Self::Id) -> Result<Self::Entity>;
    async fn read_many(&self, filter: Option<Self::Filter>) -> Result<Vec<Self::Entity>>;
    async fn delete_one(&self, id: Self::Id) -> Result<bool>;
    async fn delete_many(&self, filter: Option<Self::Filter>) -> Result<bool>;
    async fn update_one(&self, entity: Self::Entity) -> Result<bool>;
    async fn update_many(
        &self,
        filter: Option<Self::Filter>,
        update: Self::Updater,
    ) -> Result<bool>;
}
