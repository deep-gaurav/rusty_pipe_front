use super::downloader::{send_future, DownloaderExample};
use super::search_result::layout_result;
use rusty_pipe::youtube_extractor::error::ParsingError;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchItem;
use rusty_pipe::youtube_extractor::trending_extractor::YTTrendingExtractor;
use yew::prelude::*;

pub struct Trending {
    link: ComponentLink<Self>,
    is_loading: bool,
    trending_extractor: Result<YTTrendingExtractor, ParsingError>,
    page_num: u32,
}

pub enum Msg {
    Loaded(Result<YTTrendingExtractor, ParsingError>),
    LoadMore,
}

impl Component for Trending {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let future = async move {
            let ytex = YTTrendingExtractor::new(DownloaderExample).await;
            Msg::Loaded(ytex)
        };
        send_future(link.clone(), future);
        Self {
            link,
            is_loading: true,
            trending_extractor: Err(ParsingError::from("still loading")),
            page_num: 1,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(ex) => {
                self.trending_extractor = ex;
                self.is_loading = false;
                true
            }
            Msg::LoadMore => {
                self.page_num += 1;
                true
            }
        }
    }

    fn view(&self) -> Html {
        if self.is_loading {
            html! {
                <progress class="progress is-small is-primary" max="100"></progress>
            }
        } else {
            match self.trending_extractor.as_ref() {
                Ok(extractor) => match extractor.get_videos() {
                    Ok(extractor) => {
                        let mut results = vec![];
                        for vid in extractor.iter().take((self.page_num * 20) as usize) {
                            results.push(YTSearchItem::StreamInfoItem(vid.clone()));
                        }
                        let items = layout_result(320_f64, &results);
                        html! {
                            <>
                            {for items}
                            {
                                if extractor.len()>(self.page_num*20) as usize{
                                    html!{
                                        <button onclick=self.link.callback(|_|Msg::LoadMore) class={
                                            let mut classes = "button is-fullwidth".to_string();
                                            if self.is_loading {
                                              classes = format!("{} is-loading",classes);
                                            }
                                            classes
                                        }>{"Load More"}</button>
                                    }
                                }else{
                                    log::info!("extractor len : {} pagel: {}",extractor.len(),self.page_num*20);
                                    html!{

                                    }
                                }
                            }
                            </>
                        }
                    }
                    Err(err) => html! {

                        <article class="message is-danger">
                        <div class="message-header">
                          <p>{"Error"}</p>
                        </div>
                        <div class="message-body">
                          {format!("{:#?}",err)}
                        </div>
                      </article>
                    },
                },
                Err(err) => html! {

                    <article class="message is-danger">
                    <div class="message-header">
                      <p>{"Error"}</p>
                    </div>
                    <div class="message-body">
                      {format!("{:#?}",err)}
                    </div>
                  </article>
                },
            }
        }
    }
}
