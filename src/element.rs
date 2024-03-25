use std::any::Any;

use crate::apps::counter_app::{CounterAction, Counters};
use crate::{apps::counter_app::Counter, elements::*};
use crate::instance::Instance;

use serde::{Deserialize, Serialize};

pub type ElementId = u64;

#[typetag::serde(tag = "type")]
pub trait Element {
    fn get_id(&self) -> ElementId;
    fn parse_action<T: for<'a> Deserialize<'a> + Serialize>(action: Vec<u8>) -> Option<T> where Self: Sized {
        Some(serde_json::from_slice::<T>(&action).expect("Invalid JSON"))
    } 
    fn update(&mut self, action: Vec<u8>) -> Option<()>;
    fn render(&self, _instance: &Instance) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait Action{}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SomeAction {
    ParagraphOrBold(ParagraphOrBoldAction),
    LogBox(LogBoxAction),
    Counter(CounterAction)
}
