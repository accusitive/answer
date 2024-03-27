use std::any::Any;
use std::fmt::Debug;

use crate::instance::{Instance, SomeAction};

pub type ElementId = u64;

#[typetag::serde(tag = "type")]
pub trait Element {
    /// Required to be able to downcast into any specific Element
    /// 
    /// example: [`instance.get_element(&id).as_any().downcast_ref::<AnElement>().state`]
    fn as_any(&self) -> &dyn Any;
    fn get_id(&self) -> ElementId;
    fn handle_action(&mut self, action: SomeAction) -> Option<()>;
    fn render(&self, _instance: &Instance) -> String;
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
