use derive_new::new;
use mogwai::prelude::*;

#[derive(new)]
pub struct BackOffice {}

#[derive(Clone)]
pub enum BackOfficeModel {}

#[derive(Clone)]
pub enum BackOfficeView {}

impl Component for BackOffice {
    type DomNode = HtmlElement;
    type ModelMsg = BackOfficeModel;
    type ViewMsg = BackOfficeView;

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
            <div id="backoffice">"BackOffice"</div>
        }
    }
}

impl BackOffice {
    pub fn gizmo() -> Gizmo<Self> {
        Gizmo::from(BackOffice {})
    }
}
