use serde::{Deserialize, Serialize};

use crate::{
    element::{Action, Element, ElementId},
    elements::*,
    instance::Instance,
};

pub fn counter_app() -> Instance {
    let mut i = Instance::new();

    let counter_instances = vec![
        Counter::new(&mut i),
        Counter::new(&mut i),
        Counter::new(&mut i),
        Counter::new(&mut i),
    ];

    let counters = Counters {
        id: i.next_id(),
        state: CountersState {
            counters: counter_instances
                .iter()
                .map(|ci| ci.get_id())
                .collect::<Vec<_>>(),
        },
    };

    for counter in counter_instances {
        i.register_element(counter);
    }
    let c = i.register_element(counters);
    let root = Root {
        id: i.next_id(),
        head_chilren: vec![],
        body_children: vec![c],
    };
    i.root = i.register_element(root);

    i
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub id: ElementId,
    pub state: CounterState,

    pub add: ElementId,
    pub remove: ElementId,
    pub reset: ElementId,
}
impl Counter {
    fn new(instance: &mut Instance) -> Self {
        let this = instance.next_id();

        let add = ActionButton {
            id: instance.next_id(),
            state: ActionButtonState {
                value: "+".to_string(),
                effects: this,
                action: Box::new(CounterAction::Increment),
            },
        };
        let remove = ActionButton {
            id: instance.next_id(),
            state: ActionButtonState {
                value: "-".to_string(),
                effects: this,
                action: Box::new(CounterAction::Decrement),
            },
        };
        let reset = ActionButton {
            id: instance.next_id(),
            state: ActionButtonState {
                value: "0".to_string(),
                effects: this,
                action: Box::new(CounterAction::Zero),
            },
        };

        Self {
            id: this,
            state: CounterState { count: 0 },
            add: instance.register_element(add),
            remove: instance.register_element(remove),
            reset: instance.register_element(reset),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CounterState {
    pub count: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CounterAction {
    Increment,
    Decrement,
    Zero,
}
#[typetag::serde]
impl Action for CounterAction {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counters {
    pub id: ElementId,
    pub state: CountersState,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CountersState {
    pub counters: Vec<ElementId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CountersAction {
    Reset,
    RemoveCounter(usize),
}
#[typetag::serde]
impl Action for CountersAction {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[typetag::serde]

impl Element for Counter {
    fn get_id(&self) -> ElementId {
        self.id
    }

    fn update(&mut self, action: Vec<u8>) -> Option<()> {
        let mut action = self.parse_action(action)?;
        let action = action.as_any().downcast_ref::<CounterAction>()?;
        match action {
            CounterAction::Increment => self.state.count += 1,
            CounterAction::Decrement if self.state.count > 0 => self.state.count -= 1,

            CounterAction::Decrement | CounterAction::Zero => self.state.count = 0,
        }
        Some(())
    }

    fn render(&self, instance: &Instance) -> String {
        format!(
            "<div style=\"display:flex; justify-content: space-around;\"id=\"counter\"><p>{}</p> {} {} {}</div>",
            self.state.count,
            instance.render_element(self.add),
            instance.render_element(self.remove),
            instance.render_element(self.reset)
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[typetag::serde]

impl Element for Counters {
    fn get_id(&self) -> ElementId {
        self.id
    }

    fn update(&mut self, action: Vec<u8>) -> Option<()> {
        let mut action = self.parse_action(action)?;
        let action = action.as_any().downcast_mut::<CountersAction>()?;
        match action {
            CountersAction::Reset => todo!(),
            CountersAction::RemoveCounter(index) => {
                self.state.counters.remove(*index);
            }
        }
        Some(())
    }

    fn render(&self, instance: &Instance) -> String {
        let unique_items = self
            .state
            .counters
            .iter()
            .map(|counter| instance.get_element(counter).unwrap())
            .filter(|counter| {
                counter
                    .as_any()
                    .downcast_ref::<Counter>()
                    .expect("Child of counters is not a counter")
                    .state
                    .count
                    > 0
            })
            .collect::<Vec<_>>()
            .len();

        format!(
            "<p>{} items</p><div style=\"background-color: #cccccc\" id=\"counters\">{}</div>",
            unique_items,
            self.state
                .counters
                .iter()
                .map(|c| instance.render_element(*c))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
