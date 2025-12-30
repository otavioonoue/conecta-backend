use std::{error::Error, sync::Arc};

use axum::{Router, extract::{Query, State, WebSocketUpgrade}, response::IntoResponse, routing::{get, post}};
use http::StatusCode;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::{
  modules::public::{auth::{AuthAppState, application::usecase::{login_consultant_usecase::LoginConsultantUseCase, login_usecase::LoginUseCase}, infrastructure::{database::repository::auth_repository_impl::AuthRepositoryImpl, jwt::claim::{Claims, decode_token}}, presentation::controller::auth_controller::auth_router}, consultant::{ConsultantAppState, application::usecase::{add_service_consultant_usecase::AddServiceConsultantUseCase, create_consultant_usecase::CreateConsultantUseCase, find_all_by_service_usecase::FindAllByServiceUseCase, get_all_consultant_usecase::GetAllConsultantsUseCase, remove_service_consultant_usecase::RemoveServiceConsultantUseCase}, infrastructure::database::repository::consultant_repository_impl::ConsultantRepositoryImpl, presentation::controller::consultant_controller::consultant_router}, payment::{application::usecase::create_visit_payment_usecase::CreateVisitPaymentUseCase, appstate::PaymentAppState, infrastructure::{database::repository::{payment_repository_impl::PaymentRepositoryImpl, service_repository_impl::ServiceRepositoryImpl as PaymentServiceRepositoryImpl}, service::payment_service_impl::PaymentServiceAsaasImpl}, presentation::controller::payment_controller::payment_router}, service::{application::usecase::{create_service_usecase::CreateServiceUseCase, get_all_service_usecase::GetAllServicesUseCase, schedule_service_usecase::ScheduleServiceUseCase}, appstate::ServiceAppState, infrastructure::database::repository::{address_repository_impl::AddressRepositoryImpl as ServiceAddressRepositoryImpl, consultant_repository_impl::ConsultantRepositoryImpl as ConsultantServiceRepositoryImpl, service_repository_impl::ServiceRepositoryImpl}, presentation::controller::service_controller::service_router}, user::{
    UserAppState, application::usecase::{create_address_usecase::CreateAddressUseCase, create_user_usecase::CreateUserUseCase, get_all_addresses_usecase::GetAllAddressesUseCase, get_all_users_usecase::GetAllUsersUseCase}, infrastructure::database::repository::{address_repository_impl::AddressRepositoryImpl, user_repository_impl::UserRepositoryImpl}, presentation::controller::user_controller::user_router
  }},
  shared::{infra::{database::db_config::{Database, Db}, error::AppError, service::{hash_service::HashServiceImpl, notification_service::NotificationServiceImpl}, ws::ws_impl::WsHub}, presentation::{controller::ws_controller::handle_socket, types::ApiResult}}
};

pub mod modules;
pub mod shared;

pub async fn api() -> Result<(), Box<dyn Error>> {
    let db_url: String = std::env::var("DATABASE_URL").expect("Coundn't find the DATABASE_URL");
    let db = Database::<Pool<Postgres>>::new(&db_url).await;
    sqlx::migrate!("./migrations").run(&*db.pool).await?;
    
    let hub = WsHub::new();
    
    let auth_app_state = AuthAppState {
        auth_repository: Arc::new(AuthRepositoryImpl::new(db.clone())),
        hash_service: Arc::new(HashServiceImpl::new()),
        login: Arc::new(LoginUseCase),
        login_consultant: Arc::new(LoginConsultantUseCase)
    };
    
    let user_app_state = UserAppState {
        user_repository: Arc::new(UserRepositoryImpl::new(db.clone())),
        address_repository: Arc::new(AddressRepositoryImpl::new(db.clone())),
        hash_service: Arc::new(HashServiceImpl::new()),
        create_user: Arc::new(CreateUserUseCase),
        get_all_users: Arc::new(GetAllUsersUseCase),
        get_all_addresses: Arc::new(GetAllAddressesUseCase),
        create_address: Arc::new(CreateAddressUseCase)
    };
    
    let consultant_app_state = ConsultantAppState {
        consultant_repository: Arc::new(ConsultantRepositoryImpl::new(db.clone())),
        hash_service: Arc::new(HashServiceImpl::new()),
        get_all_consultant: Arc::new(GetAllConsultantsUseCase),
        create_consultant: Arc::new(CreateConsultantUseCase),
        add_service: Arc::new(AddServiceConsultantUseCase),
        remove_service: Arc::new(RemoveServiceConsultantUseCase),
        find_all_by_service: Arc::new(FindAllByServiceUseCase)
    };
    
    let service_app_state = ServiceAppState {
        service_repository: Arc::new(ServiceRepositoryImpl::new(db.clone())),
        consultant_repository: Arc::new(ConsultantServiceRepositoryImpl::new(db.clone())),
        address_repository: Arc::new(ServiceAddressRepositoryImpl::new(db.clone())),
        get_all_services: Arc::new(GetAllServicesUseCase),
        create_service: Arc::new(CreateServiceUseCase),
        schedule_service: Arc::new(ScheduleServiceUseCase)
    };
    
    let payment_app_state = PaymentAppState {
        payment_service: Arc::new(PaymentServiceAsaasImpl::new()),
        payment_repository: Arc::new(PaymentRepositoryImpl::new(db.clone())),
        service_repository: Arc::new(PaymentServiceRepositoryImpl::new(db.clone())),
        notification_service: Arc::new(NotificationServiceImpl::new(db.clone(), hub.clone())),
        create_visit_payment: Arc::new(CreateVisitPaymentUseCase)
    };
    
    let app: Router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(hub.clone())
        .nest("/auth", auth_router(auth_app_state))
        .nest("/user", user_router(user_app_state))
        .nest("/consultant", consultant_router(consultant_app_state))
        .nest("/service", service_router(service_app_state))
        .nest("/payment", payment_router(payment_app_state));
    
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<WsHub>,
    Query(query): Query<WsQuery>
) -> ApiResult<impl IntoResponse> {
    let claims = decode_token(&query.token)
        .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, "Problem to authenticate"))?;
    
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, Box::new(hub), claims.sub)))
}

#[derive(Deserialize)]
pub struct WsQuery {
    token: String,
}

