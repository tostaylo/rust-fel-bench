use crate::handle;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum Actions {
    Create,
    Clear,
}
#[derive(Debug, Default, Clone)]
pub struct Main {
    id: String,
    state: bool,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            state: false,
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = ();
    type Message = Actions;
    type State = ();

    fn add_props(&mut self, _props: Self::Properties) {}

    fn reduce_state(&mut self, message: Self::Message) {
        match message {
            Actions::Create => self.0.borrow_mut().state = true,
            Actions::Clear => self.0.borrow_mut().state = false,
        }
        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow_mut();
        let state = borrow.state;
        let mut el = None;
        if state {
            let mut main_table = rust_fel::html("<table></table>".to_owned());
            let mut table_body = rust_fel::html("<tbody></tbody>".to_owned());
            let mut table_rows = vec![];
            for num in 0..10000 {
                table_rows.push(rust_fel::html(format!(
                    "<tr><td>{}</td><td>a bunch of words man</td></tr>",
                    num
                )));
            }
            table_body.props.children = Some(table_rows);
            main_table.props.children = Some(vec![table_body]);
            el = Some(main_table);
        }

        let action = match state {
            true => Actions::Clear,
            false => Actions::Create,
        };

        let create_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                on_click: Some(Box::new(move || clone.reduce_state(action))),
                ..Default::default()
            },
        );
        let mut children = vec![create_button];
        if let Some(x) = el {
            children.push(x);
        }
        rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some("main".to_owned()),
                children: Some(children),
                ..Default::default()
            },
        )
    }
}
