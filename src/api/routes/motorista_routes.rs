use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(super::controllers::motorista_controller::get_motoristas);
}
