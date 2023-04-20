use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::prisma::{post, user, PrismaClient};
use crate::service::user::types::User;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub user_id: String,
}

#[ComplexObject]
impl Post {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<Box<User>>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db.user().find_unique(user::id::equals(self.user_id.clone())).exec().await?.map(|u| Box::new(u.into())))
    }
}

impl From<post::Data> for Post {
    fn from(data: post::Data) -> Post {
        Post {
            id: data.id,
            content: data.content,
            user_id: data.user_id,
        }
    }
}
