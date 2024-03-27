use std::fmt::Debug;

use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::{
    element::{Element, ElementId},
    instance::{Instance, SomeAction},
};

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
pub struct ActionButton {
    pub id: ElementId,
    pub state: ActionButtonState,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ActionButtonState {
    pub value: String,
    pub effects: ElementId,
    pub action: SomeAction,
}

#[typetag::serde]
impl Element for ActionButton {
    fn handle_action(&mut self, _action: SomeAction) -> Option<()> {
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[must_use]

pub struct Div {
    pub id: ElementId,
    pub children: Vec<ElementId>,
    pub kind: DivKind,
}
#[typetag::serde]
impl Element for Div {
    fn get_id(&self) -> ElementId {
        self.id
    }

    fn handle_action(&mut self, _: SomeAction) -> Option<()> {
        unreachable!()
    }

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
#[must_use]

pub struct Root {
    pub id: ElementId,
    pub head_chilren: Vec<ElementId>,
    pub body_children: Vec<ElementId>,
}
#[typetag::serde]
impl Element for Root {
    fn get_id(&self) -> ElementId {
        self.id
    }

    fn handle_action(&mut self, _: SomeAction) -> Option<()> {
        unreachable!()
    }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
