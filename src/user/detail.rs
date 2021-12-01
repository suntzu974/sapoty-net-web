use super::super::{Anchor, AppRoute};
use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    posts: Option<Vec<PostResponse>>,
    user: Option<UserResponse>,
    fetch_posts_task: Option<FetchTask>,
    fetch_user_task: Option<FetchTask>,
    delete_post_task: Option<FetchTask>,
}

impl Detail {
    fn render_detail(
        &self,
        user: &Option<UserResponse>,
        posts: &Option<Vec<PostResponse>>,
    ) -> Html {
        match user {
            Some(o) => {
                html! {
                    <div class=classes!("detail")>
                        <h1>{&o.name}{" ("}<span class=classes!("id")>{o.id}</span>{")"}</h1>
                        {
                            self.view_post_list(posts)
                        }

                    <br />
                    <Anchor route=AppRoute::CreatePost(o.id as i32)>
                        { "Create New Post" }
                    </Anchor>
                    </div>
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                }
            }
        }
    }

    fn view_post_list(&self, posts: &Option<Vec<PostResponse>>) -> Html {
        match posts {
            Some(p) => {
                html! {
                    p.iter().map(|post| self.view_post(post)).collect::<Html>()
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                }
            }
        }
    }

    fn view_post(&self, post: &PostResponse) -> Html {
        let id = post.id;
        let user_id = self.props.user_id;
        html! {
            <div class=classes!("list-item", "pet")>
                <div><b>{ &post.title }</b> { " (" } <button onclick=self.link.callback(move |_| Msg::MakeDeletePostReq(user_id, id))>{"Delete"}</button> {")|"}{ " |(" } <button onclick=self.link.callback(move |_| Msg::MakeDeletePostReq(user_id, id))>{"Update"}</button> {")"}</div>
            </div>
        }
    }
}

pub enum Msg {
    MakePostReq(i32),
    MakeUserReq(i32),
    MakeDeletePostReq(i32, i32),
    RespPosts(Result<Vec<PostResponse>, anyhow::Error>),
    RespUser(Result<UserResponse, anyhow::Error>),
    RespDeletePost(Response<Json<Result<(), anyhow::Error>>>, i32),
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakePostReq(props.user_id));
        link.send_message(Msg::MakeUserReq(props.user_id));
        Self {
            props,
            link,
            user: None,
            posts: None,
            fetch_posts_task: None,
            fetch_user_task: None,
            delete_post_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_detail(&self.user, &self.posts)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakePostReq(id) => {
                let req = Request::get(&format!("https://localhost:3000/publisher/{}/posts", id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<PostResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespPosts(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_posts_task = Some(task);
                ()
            }
            Msg::MakeUserReq(id) => {
                let req = Request::get(&format!("https://localhost:3000/publishers/{}", id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<UserResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespUser(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_user_task = Some(task);
                ()
            }
            Msg::MakeDeletePostReq(user_id, post_id) => {
                let req = Request::delete(&format!(
                    "https://localhost:3000/publisher/{}/post/{}",
                    user_id, post_id
                ))
                .body(Nothing)
                .expect("can make req to backend");

                let cb = self.link.callback(
                    move |response: Response<Json<Result<(), anyhow::Error>>>| {
                        Msg::RespDeletePost(response, post_id)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.delete_post_task = Some(task);
                ()
            }
            Msg::RespPosts(resp) => {
                if let Ok(data) = resp {
                    self.posts = Some(data);
                }
            }
            Msg::RespUser(resp) => {
                if let Ok(data) = resp {
                    self.user = Some(data);
                }
            }
            Msg::RespDeletePost(resp, id) => {
                if resp.status().is_success() {
                    self.posts = self
                        .posts
                        .as_ref()
                        .map(|posts| posts.into_iter().filter(|p| p.id != id).cloned().collect());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
