use serde::{Deserialize, Serialize};

use crate::element::{Action, Element, ElementId};

pub type SomeElement = Box<dyn Element>;
pub type SomeAction = Box<dyn Action>;

#[derive(Serialize, Deserialize)]
pub struct Instance {
    id_accumulator: ElementId,
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

    pub fn handle_action(&mut self, id: ElementId, action: Vec<u8>) {
        let element = self.get_element_mut(&id).unwrap();
        let action = serde_json::from_slice::<SomeAction>(&action).expect("failed to decode action");
        element
            .handle_action(action)
            .expect("Element failed to update");
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
    pub fn register_element<E: Element + 'static>(&mut self, element: E) -> ElementId {
        let element = Box::new(element);
        let id = element.get_id();
        self.elements.push((element.get_id(), element));

        id
    }
}
