#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, router::Router, Switch};

mod user;
mod post;

pub type Anchor = RouterAnchor<AppRoute>;

struct FullStackApp {}

pub enum Msg {}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/app/create-user"]
    CreateUser,
    #[to = "/app/create-post/{id}"]
    CreatePost(i32),
    #[to = "/app/update-user/{id}"]
    UpdateUser(i32),
    #[to = "/app/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}

impl Component for FullStackApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("app")>
                <div class=classes!("nav")>
                    <nav class="navbar navbar-expand-md navbar-dark bg-dark fixed-top">
                        <a class="navbar-brand" href="#">{"Sapoty"}</a>
                        <button class="navbar-toggler">
                            <span class="navbar-toggler-icon"></span>
                        </button>
                        <div class="collapse navbar-collapse" id="navbarsExampleDefault">
                            <ul class="navbar-nav mr-auto">
                                <li class="nav-item active">
                                    <a class="nav-link">
                                        <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                                        <span class="sr-only">{"(current)"}</span>
                                    </a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link" href="https://example.com">{"Link"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link"> <Anchor route=AppRoute::Home>{"Home"}</Anchor> </a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link">
                                         <Anchor route=AppRoute::CreateUser>{"Create User"}</Anchor> 
                                    </a>
                                </li>
                            </ul>
                            <form class="form-inline my-2 my-lg-0">
                                <input class="form-control mr-sm-2" type="text" placeholder="Search" />
                                <button class="btn btn-outline-success my-2 my-sm-0" type="submit">{"Search"}</button>
                            </form>
                        </div>
                    </nav>
                </div>
                <div class=classes!("content")>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::CreateUser => {
                                    html! {
                                        <div>
                                            <user::create::CreateForm />
                                        </div>}
                                }
                                AppRoute::UpdateUser(user_id) => {
                                    html! {
                                        <div>
                                            <user::update::UpdateForm user_id=user_id/>
                                        </div>}
                                }
                                AppRoute::CreatePost(user_id) => {
                                    html! {
                                        <div>
                                            <post::create::CreateForm user_id=user_id/>
                                        </div>}
                                }
                                AppRoute::Detail(user_id) => {
                                    html! {
                                        <div>
                                            <user::detail::Detail user_id=user_id/>
                                        </div>}
                                }
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <user::list::List />
                                            <br />
                                            <Anchor route=AppRoute::CreateUser>
                                            { "Create New User" }
                                                </Anchor>
                                        </div>
                                    }
                                }
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<FullStackApp>::new().mount_to_body();
}
