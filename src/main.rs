mod element;

use actix_web::{
    get,
    web::{self, Redirect},
    App, HttpResponse, HttpServer, Responder,
};
use base64::Engine;
use element::{
    Div, ElementId, LogBox, LogBoxAction, LogBoxState, Paragraph, ParagraphOrBoldAction,
    ParagraphState, Root,
};
use serde::{Deserialize, Serialize};

use crate::element::{ActionButton, ActionButtonState, Element, ParagraphOrBold};
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
    let instance = logbox_test();
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
#[derive(Debug, Serialize, Deserialize)]
struct Instance {
    pub id_accumulator: ElementId,
    elements: Vec<(ElementId, SomeElement)>,
    pub root: ElementId,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            id_accumulator: 0,
            elements: Vec::new(),
            root: 0,
        }
    }

    pub fn next_id(&mut self) -> ElementId {
        self.id_accumulator += 1;
        self.id_accumulator - 1
    }
    pub fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.render_element(self.root));
        s
    }
    pub fn render_element(&self, id: ElementId) -> String {
        let element = self.get_element(&id).unwrap();
        element.render(&self)
    }

    pub fn handle_action(&mut self, id: ElementId, action: SomeAction) {
        match (self.get_element_mut(&id).unwrap(), action) {
            (SomeElement::ParagraphOrBold(element), SomeAction::ParagraphOrBold(action)) => {
                element.update(action)
            }
            (SomeElement::LogBox(element), SomeAction::LogBox(action)) => element.update(action),
            _ => unreachable!(),
        }
        // match self.elements.get_mut(&id).unwrap() {
        //     SomeElement::ParagraphOrBold(pob) => match action {
        //         SomeAction::ParagraphOrBold(poba) => pob.update(poba),
        //         _ => unreachable!("Invalid action for this element"),
        //     },
        //     _ => unreachable!("Remaining elements do not have actions"),
        // }
    }
    pub fn get_element_mut(&mut self, id: &ElementId) -> Option<&mut SomeElement> {
        self.elements
            .iter_mut()
            .find(|element: &&mut (u64, SomeElement)| element.0 == *id)
            .map(|(_id, ele)| ele)
    }
    pub fn get_element(&self, id: &ElementId) -> Option<&SomeElement> {
        self.elements
            .iter()
            .find(|element: &&(u64, SomeElement)| element.0 == *id)
            .map(|(_id, ele)| ele)
    }
    pub fn register_element(&mut self, element: SomeElement) {
        self.elements.push((element.get_id(), element));
    }
}
#[derive(Debug, Serialize, Deserialize)]

enum SomeElement {
    ActionButton(ActionButton),
    ParagraphOrBold(ParagraphOrBold),
    Paragraph(Paragraph),
    LogBox(LogBox),
    Div(Div),
    Root(Root),
}
impl SomeElement {
    pub fn get_id(&self) -> ElementId {
        match self {
            SomeElement::ActionButton(action_button) => action_button.id,
            SomeElement::ParagraphOrBold(paragraph_or_bold) => paragraph_or_bold.id,
            SomeElement::Paragraph(paragraph) => paragraph.id,
            SomeElement::LogBox(logbox) => logbox.id,
            SomeElement::Div(div) => div.id,
            SomeElement::Root(root) => root.id,
        }
    }
    pub fn render(&self, instance: &Instance) -> String {
        match self {
            SomeElement::ActionButton(action_button) => action_button.render(&instance),
            SomeElement::ParagraphOrBold(paragraph_or_bold) => paragraph_or_bold.render(&instance),
            SomeElement::Paragraph(paragraph) => paragraph.render(&instance),
            SomeElement::LogBox(logbox) => logbox.render(&instance),
            SomeElement::Div(div) => div.render(&instance),
            SomeElement::Root(root) => root.render(&instance),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SomeAction {
    ParagraphOrBold(ParagraphOrBoldAction),
    LogBox(LogBoxAction),
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
fn logbox_test() -> Instance {
    let mut instance = Instance::new();

    let logbox = LogBox {
        id: instance.next_id(),
        state: LogBoxState { logs: vec![] },
    };
    let add = ActionButton {
        id: instance.next_id(),
        state: ActionButtonState {
            value: "Add".to_string(),
            effects: logbox.id,
            action: SomeAction::LogBox(LogBoxAction::AddLog("Example Add".to_string())),
        },
    };
    let remove = ActionButton {
        id: instance.next_id(),
        state: ActionButtonState {
            value: "Remove".to_string(),
            effects: logbox.id,
            action: SomeAction::LogBox(LogBoxAction::RemoveLog(0)),
        },
    };

    let p = Paragraph {
        id: instance.next_id(),
        state: ParagraphState {
            text: "p1".to_string(),
        },
    };
    let p2 = Paragraph {
        id: instance.next_id(),
        state: ParagraphState {
            text: "p2".to_string(),
        },
    };
    let div = Div {
        id: instance.next_id(),
        kind: element::DivKind::InlineBlock,
        children: vec![p.id, p2.id, add.id, remove.id, logbox.id],
    };
    let root = Root {
        id: instance.next_id(),
        head_chilren: vec![],
        body_children: vec![div.id],
    };
    instance.root = root.id;
    instance.register_element(SomeElement::Root(root));
    instance.register_element(SomeElement::ActionButton(add));

    instance.register_element(SomeElement::ActionButton(remove));
    instance.register_element(SomeElement::LogBox(logbox));

    instance.register_element(SomeElement::Paragraph(p));
    instance.register_element(SomeElement::Paragraph(p2));

    instance.register_element(SomeElement::Div(div));

    instance
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
