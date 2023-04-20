use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};

pub struct UserSubscription;

#[Subscription]
impl UserSubscription {
    async fn user_list(&self, mutation_type: Option<MutationType>) {}
}
