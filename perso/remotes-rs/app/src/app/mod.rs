mod account;
mod back_office;
mod post;

use account::Account;
use back_office::BackOffice;
use post::Post;

use mogwai::prelude::*;
use std::convert::TryFrom;

pub enum App {
    Account(Gizmo<Account>),
    BackOffice(Gizmo<BackOffice>),
    Post(Gizmo<Post>),
}

#[derive(Clone)]
pub enum AppModel {}

#[derive(Clone)]
pub enum AppView {}

impl TryFrom<Option<String>> for App {
    type Error = JsValue;

    fn try_from(app_name: Option<String>) -> Result<Self, Self::Error> {
        let app = if let Some(name) = app_name {
            match name.as_str() {
                "account" => Self::Account(Account::gizmo()),
                "backoffice" => Self::BackOffice(BackOffice::gizmo()),
                "post" => Self::Post(Post::gizmo()),
                _ => return Err(JsValue::from_str("Unsupported app_name")),
            }
        } else {
            return Err(JsValue::from_str("Missing app name"));
        };

        Ok(app)
    }
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

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
            <div id="app">
                    {
                        match self {
                            Self::Account(account) => account.view_builder(),
                            Self::BackOffice(back_office) => back_office.view_builder(),
                            Self::Post(post) => post.view_builder()
                        }
                    }
            </div>
        }
    }
}
