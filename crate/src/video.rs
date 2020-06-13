use super::downloader::{send_future, DownloaderExample};
use super::search_result::layout_result;
use rusty_pipe::youtube_extractor::error::ParsingError;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchItem;
use rusty_pipe::youtube_extractor::trending_extractor::YTTrendingExtractor;
use rusty_pipe::youtube_extractor::stream_extractor::YTStreamExtractor;
use yew::prelude::*;


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
            // let videoinfo = _props.video_id.clone().map(|vidid|Self::load_video(vidid));

            // self.props=_props;
        }
        false
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
                        html!{
                            html!{
                                "extractor"
                            }
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