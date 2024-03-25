use serde::{Deserialize, Serialize};

use crate::{
    element::{Element, ElementId, SomeAction, SomeElement},
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
        i.register_element(SomeElement::Counter(counter));
    }
    let c = i.register_element(SomeElement::Counters(counters));
    let root = Root {
        id: i.next_id(),
        head_chilren: vec![],
        body_children: vec![c],
    };
    i.root = i.register_element(SomeElement::Root(root));

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
                action: SomeAction::Counter(CounterAction::Increment),
            },
        };
        let remove = ActionButton {
            id: instance.next_id(),
            state: ActionButtonState {
                value: "-".to_string(),
                effects: this,
                action: SomeAction::Counter(CounterAction::Decrement),
            },
        };
        let reset = ActionButton {
            id: instance.next_id(),
            state: ActionButtonState {
                value: "0".to_string(),
                effects: this,
                action: SomeAction::Counter(CounterAction::Zero),
            },
        };

        Self {
            id: this,
            state: CounterState { count: 0 },
            add: instance.register_element(SomeElement::ActionButton(add)),
            remove: instance.register_element(SomeElement::ActionButton(remove)),
            reset: instance.register_element(SomeElement::ActionButton(reset)),
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

impl Element for Counter {
    type State = CounterState;
    type Action = CounterAction;

    fn get_id(&self) -> ElementId {
        self.id
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn update(&mut self, action: Self::Action) {
        match action {
            CounterAction::Increment => self.state.count += 1,
            CounterAction::Decrement if self.state.count > 0 => self.state.count -= 1,

            CounterAction::Decrement | CounterAction::Zero => self.state.count = 0,
        }
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
}
impl Element for Counters {
    type State = CountersState;
    type Action = CountersAction;

    fn get_id(&self) -> ElementId {
        self.id
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn update(&mut self, action: Self::Action) {
        match action {
            CountersAction::Reset => todo!(),
            CountersAction::RemoveCounter(index) => {
                self.state.counters.remove(index);
            }
        }
    }

    fn render(&self, instance: &Instance) -> String {
        let unique_items = self
            .state
            .counters
            .iter()
            .map(|counter| instance.get_element(counter).unwrap())
            .filter(|counter| {
                if let SomeElement::Counter(counter) = counter {
                    counter.state.count > 0
                } else {
                    false
                }
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
}
