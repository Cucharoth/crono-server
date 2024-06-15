use axum::{routing::{get, post}, Router};
use crate::{handler::{auth_handler::*, cronograma_handler::get_cronograma_by_user_id, group_handler::*, timer_handler::*}, AppState};

pub fn init_router(state: AppState) -> Router {
    /* rutas groups */
    let groups_routes = Router::new()
        .route("/", get(get_groups));

    /* rutas timers */
    let timers_routes = Router::new()
        .route("/group/:id", get(get_timers_by_group_id))
        .route("/cronograma/:id", get(get_timers_by_cronograma_id));

    /* rutas cronograma */
    let cronograma_routes = Router::new()
        .route("/user/:id", get(get_cronograma_by_user_id));

    /* rutas user */
    let user_routes = Router::new()
        .route("/login", post(login))
        .route("/register", post(register));

    // con nest aquí, cada ruta esta separada, ej: api/user/login
    let api_routes = Router::new()
        .nest("/groups", groups_routes)
        .nest("/cronograma", cronograma_routes)
        .nest("/timers", timers_routes)
        .nest("/user", user_routes);

    // Main router creator, rutas comienzan con /api
    Router::new()
        .nest("/api", api_routes)
        .with_state(state)
}