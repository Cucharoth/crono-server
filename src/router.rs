use axum::{ routing::{delete, get, post}, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::{handler::{auth_handler::*, cronograma_handler::*, group_handler::*, timer_handler::*, user_handler::add_group_to_user}, AppState};

pub fn init_router(state: AppState) -> Router {
    /* rutas groups */
    let groups_routes = Router::new()
        .route("/", get(get_groups))
        .route("/user", get(get_groups_by_user))
        .route("/new", post(create_group))
        .route("/delete", delete(delete_group));

    /* rutas timers */
    let timers_routes = Router::new()
        .route("/group/:id", get(get_timers_by_group_id))
        .route("/cronograma/:id", get(get_timers_by_cronograma_id));

    /* rutas cronograma */
    let cronograma_routes = Router::new()
        .route("/user", get(get_cronograma_by_user));

    /* rutas user */
    let user_routes = Router::new()
        .route("/social-login", post(social_login))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/add-group", post(add_group_to_user));

    // con nest aquí, cada ruta esta separada, ej: api/user/login
    let api_routes = Router::new()
        .nest("/groups", groups_routes)
        .nest("/cronograma", cronograma_routes)
        .nest("/timers", timers_routes)
        .nest("/user", user_routes);

    /* cors setup */
    let cors = CorsLayer::permissive();

    // Main router creator, rutas comienzan con /api
    Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}