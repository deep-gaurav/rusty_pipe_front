use yew::format::Json;
use yew::prelude::*;
use yew::services::FetchService;

use super::graphql;
use graphql_client::GraphQLQuery;

pub static RUSTY_PIPE_SERVER: &str = "https://rustypipe.herokuapp.com/graphql";

pub struct App {
    fetch_service: FetchService,
    link: ComponentLink<Self>,
    suggestion_fetch_task: Option<yew::services::fetch::FetchTask>,
    suggestions: Vec<String>,
    show_nav_menu: bool
}

pub enum Msg {
    Ignore,
    QuerySearch(String),
    ShowSearch(graphql::suggestion_query::suggestions::SuggestionsSearch),
    ToggleNavMenu
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            fetch_service: FetchService::default(),
            link,
            suggestion_fetch_task: None,
            suggestions: vec![],
            show_nav_menu: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

            Msg::ToggleNavMenu => {
              self.show_nav_menu = !self.show_nav_menu;
              true
            }

            Msg::QuerySearch(change) => {
                log::info!("Query: {}", change);
                use graphql::suggestion_query::{
                    suggestions::ResponseData, suggestions::Variables, Suggestions,
                };
                use graphql_client::Response;

                let query = Suggestions::build_query(Variables { query: change });

                let querys = serde_json::to_string(&query).expect("Query not json");
                log::info!("{:#?}", querys);
                // log::info!("{:#?}");
                let req = http::Request::post(RUSTY_PIPE_SERVER)
                    .header("Content-Type", "application/json")
                    .body(Ok(querys))
                    .expect("Cant make request");
                let task = self
                    .fetch_service
                    .fetch(
                        req,
                        self.link
                            .callback(|res: http::Response<Result<String, anyhow::Error>>| {
                                let body = res.into_body();
                                if let Ok(body) = body {
                                    let bodyres: Response<ResponseData> =
                                        serde_json::from_str(&body).expect("Not Resp");
                                    // log::info!("{:#?}",bodyres);
                                    if let Some(data) = bodyres.data {
                                        Msg::ShowSearch(data.search)
                                    } else {
                                        Msg::Ignore
                                    }
                                } else {
                                    Msg::Ignore
                                }
                            }),
                    )
                    .expect("Cant create fetch task");
                self.suggestion_fetch_task = Some(task);

                false
            }


            Msg::ShowSearch(suggestions) => {
                self.suggestions = suggestions.suggestion;
                true
            }

            Msg::Ignore => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let suggestionlist = html! {
          for self.suggestions.iter().map(|s|
            html!{
              <a href="#" key=s.to_string() class="dropdown-item">
                {s}
              </a>
            }
          )
        };

        html! {
          <>
            <div class="navbar is-active">
              <div class="navbar-brand">
                <div class="navbar-item">
                  <h2 class="title">{"RustyPipe"}</h2>
                </div>

                <a role="button" onclick=self.link.callback(|_|Msg::ToggleNavMenu) class="navbar-burger" aria-label="menu" aria-expanded="false">
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>
              <div class={
                let mut classes = "navbar-menu".to_string();
                if self.show_nav_menu{
                  classes = format!("{} is-active",classes);
                }
                classes
              }>
                <div class="navbar-end">
                  <div class="navbar-item">
                    <div class="dropdown is-hoverable">
                      <div class="dropdown-trigger">
                        <div class="field">
                          <div class="control has-icons-left is-expanded">
                            <input class="input" oninput=self.link.callback(
                                |ip:yew::InputData|Msg::QuerySearch(ip.value)
                            ) />
                            <span class="icon is-left"><i class="fas fa-search"></i></span>
                          </div>
                        </div>
                      </div>
                      <div class="dropdown-menu" id="dropdown-menu" role="menu">
                        <div class="dropdown-content">
                          {
                            suggestionlist
                          }
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <section class="section">
              <div class="container">
                <h1 class="title">
                  {
                    format!("{:#?}",self.suggestions)
                  }
                </h1>
              </div>
            </section>
          </>
        }
    }
}
