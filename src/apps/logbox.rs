use crate::element::{SomeAction, SomeElement};
use crate::elements::*;
use crate::instance::Instance;

pub fn logbox_test() -> Instance {
    let mut instance = Instance::new();

    let logbox = LogBox {
        id: instance.next_id(),
        state: LogBoxState { logs: vec![] },
    };
    let add = ActionButton {
        id: instance.next_id(),
        state: ActionButtonState {
            value: "Add".to_string(),
            effects: logbox.id,
            action: SomeAction::LogBox(LogBoxAction::AddLog("Example Add".to_string())),
        },
    };
    let remove = ActionButton {
        id: instance.next_id(),
        state: ActionButtonState {
            value: "Remove".to_string(),
            effects: logbox.id,
            action: SomeAction::LogBox(LogBoxAction::RemoveLog(0)),
        },
    };

    let p = Paragraph {
        id: instance.next_id(),
        state: ParagraphState {
            text: "p1".to_string(),
        },
    };
    let p2 = Paragraph {
        id: instance.next_id(),
        state: ParagraphState {
            text: "p2".to_string(),
        },
    };
    let div = Div {
        id: instance.next_id(),
        kind: DivKind::InlineBlock,
        children: vec![p.id, p2.id, add.id, remove.id, logbox.id],
    };
    let root = Root {
        id: instance.next_id(),
        head_chilren: vec![],
        body_children: vec![div.id],
    };
    instance.root = root.id;
    instance.register_element(SomeElement::Root(root));
    instance.register_element(SomeElement::ActionButton(add));

    instance.register_element(SomeElement::ActionButton(remove));
    instance.register_element(SomeElement::LogBox(logbox));

    instance.register_element(SomeElement::Paragraph(p));
    instance.register_element(SomeElement::Paragraph(p2));

    instance.register_element(SomeElement::Div(div));

    instance
}
