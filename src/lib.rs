use std::{error::Error, sync::Arc};

use axum::Router;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::{
  modules::public::{auth::{AuthAppState, application::usecase::login_usecase::LoginUseCase, infrastructure::database::repository::auth_repository_impl::AuthRepositoryImpl, presentation::controller::auth_controller::auth_router}, consultant::{ConsultantAppState, application::usecase::{add_service_consultant_usecase::AddServiceConsultantUseCase, create_consultant_usecase::CreateConsultantUseCase, get_all_consultant_usecase::GetAllConsultantsUseCase, remove_service_consultant_usecase::RemoveServiceConsultantUseCase}, infrastructure::database::repository::consultant_repository_impl::ConsultantRepositoryImpl, presentation::controller::consultant_controller::consultant_router}, service::{ServiceAppState, application::usecase::{create_service_usecase::CreateServiceUseCase, get_all_service_usecase::GetAllServicesUseCase}, infrastructure::database::repository::service_repository_impl::ServiceRepositoryImpl, presentation::controller::service_controller::service_router}, user::{
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
    
    let auth_app_state = AuthAppState {
        auth_repository: Arc::new(AuthRepositoryImpl::new(db.clone())),
        login: Arc::new(LoginUseCase)
    };
    
    let user_app_state = UserAppState {
        user_repository: Arc::new(UserRepositoryImpl::new(db.clone())),
        create_user: Arc::new(CreateUserUseCase),
        get_all_users: Arc::new(GetAllUsersUseCase)
    };
    
    let consultant_app_state = ConsultantAppState {
        consultant_repository: Arc::new(ConsultantRepositoryImpl::new(db.clone())),
        get_all_consultant: Arc::new(GetAllConsultantsUseCase),
        create_consultant: Arc::new(CreateConsultantUseCase),
        add_service: Arc::new(AddServiceConsultantUseCase),
        remove_service: Arc::new(RemoveServiceConsultantUseCase)
    };
    
    let service_app_state = ServiceAppState {
        service_repository: Arc::new(ServiceRepositoryImpl::new(db.clone())),
        get_all_services: Arc::new(GetAllServicesUseCase),
        create_service: Arc::new(CreateServiceUseCase)
    };
    
    let app: Router = Router::new()
        .nest("/auth", auth_router(auth_app_state))
        .nest("/user", user_router(user_app_state))
        .nest("/consultant", consultant_router(consultant_app_state))
        .nest("/service", service_router(service_app_state));
    
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

