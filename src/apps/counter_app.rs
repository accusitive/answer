use crate::{element::SomeElement, elements::*, instance::Instance};

pub fn counter_app() -> Instance {
    let mut i = Instance::new();
    let root = Root {
        id: i.next_id(),
        head_chilren: vec![],
        body_children: vec![],
    };

    i.register_element(SomeElement::Root(root));
    i
}
