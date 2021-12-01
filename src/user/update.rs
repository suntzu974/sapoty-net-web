use common::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    prelude::*,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: i32,
}

pub struct UpdateForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_update_name: String,
}

impl UpdateForm {
    fn render_form(&self, user_id: i32) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));
        html! {
            <div class=classes!("pet-form")>
                <div>
                    <input type="text" value={self.state_update_name.clone()} oninput={edit_name} />
                </div>
                <div>
                    <button onclick=self.link.callback(move |_| Msg::MakeReq(user_id))>{"Submit"}</button>
                </div>
            </div>
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<UserResponse, anyhow::Error>),
    EditName(String),
}

impl Component for UpdateForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state_update_name: String::new(),
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form(self.props.user_id) }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq(id) => {
                let body = UserRequest {
                    name: self.state_update_name.clone(),
                };
                let req = Request::put(&format!("https://localhost:3000/publisher/{}",id))
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<UserResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can update task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("post created: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: format!("/app/{}", self.props.user_id),
                        state: (),
                    }));
                }
            }
            Msg::EditName(input) => {
                self.state_update_name = input;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
