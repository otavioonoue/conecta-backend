use std::{error::Error, sync::Arc};

use axum::Router;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::{
  modules::public::{consultant::{ConsultantAppState, application::usecase::{create_consultant_usecase::CreateConsultantUseCase, get_all_consultant_usecase::GetAllConsultantsUseCase}, infrastructure::database::repository::consultant_repository_impl::ConsultantRepositoryImpl, presentation::controller::consultant_controller::consultant_router}, user::{
    UserAppState, application::usecase::{create_user_usecase::CreateUserUseCase, get_all_users_usecase::GetAllUsersUseCase}, infrastructure::database::repository::user_repository_impl::UserRepositoryImpl, presentation::controller::user_controller::user_router
  }},
  shared::infra::database::db_config::{Database, Db},
};

pub mod modules;
pub mod shared;

pub async fn api() -> Result<(), Box<dyn Error>> {
  let db_url: String = std::env::var("DATABASE_URL").expect("Coundn't find the DATABASE_URL");
  let db = Database::<Pool<Postgres>>::new(&db_url).await;
  sqlx::migrate!("./migrations").run(&*db.pool).await?;
  
  let user_app_state = UserAppState {
    user_repository: Arc::new(UserRepositoryImpl::new(db.clone())),
    create_user: Arc::new(CreateUserUseCase),
    get_all_users: Arc::new(GetAllUsersUseCase)
  };
  
  let consultant_app_state = ConsultantAppState {
    consultant_repository: Arc::new(ConsultantRepositoryImpl::new(db.clone())),
    get_all_consultant: Arc::new(GetAllConsultantsUseCase),
    create_consultant: Arc::new(CreateConsultantUseCase)
  };

  let app: Router = Router::new()
    .nest("/user", user_router(user_app_state))
    .nest("/consultant", consultant_router(consultant_app_state));

  let listener = TcpListener::bind("0.0.0.0:3000").await?;
  axum::serve(listener, app).await?;

  Ok(())
}

