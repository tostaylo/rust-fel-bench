use crate::handle;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Main {
    id: String,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = ();
    type Message = ();
    type State = ();

    fn add_props(&mut self, _props: Self::Properties) {
        ()
    }

    fn reduce_state(&mut self, _message: Self::Message) {}

    fn render(&self) -> rust_fel::Element {
        let mut main_table = rust_fel::html("<table></table>".to_owned());
        let mut table_body = rust_fel::html("<tbody></tbody>".to_owned());
        let mut table_rows = vec![];
        for num in 0..1000 {
            table_rows.push(rust_fel::html(format!(
                "<tr><td>{}</td><td>a bunch of words man</td></tr>",
                num
            )));
        }
        table_body.props.children = Some(table_rows);
        main_table.props.children = Some(vec![table_body]);
        rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(self.0.borrow().id.clone()),
                class_name: Some("main".to_owned()),
                children: Some(vec![main_table]),
                ..Default::default()
            },
        )
    }
}
