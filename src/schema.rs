use juniper::{FieldResult};

use uuid::Uuid;

#[derive(juniper::GraphQLEnum, Clone)]
enum UserType {
  Admin,
  User
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A user within Banur")]
struct User {
  id: String,
  email: String,
  user_type: UserType
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description="A new user to insert")]
struct NewUser {
  email: String,
  user_type: UserType
}

pub struct Context {}

impl juniper::Context for Context {}

pub struct Query;
#[juniper::object(Context = Context)]
impl Query {
  fn apiVersion() -> &str {
    "0.1"
  }

  fn user(context: &Context, id: String) -> FieldResult<User> {
    Ok(User { id: Uuid::new_v4().to_string(), email: String::from("test@test.xyz"), user_type: UserType::User})
  }
}

pub struct Mutation;
#[juniper::object(Context = Context)]
impl Mutation {
  fn createUser(context: &Context, new_user: NewUser) -> FieldResult<User> {
    Ok(User { id: Uuid::new_v4().to_string(), email: new_user.email.clone(), user_type: new_user.user_type.clone() })
  }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
  Schema::new(Query {}, Mutation {})
}
