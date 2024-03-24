use serde::{Deserialize, Serialize};

use crate::element::{Element, ElementId, SomeAction, SomeElement};


#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
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