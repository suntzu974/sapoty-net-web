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

pub struct CreateForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_post_title: String,
}

impl CreateForm {
    fn render_form(&self, user_id: i32) -> Html {
        let edit_title = self
            .link
            .callback(move |e: InputData| Msg::EditTitle(e.value));
        html! {
            <div class=classes!("pet-form")>
                <div>
                    <input type="text" value={self.state_post_title.clone()} oninput={edit_title} />
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
     Resp(Result<PostResponse, anyhow::Error>),
    EditTitle(String),
}

impl Component for CreateForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state_post_title: String::new(),
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
                let body = PostRequest {
                    user_id: self.props.user_id,
                    title: self.state_post_title.clone(),
                };
                let req = Request::post(&format!("https://localhost:3000/publisher/{}/post",id))
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<PostResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
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
            Msg::EditTitle(input) => {
                self.state_post_title = input;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
