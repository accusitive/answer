use actix_web::{
    get,
    web::{self, Redirect},
    App, HttpResponse, HttpServer, Responder,
};
use answer_eas::instance::Instance;
use base64::Engine;

mod apps;

#[get("/action/{id}/{action}/{instance}")]
async fn handle_action(path: web::Path<(u64, String, String)>) -> impl Responder {
    {
        let (id, action, instance) = path.into_inner();
        let action_decoded = base64::engine::general_purpose::STANDARD
            .decode(action.clone())
            .unwrap();

        let instance = base64::engine::general_purpose::STANDARD
            .decode(instance)
            .unwrap();
        let mut instance = serde_json::from_slice::<Instance>(&instance).expect("Invalid JSON");

        instance.handle_action(id, action_decoded);

        Redirect::to(format!(
            "/instance/{}",
            &base64::engine::general_purpose::STANDARD
                .encode(serde_json::to_string(&instance).unwrap())
        ))
    }
}

#[get("/")]
async fn index() -> impl Responder {
    // let instance = logbox_test();
    let instance = crate::apps::counter_app::counter_app();
    Redirect::to(format!(
        "/instance/{}",
        &base64::engine::general_purpose::STANDARD
            .encode(serde_json::to_string(&instance).unwrap())
    ))
}
pub const MINIMIZE: bool = false;
#[get("/instance/{instance}")]
async fn greet(instance: web::Path<String>) -> HttpResponse {
    let instance = base64::engine::general_purpose::STANDARD
        .decode(instance.into_inner())
        .unwrap();
    let instance = serde_json::from_slice::<Instance>(&instance).expect("Invalid JSON");

    let mut body = instance.render();
    // let mut body = instance.render().replace(
    //     "$INSTANCE",
    //     ,
    // );
    let c = minify_html_onepass::Cfg {
        ..Default::default()
    };
    if MINIMIZE {
        let size = minify_html_onepass::in_place(unsafe { body.as_bytes_mut() }, &c).unwrap();
        let body = body.split_at_mut(size).0.to_string();
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body)
    } else {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(handle_action)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
