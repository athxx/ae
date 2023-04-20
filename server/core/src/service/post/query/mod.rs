use async_graphql::{Context, Object, Result};

use crate::{
    prisma::{post, PrismaClient},
    service::post::types::Post,
};

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn get_posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let db = ctx.data::<PrismaClient>().unwrap();
        Ok(db.post().find_many(vec![]).exec().await?.into_iter().map(|p| p.into()).collect())
    }

    async fn get_post(&self, ctx: &Context<'_>, id: String) -> Result<Option<Post>> {
        let db = ctx.data::<PrismaClient>().unwrap();
        Ok(db.post().find_unique(post::id::equals(id)).exec().await?.map(|u| u.into()))
    }
}
