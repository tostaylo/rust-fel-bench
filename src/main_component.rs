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

    fn add_props(&mut self, props: Self::Properties) {
        ()
    }

    fn reduce_state(&mut self, message: Self::Message) {}

    fn render(&self) -> rust_fel::Element {
        let main_text = rust_fel::html(format!("<span | data-cy=main-text| >Main</span>"));

        let main_el = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("main-el".to_owned()),
                children: Some(vec![main_text]),
                ..Default::default()
            },
        );

        rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(self.0.borrow().id.clone()),
                class_name: Some("main".to_owned()),
                children: Some(vec![main_el]),
                ..Default::default()
            },
        )
    }
}
