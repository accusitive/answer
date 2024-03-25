use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::{
    element::{Element, ElementId, SomeAction},
    instance::Instance,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Paragraph {
    pub id: ElementId,
    pub state: ParagraphState,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ParagraphState {
    pub text: String,
}
impl Element for Paragraph {
    type State = ParagraphState;
    type Action = ();

    fn state(&self) -> &Self::State {
        &self.state
    }
    fn update(&mut self, _action: Self::Action) {
        unreachable!()
    }

    fn render(&self, _instance: &Instance) -> String {
        format!("<p style=\"margin: 0; padding: 0\">{}</p>", self.state.text)
    }

    fn get_id(&self) -> ElementId {
        self.id
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ActionButton {
    pub id: ElementId,
    pub state: ActionButtonState,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ActionButtonState {
    pub value: String,
    pub effects: ElementId,
    pub action: SomeAction, // pub action: Action
                            // TODO: pub action_id: i32
}
impl Element for ActionButton {
    type State = ActionButtonState;
    type Action = ();
    fn state(&self) -> &Self::State {
        &self.state
    }
    fn update(&mut self, _action: Self::Action) {
        unreachable!()
    }
    fn render(&self, instance: &Instance) -> String {
        let action = base64::engine::general_purpose::STANDARD
            .encode(serde_json::to_string(&self.state.action).unwrap());
        let ins = &base64::engine::general_purpose::STANDARD
            .encode(serde_json::to_string(&instance).unwrap());

        format!(
            r#"<form style="margin: 0; padding: 0;" method="get" action="/action/{}/{}/{}">
        <input
          value="{}"
          type="submit"
          style="
            background: none;
            border: none;
            font: inherit;
            cursor: pointer;
            padding: 0px;
            margin: 0px;
            color: #3498db;
          "
        >
      </form>"#,
            self.state.effects, action, ins, self.state.value, 
        )
    }
    fn get_id(&self) -> ElementId {
        self.id
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ParagraphOrBold {
    pub id: ElementId,

    pub state: ParagraphOrBoldState,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ParagraphOrBoldState {
    pub text: String,
    pub bold: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParagraphOrBoldAction {
    ToggleBold,
}
impl Element for ParagraphOrBold {
    type State = ParagraphOrBoldState;

    type Action = ParagraphOrBoldAction;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn update(&mut self, action: Self::Action) {
        match action {
            ParagraphOrBoldAction::ToggleBold => self.state.bold = !self.state.bold,
        }
    }

    fn render(&self, _instance: &Instance) -> String {
        match self.state.bold {
            true => format!("<p><b>{}</b></p>", self.state.text),
            false => format!("<p>{}</p>", self.state.text),
        }
    }
    fn get_id(&self) -> ElementId {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogBox {
    pub id: ElementId,

    pub state: LogBoxState,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LogBoxState {
    pub logs: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogBoxAction {
    AddLog(String),
    RemoveLog(usize),
    ReplaceLog(usize, String),
    ClearLogs,
}

impl Element for LogBox {
    type State = LogBoxState;

    type Action = LogBoxAction;

    fn get_id(&self) -> ElementId {
        self.id
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn update(&mut self, action: Self::Action) {
        match action {
            LogBoxAction::AddLog(log) => self.state.logs.push(log),
            LogBoxAction::RemoveLog(index) => {
                self.state.logs.remove(index);
            }
            LogBoxAction::ReplaceLog(index, log) => self.state.logs[index] = log,
            LogBoxAction::ClearLogs => self.state.logs.clear(),
        }
    }

    fn render(&self, _instance: &Instance) -> String {
        format!("<div>{}</div>", self.state.logs.join("<br>"))
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct Div {
    pub id: ElementId,
    pub children: Vec<ElementId>,
    pub kind: DivKind,
}
impl Element for Div {
    type State = ();

    type Action = ();

    fn get_id(&self) -> ElementId {
        self.id
    }

    fn state(&self) -> &Self::State {
        &()
    }

    fn update(&mut self, _: Self::Action) {}

    fn render(&self, instance: &Instance) -> String {
        format!(
            "<div style=\"display: {};\">{}</div>",
            self.kind.to_string(),
            self.children
                .iter()
                .map(|child| instance.render_element(*child))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]

pub enum DivKind {
    Normal,
    InlineBlock,
}
impl ToString for DivKind {
    fn to_string(&self) -> String {
        match self {
            DivKind::Normal => "block",
            DivKind::InlineBlock => "inline-block",
        }
        .to_string()
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct Root {
    pub id: ElementId,
    pub head_chilren: Vec<ElementId>,
    pub body_children: Vec<ElementId>,
}
impl Element for Root {
    type State = ();

    type Action = ();

    fn get_id(&self) -> ElementId {
        self.id
    }

    fn state(&self) -> &Self::State {
        &()
    }

    fn update(&mut self, _: Self::Action) {}
    fn render(&self, _instance: &Instance) -> String {
        format!(
            r#"
        <!DOCTYPE html>
        <html lang="en">
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        
        <head>
            {}
        </head>
        <body>
            {}
        </body>
        </html>
        "#,
            self.head_chilren
                .iter()
                .map(|child| _instance.render_element(*child))
                .collect::<Vec<String>>()
                .join(""),
            self.body_children
                .iter()
                .map(|child| _instance.render_element(*child))
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
