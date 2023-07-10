use std::{error::Error, marker::PhantomData, str::FromStr, sync::Arc};

use cqrs_es::{
    persist::{PersistenceError, ViewContext, ViewRepository},
    Aggregate, View,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    ModelTrait, PrimaryKeyTrait,
};

pub mod lobby;

/// Provides the version column for a SeaORM Model.
pub trait MaterializedViewTrait {
    type VersionColumnType: ColumnTrait;

    fn version_column() -> Self::VersionColumnType;
}

pub struct SeaOrmViewRepository<V, A, T> 
where
    A: Aggregate, 
    // TODO: Figure out how to require IntoActiveModel here.
    V: View<A> + ModelTrait + MaterializedViewTrait + IntoActiveModel<T>,
    T: ActiveModelTrait<Entity = V::Entity> + Send + Sync + 'static,
    <<<<V as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType as FromStr>::Err:
        Into<Box<dyn Error + Send + Sync + 'static>>,
    <<<V as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: FromStr,
    Option<<<V as ModelTrait>::Entity as sea_orm::EntityTrait>::Model>: Into<Option<V>>,
    <V::Entity as EntityTrait>::Model: IntoActiveModel<T>
 {
    pub connection: Arc<DatabaseConnection>,
    pub phantom: PhantomData<(V, A, T)>,
}

impl<V, A, T> SeaOrmViewRepository<V, A, T>
where
    A: Aggregate, 
    V: View<A> + ModelTrait + MaterializedViewTrait + IntoActiveModel<T>,
    T: ActiveModelTrait<Entity = V::Entity> + Send + Sync + 'static,
    <<<<V as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType as FromStr>::Err:
        Into<Box<dyn Error + Send + Sync + 'static>>,
    <<<V as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: FromStr,
    Option<<<V as ModelTrait>::Entity as sea_orm::EntityTrait>::Model>: Into<Option<V>>,
    <V::Entity as EntityTrait>::Model: IntoActiveModel<T>
{
    pub async fn create_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        let res = <V::Entity as EntityTrait>::insert(view.into_active_model()).exec(self.connection.as_ref()).await.map_err(|e| PersistenceError::UnknownError(e.into()))?;
        Ok(())
    }

    pub async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        <V::Entity as EntityTrait>::update(view.into_active_model()).exec(self.connection.as_ref()).await.map_err(|e |PersistenceError::UnknownError(e.into()))?;
        Ok(())
    }
}

#[async_trait]
impl<V, A, T> ViewRepository<V, A> for SeaOrmViewRepository<V, A, T>
where
    A: Aggregate, 
    V: View<A> + ModelTrait + MaterializedViewTrait + IntoActiveModel<T>,
    T: ActiveModelTrait<Entity = V::Entity> + Send + Sync + 'static,
    <<<<V as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType as FromStr>::Err:
        Into<Box<dyn Error + Send + Sync + 'static>>,
    <<<V as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: FromStr,
    Option<<<V as ModelTrait>::Entity as sea_orm::EntityTrait>::Model>: Into<Option<V>>,
    <V::Entity as EntityTrait>::Model: IntoActiveModel<T>
{
    async fn load(&self, view_id: &str) -> Result<Option<V>, PersistenceError> {
        let key = <<<V::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType as FromStr>::from_str(view_id).map_err(|e| PersistenceError::UnknownError(e.into()))?;
        let view = V::Entity::find_by_id(key)
            .one(self.connection.as_ref())
            .await
            .map_err(|e| match e {
                DbErr::Conn(conn) => PersistenceError::ConnectionError(conn.into()),
                DbErr::ConnectionAcquire => PersistenceError::ConnectionError(
                    anyhow!("could not acquire a connection from the pool.").into(),
                ),
                _ => PersistenceError::UnknownError(e.into()),
            })?;

        return Ok(view.into());
    }

    async fn load_with_context(
        &self,
        view_id: &str,
    ) -> Result<Option<(V, ViewContext)>, PersistenceError> {
        let key = <<<V::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType as FromStr>::from_str(view_id).map_err(|e| PersistenceError::UnknownError(e.into()))?;
        let view_res = V::Entity::find_by_id(key)
            .one(self.connection.as_ref())
            .await
            .map_err(|e| match e {
                DbErr::Conn(conn) => PersistenceError::ConnectionError(conn.into()),
                DbErr::ConnectionAcquire => PersistenceError::ConnectionError(
                    anyhow!("could not acquire a connection from the pool.").into(),
                ),
                _ => PersistenceError::UnknownError(e.into()),
            })?.into();

        if let Some(view) = view_res {
            return Ok(Some((view, ViewContext::new(view_id.to_string(), 0))));
        } else {
            return Ok(None);
        }
    }

    async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        let exists = context.version > 0;
        if exists {
            self.update_view(view, context)
                .await?;
        } else {
            self.create_view(view, context)
                .await?;
        }
        Ok(())
    }
}
