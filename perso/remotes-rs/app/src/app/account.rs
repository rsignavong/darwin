use derive_new::new;
use mogwai::prelude::*;

#[derive(new)]
pub struct Account {}

#[derive(Clone)]
pub enum AccountModel {}

#[derive(Clone)]
pub enum AccountView {}

impl Component for Account {
    type DomNode = HtmlElement;
    type ModelMsg = AccountModel;
    type ViewMsg = AccountView;

    fn update(
        &mut self,
        _msg: &Self::ModelMsg,
        tx: &Transmitter<Self::ViewMsg>,
        _sub: &Subscriber<Self::ModelMsg>,
    ) {
        ()
    }

    fn view(
        &self,
        tx: &Transmitter<Self::ModelMsg>,
        rx: &Receiver<Self::ViewMsg>,
    ) -> ViewBuilder<HtmlElement> {
        builder! {
            <div id="account">"Account"</div>
        }
    }
}

impl Account {
    pub fn gizmo() -> Gizmo<Self> {
        Gizmo::from(Account {})
    }
}
