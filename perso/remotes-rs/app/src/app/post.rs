use derive_new::new;
use mogwai::prelude::*;

#[derive(new)]
pub struct Post {}

#[derive(Clone)]
pub enum PostModel {}

#[derive(Clone)]
pub enum PostView {}

impl Component for Post {
    type DomNode = HtmlElement;
    type ModelMsg = PostModel;
    type ViewMsg = PostView;

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
            <div id="job">"Post"</div>
        }
    }
}

impl Post {
    pub fn gizmo() -> Gizmo<Self> {
        Gizmo::from(Post {})
    }
}
