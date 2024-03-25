use std::any::Any;
use std::fmt::Debug;

use crate::instance::{Instance, SomeAction};

pub type ElementId = u64;

#[typetag::serde(tag = "type")]
pub trait Element {
    fn get_id(&self) -> ElementId;
    fn parse_action(&self, action: Vec<u8>) -> Option<SomeAction> {
        // dbg!("Parsing action", String::from_utf8(action.clone()).unwrap());
        let some_action = serde_json::from_slice::<SomeAction>(&action).expect("Invalid JSON");
        Some(some_action)
    }
    fn update(&mut self, action: Vec<u8>) -> Option<()>;
    fn render(&self, _instance: &Instance) -> String;
    fn as_any(&self) -> &dyn Any;
}
impl Debug for Box<dyn Element> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sealed Element")
    }
}
#[typetag::serde(tag = "type")]
pub trait Action {
    fn as_any(&mut self) -> &mut dyn Any;
}
impl Debug for Box<dyn Action> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sealed Action")
    }
}
