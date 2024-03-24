use crate::{
    apps::logbox::logbox_test,
    element::SomeAction,
    elements::{ActionButton, ActionButtonState},
    instance::Instance,
};
use actix_web::{
    get,
    web::{self, Redirect},
    App, HttpResponse, HttpServer, Responder,
};
use base64::Engine;
use elements::{Div, LogBox, LogBoxAction, LogBoxState, Paragraph, ParagraphState, Root};

mod apps;
mod element;
mod elements;
mod instance;

#[get("/action/{id}/{action}/{instance}")]
async fn handle_action(path: web::Path<(u64, String, String)>) -> impl Responder {
    {
        let (id, action, instance) = path.into_inner();
        let action_decoded = base64::engine::general_purpose::STANDARD
            .decode(action.clone())
            .unwrap();
        let some_action =
            serde_json::from_slice::<SomeAction>(&action_decoded).expect("Invalid JSON");

        let instance = base64::engine::general_purpose::STANDARD
            .decode(instance)
            .unwrap();
        let mut instance = serde_json::from_slice::<Instance>(&instance).expect("Invalid JSON");

        instance.handle_action(id, some_action);
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

    let mut body = instance.render().replace(
        "$INSTANCE",
        &base64::engine::general_purpose::STANDARD
            .encode(serde_json::to_string(&instance).unwrap()),
    );
    let c = minify_html_onepass::Cfg {
        ..Default::default()
    };
    if MINIMIZE {
        let size = minify_html_onepass::in_place(unsafe { body.as_bytes_mut() }, &c).unwrap();
        let body = body.split_at_mut(size).0.to_string();
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::Ok().body(body)
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

// fn bold_test() -> Instance {
//     let mut instance = Instance::new();
//     let p = Paragraph {
//         id: instance.next_id(),
//         state: ParagraphState {
//             text: format!("Pressing either of these buttons will toggle the boldness of their respective paragraps.")
//         }
//     };

//     let pob = ParagraphOrBold {
//         id: instance.next_id(),
//         state: ParagraphOrBoldState {
//             text: "First Bold or paragraph".to_string(),
//             bold: true,
//         },
//     };
//     let pob2 = ParagraphOrBold {
//         id: instance.next_id(),
//         state: ParagraphOrBoldState {
//             text: "Second Bold or paragraph".to_string(),
//             bold: true,
//         },
//     };
//     let ab = ActionButton {
//         id: instance.next_id(),
//         state: ActionButtonState {
//             value: "First".to_string(),
//             effects: pob.id,
//             action: SomeAction::ParagraphOrBold(ParagraphOrBoldAction::ToggleBold),
//         },
//     };
//     let ab2 = ActionButton {
//         id: instance.next_id(),
//         state: ActionButtonState {
//             value: "Second".to_string(),
//             effects: pob2.id,
//             action: SomeAction::ParagraphOrBold(ParagraphOrBoldAction::ToggleBold),
//         },
//     };

//     instance.elements.insert(p.id, SomeElement::Paragraph(p));

//     instance
//         .elements
//         .insert(pob.id, SomeElement::ParagraphOrBold(pob));
//     instance
//         .elements
//         .insert(ab.id, SomeElement::ActionButton(ab));

//     instance
//         .elements
//         .insert(pob2.id, SomeElement::ParagraphOrBold(pob2));
//     instance
//         .elements
//         .insert(ab2.id, SomeElement::ActionButton(ab2));

//     instance
// }
