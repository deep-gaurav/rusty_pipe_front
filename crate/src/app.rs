use yew::prelude::*;
use yew_router::prelude::*;

use super::search_result::SearchResult;
use super::trending::Trending;
use super::video::Video;
use crate::downloader::{send_future, DownloaderExample};
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;

pub struct App {
    link: ComponentLink<Self>,
    suggestions: Vec<String>,
    search_inputref: NodeRef,
    search_result: Option<(String, YTSearchExtractor)>,
    show_nav_menu: bool,
    is_loading_search: bool,
}

pub enum Msg {
    Ignore,
    QuerySearch(String),
    ShowSearch(String, Vec<String>),
    Search,
    ClickSuggestion(String),
    ToggleNavMenu,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/search/{query}"]
    Search(String),
    #[to = "/video/{videoid}"]
    Video(String),
    #[to = "/"]
    Home,
}

pub fn go_to_route(route:Route){
  use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
  let mut dispatcher = RouteAgentDispatcher::<()>::new();
  dispatcher.send(RouteRequest::ChangeRoute(route));
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            suggestions: vec![],
            show_nav_menu: false,
            search_inputref: NodeRef::default(),
            is_loading_search: false,
            search_result: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search => {
                // log::info!("Search");
                if self.is_loading_search {
                    return false;
                }
                use web_sys::HtmlInputElement;
                let searchel: HtmlInputElement =
                    self.search_inputref.cast().expect("Not htmlinputelement");
                let searchquery = searchel.value();
                use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
                let mut dispatcher = RouteAgentDispatcher::<()>::new();
                dispatcher.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Search(
                    searchquery,
                ))));
                true
            }

            Msg::ClickSuggestion(suggestion) => {
                use web_sys::HtmlInputElement;
                let searchel: HtmlInputElement =
                    self.search_inputref.cast().expect("Not htmlinputelement");
                searchel.set_value(&suggestion);
                let ch2 = suggestion.clone();
                let future = async move {
                    let change = ch2.clone();
                    let ytex =
                        YTSearchExtractor::get_search_suggestion(&change, &DownloaderExample).await;
                    match ytex {
                        Ok(suggestion) => Msg::ShowSearch(change, suggestion),
                        Err(err) => {
                            log::error!("{:#?}", err);
                            Msg::Ignore
                        }
                    }
                };
                send_future(self.link.clone(), future);
                false
            }

            Msg::ToggleNavMenu => {
                self.show_nav_menu = !self.show_nav_menu;
                true
            }

            Msg::QuerySearch(change) => {
                // log::info!("Query: {}", change);
                let ch2 = change.clone();
                let future = async move {
                    let change = ch2.clone();
                    let ytex =
                        YTSearchExtractor::get_search_suggestion(&change, &DownloaderExample).await;
                    match ytex {
                        Ok(suggestion) => Msg::ShowSearch(change, suggestion),
                        Err(err) => {
                            log::error!("{:#?}", err);
                            Msg::Ignore
                        }
                    }
                };
                send_future(self.link.clone(), future);

                false
            }

            Msg::ShowSearch(query, suggestions) => {
                use web_sys::HtmlInputElement;
                let searchel: HtmlInputElement =
                    self.search_inputref.cast().expect("Not htmlinputelement");
                let searchquery = searchel.value();
                if query == searchquery {
                    self.suggestions = suggestions;
                    true
                } else {
                    false
                }
            }

            Msg::Ignore => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let suggestionlist = html! {
          for self.suggestions.iter().map(|s|{
            let s2 = s.clone();
            html!{
              <a href="#" key=s.to_string() class="dropdown-item"
                onclick=self.link.callback(move |_| {

                  Msg::ClickSuggestion(s2.to_string())
                })
              >
                {s}
              </a>
            }}
          )
        };

        html! {
          <>
            <div class="navbar is-active" style="position:sticky;top:0;">
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
                  <div class="field has-addons">
                  <div class="control">

                    <div class="dropdown is-hoverable">
                      <div class="dropdown-trigger">
                            <form onsubmit=self.link.callback(|_|Msg::Search) >
                            <input ref=self.search_inputref.clone() class="input" oninput=self.link.callback(
                                |ip:yew::InputData|Msg::QuerySearch(ip.value)
                            ) onsubmit=self.link.callback(|_|Msg::Search) />
                            </form>
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
                    <div class="control">
                    <a class={
                        let mut classes = "button is-info".to_string();
                        if self.is_loading_search {
                          classes = format!("{} is-loading",classes);
                        }
                        classes
                      } onclick=self.link.callback(|_|Msg::Search)>
                        <span class="icon">
                            <i class="fas fa-search" />
                        </span>
                    </a>
                  </div>
                    </div>


                  </div>
                </div>
              </div>
            </div>
            <section class="section">
              <div class="container">

                 <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                      html!{
                        <>
                        {
                          if let AppRoute::Video(videoId)=&switch{
                            html!{
                              <Video video_id=videoId.clone()/>
                            }
                          }else{
                            html!{
                              <Video />
                            }
                          }
                        }
                        {
                          match switch{
                            AppRoute::Home => html!{
                              <Trending />
                            },
                            AppRoute::Search(query) =>{
                              log::info!("query : {}",query);
                              html!{
                                <div key=query.clone()>
                                  <SearchResult key=query.clone() query=query.clone()/>
                                </div>
                            } }
                            AppRoute::Video(_)=>html!{}
                          }
                        }
                        </>
                      }
                    })
                 />
              </div>
            </section>
          </>
        }
    }
}
