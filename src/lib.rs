use std::{error::Error, sync::Arc};

use axum::Router;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::{
    modules::public::user::{
        application::usecase::{create_user_usecase::CreateUserUseCase, get_all_users_usecase::GetAllUsersUseCase}, infrastructure::database::repository::user_repository_impl::UserRepositoryImpl, presentation::controller::user_controller::user_router, UserAppState
    },
    shared::infra::database::db_config::{Database, Db},
};

mod modules;
mod shared;

pub async fn api() -> Result<(), Box<dyn Error>> {
    let db_url: String = std::env::var("DATABASE_URL").expect("Coundnt find the DATABASE_URL");
    let db = Database::<Pool<Postgres>>::new(&db_url).await;
    sqlx::migrate!("./migrations").run(&*db.pool).await?;
    
    let user_app_state = Arc::new(UserAppState {
        user_repository: UserRepositoryImpl::new(db.clone()),
        create_user: CreateUserUseCase,
        get_all_users: GetAllUsersUseCase
    });

    let app: Router = Router::new().nest("/user", user_router(user_app_state.clone()));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

