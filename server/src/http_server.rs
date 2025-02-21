use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server};
use std::net::TcpListener;
use libcm::ChatManager;
use std::sync::Mutex;

async fn handler(
    _req: HttpRequest,
    chat_manager: web::Data<Mutex<ChatManager>>,
    cm_request_json: web::Json<libcm::ModelRequestType<String>>,
) -> impl Responder {
    let mutex_cm = chat_manager.into_inner();
    let mut cm = mutex_cm.lock().unwrap();

    let cm_request = cm_request_json.into_inner();

    // TODO: Fix unwrap
    let response = cm.receive(cm_request).await.unwrap();

    println!("DEBUG: {response:?}");
    HttpResponse::Ok().json(response)
}

fn get_root_resource(cm: ChatManager) -> impl actix_web::dev::HttpServiceFactory {
    let service_factory = web::resource("/")
        .guard(guard::Post())
        .to(handler)
        .app_data(actix_web::web::Data::new(Mutex::new(cm)));

    service_factory
}

pub fn serve(
    listener: TcpListener,
    chat_manager: ChatManager,
) -> Result<Server, Box<dyn std::error::Error>> {

    let actix_future = {
        HttpServer::new(move || {
        App::new()
            .wrap(
                Logger::new("remote-ip=%a startime=%t req=\"%r\" status=%s sent=%b referer=\"%{Referer}i\" reqtime=%T X-Forwarded-For=\"%{X-Forwarded-For}i\" X-Forwarded-Proto=\"%{X-Forwarded-Proto}i\" \"%{User-Agent}i\" ")
                    .exclude("/metrics")
                    .exclude("/healthz")
                    .exclude("/readyz"),
            )
                .service(
                    
                    get_root_resource(
                        chat_manager.clone(),
                )
            )

            // .route("/healthz", web::get().to(healthz))
            // .route("/readyz", web::get().to(readyz))
    })
    .listen(listener)?
    .run()
    };
    Ok(actix_future)
}
