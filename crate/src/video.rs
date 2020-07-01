use super::downloader::{send_future, DownloaderExample};
use super::search_result::layout_result;
use rusty_pipe::youtube_extractor::error::ParsingError;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchItem;
use rusty_pipe::youtube_extractor::trending_extractor::YTTrendingExtractor;
use rusty_pipe::youtube_extractor::stream_extractor::YTStreamExtractor;
use yew::prelude::*;

use super::video_player::VideoPlayer;


pub struct Video{
    link:ComponentLink<Self>,
    props:Props,
    videoinfo:Option<VideoInfo>
}

struct VideoInfo{
    videoId:String,
    extractor:Option<Result<YTStreamExtractor<DownloaderExample>,ParsingError>>    
}

pub enum Msg{
    LoadedExtractor(Result<YTStreamExtractor<DownloaderExample>,ParsingError>)
}

#[derive(Properties,Clone,PartialEq,Debug)]
pub struct Props{

    #[prop_or_default]
    pub video_id:Option<String>
}

impl Video{

    fn load_video(video_id:String,link:ComponentLink<Self>)->VideoInfo{

        let idclone = video_id.clone();
        let future = async move {
            let ytex = YTStreamExtractor::<DownloaderExample>::new(&idclone,DownloaderExample).await;
            Msg::LoadedExtractor(ytex)
        };
        send_future(link, future);

        VideoInfo{
            videoId:video_id,
            extractor:None
        }
    }

}


impl Component for Video{

    type Message=Msg;
    type Properties=Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{

        let videoinfo = props.video_id.clone().map(|vidid|Self::load_video(vidid,link.clone()));

        Self{
            link,
            props,
            videoinfo
        }
    }

    fn change(&mut self,_props: Self::Properties)->ShouldRender{
        if _props!=self.props{
            log::info!("video props change : {:#?}",_props);
            self.props=_props;

            if let Some(id) = self.props.video_id.clone(){
                match &self.videoinfo{
                    Some(vidinfo)=>{
                        if vidinfo.videoId!=id{
                            self.videoinfo=Some(Self::load_video(id, self.link.clone()));
                            true
                        }else{
                            true
                        }
                    }
                    None=>{
                        self.videoinfo=Some(Self::load_video(id, self.link.clone()));
                        true
                    }
                }
            }else{
                true
            }
        }
        else{
            false
        }
    }

    fn update(&mut self,msg: Self::Message)->ShouldRender{
        match msg{
            Msg::LoadedExtractor(extractor)=>{
                match &mut self.videoinfo{
                    Some(vidinfo)=>{
                        vidinfo.extractor=Some(extractor);
                        true
                    }
                    None=>false
                }
            }
        }
    }

    fn view(&self)->Html{

        match &self.videoinfo{
            Some(video)=>{
                match &video.extractor{
                    Some(extractor)=>{
                        match extractor{
                            Ok(extractor)=>{
                                let mut human_formatter = human_format::Formatter::new();
                                human_formatter.with_decimals(2);

                                let name = extractor.get_name().unwrap_or_default();
                                let author_name = extractor.get_uploader_name().unwrap_or_default();
                                let avatar = extractor.get_uploader_avatar_url().unwrap_or_default().first().and_then(|f|Some(f.url.clone())).unwrap_or_default();
                                let likes = human_formatter.format(extractor.get_like_count().unwrap_or_default() as f64);
                                let dislikes = human_formatter.format(extractor.get_dislike_count().unwrap_or_default() as f64);
                                let is_popup = self.props.video_id.is_none();
                                let view_count = extractor.get_view_count().ok().and_then(|f|Some(human_formatter.format(f as f64))).unwrap_or("unknown".to_string());

                                let description = extractor.get_description(true).unwrap_or_default();

                                let desc_div = yew::utils::document().create_element("div").unwrap();
                                desc_div.set_inner_html(&description.0);
                                use yew::virtual_dom::VNode;
                                let desc_ref = VNode::VRef(web_sys::Node::from(desc_div));
                                
                                let related_videos = extractor.get_related().unwrap_or_default();
                                use super::search_result::layout_result;
                                let related_videos = layout_result(1080_f64, &related_videos);
                                html!{
                                    <div class="columns">
                                        <div class="column">
                                            <section class="card" >
                                                <div class="card-image">
                                                    <VideoPlayer fullpage=self.props.video_id.is_some() key={"videoplayer".to_string()} extractor=extractor.clone() />
                                                </div>
                                                {
                                                    if let Some(_id)= &self.props.video_id{
                                                        html!{
                                                            <div class="card-content">
                                                                <p class="title is-5">
                                                                    {name}
                                                                </p>
                                                                <div class="level">
                                                                    <div class="level-left">
                                                                        <div class="media level-item">
                                                                            <div class="media-left">
                                                                                <figure class="image is-48x48">
                                                                                <img src=avatar alt="Channelavatar" style="border-radius: 50%"/>
                                                                                </figure>
                                                                            </div>
                                                                            <div class="media-content level is-mobile">
                                                                                <div class="subtitle is-5">{author_name}</div>
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                    <div class="level-right">
                                                                        <div class="level-item has-text-centered columns is-1">
                                                                            <div class="column">
                                                                                <span class="icon is-large">
                                                                                    <i class="fas fa-thumbs-up"></i>
                                                                                </span>
                                                                                <p class="is-5">{likes}</p>
                                                                            </div>
                                                                            <div class="column">
                                                                                <span class="icon is-large">
                                                                                    <i class="fas fa-thumbs-down"></i>
                                                                                </span>
                                                                                <p class="is-5">{dislikes}</p>
                                                                            </div>
                                                                            <div class="column">
                                                                                <span class="icon is-large">
                                                                                    <i class="fas fa-eye"></i>
                                                                                </span>
                                                                                <p class="is-5">{view_count}</p>
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                                <div style="overflow:auto">
                                                                    {desc_ref}
                                                                </div>
                                                            </div>
                                                        }
                                                    }else{
                                                        html!{

                                                        }
                                                        
                                                    }
                                                }
                                            </section>
                                        </div>
                                        <div class="column is-3">
                                            {
                                                for related_videos   
                                            }
                                        </div>
                                    </div>
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
                        }
                    }
                    None=>{
                        html!{
                            <progress class="progress is-small is-primary" max="100"></progress>
                        }
                    }
                }
            }
            None=>{
                html!{}
            }
        }
    }

}