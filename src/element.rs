use crate::apps::counter_app::{CounterAction, Counters};
use crate::{apps::counter_app::Counter, elements::*};
use crate::instance::Instance;
use serde::{Deserialize, Serialize};

pub type ElementId = u64;

pub trait Element {
    // Server side state
    type State;
    // User action
    type Action;

    fn get_id(&self) -> ElementId;
    fn state(&self) -> &Self::State;
    fn update(&mut self, action: Self::Action);
    fn render(&self, _instance: &Instance) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SomeElement {
    ActionButton(ActionButton),
    ParagraphOrBold(ParagraphOrBold),
    Paragraph(Paragraph),
    LogBox(LogBox),
    Div(Div),
    Root(Root),

    Counter(Counter),
    Counters(Counters)
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
            SomeElement::Counter(counter) => counter.id,
            SomeElement::Counters(counters) => counters.id,
            
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
            SomeElement::Counter(counter) => counter.render(&instance),
            SomeElement::Counters(counters) => counters.render(&instance),
            
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SomeAction {
    ParagraphOrBold(ParagraphOrBoldAction),
    LogBox(LogBoxAction),
    Counter(CounterAction)
}
