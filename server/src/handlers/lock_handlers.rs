

pub async fn lock_acquire_handler(params: web::Path<String>, lock_state: web::Data<LockState>) -> impl Responder {
    let lock_key = params.to_string();
    let is_acquired = lock_state.lock.try_acquire(&lock_key);

    HttpResponse::Ok().json(is_acquired)
}

pub async fn lock_release_handler(params: web::Path<String>, lock_state: web::Data<LockState>) -> impl Responder {
    let lock_key = params.to_string();
    let is_released = lock_state.lock.try_release(&lock_key);

    HttpResponse::Ok().json(is_released)
}

pub async fn lock_status_handler(params: web::Path<String>, lock_state: web::Data<LockState>) -> impl Responder {
    let lock_key = params.to_string();
    let is_acquire = lock_state.lock.is_acquire(&lock_key);

    HttpResponse::Ok().json(is_acquire)
}