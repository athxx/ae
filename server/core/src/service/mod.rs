mod post;
mod user;

pub mod schema;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(post::query::PostQuery, user::query::UserQuery);

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(post::mutation::PostMutation, user::mutation::UserMutation);

#[derive(async_graphql::MergedSubscription, Default)]
pub struct Subscription(user::subsciption::UserSubscription);
