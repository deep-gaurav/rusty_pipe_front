
interface videosource {
    url: string,
    quality: string,
    mimeType: string,
    bitrate: number,
    height: number,
    contentLength: string
}

interface audiosource {
    url: string,
    bitrate: number,
    quality: string,
    mimeType: string,
    contentLength: string
}

enum videostates {
    play,
    pause,
    loading
}

function htmlToElements(html) {
    var template = document.createElement('template');
    template.innerHTML = html;
    return template.content.childNodes;
}
function htmlToElement(html) {
    var template = document.createElement('template');
    html = html.trim(); // Never return a text node of whitespace as the result
    template.innerHTML = html;
    return template.content.firstChild;
}

class BulPlayer extends HTMLElement {
    vidcontainer: HTMLDivElement;
    vidtag: HTMLVideoElement;
    audtag: HTMLAudioElement;
    controlsbase: HTMLDivElement;
    playicon: HTMLElement;
    pauseicon: HTMLElement;
    settingicon: HTMLElement;
    fullscreenicon: HTMLElement;
    fullscreenexiticon: HTMLElement;
    buffercontainer: HTMLElement;
    progressdiv: HTMLElement;
    status: HTMLElement;
    previewvid: HTMLVideoElement;
    progressbarcover: HTMLElement;
    settingpanel: HTMLElement;
    settingclose: HTMLElement;
    qualityholder: HTMLElement;
    loadingicon: HTMLElement;
    posterurl: string;

    data:any;

    constructor() {
        super();

        console.log("created")

    }

    static get observedAttributes() { return ['data','poster','id']; }

    connectedCallback() {

        console.log("connected")
        console.log("attributes",this.getAttributeNames());
        if(!this.posterurl){
            this.posterurl=this.getAttribute("poster");
        }

        this.vidcontainer = document.createElement("div");
        this.vidcontainer.style.width = "100%";
        this.vidcontainer.style.position = "relative";
        this.vidcontainer.classList.add("has-text-light");
        this.appendChild(this.vidcontainer);

        this.vidtag = document.createElement("video");
        this.vidtag.style.display = "block";
        this.vidtag.style.width = "100%";
        this.vidtag.style.maxHeight = "70vh";
        this.vidtag.poster=this.posterurl;
        this.vidcontainer.appendChild(this.vidtag);

        this.audtag = document.createElement("audio");
        this.vidcontainer.appendChild(this.audtag);

        this.controlsbase = document.createElement("div");
        this.controlsbase.style.position = "absolute";
        this.controlsbase.style.top = "0px";
        this.controlsbase.style.width = "100%";
        this.controlsbase.style.height = "100%";
        this.controlsbase.style.backgroundColor = "rgba(0,0,0,0.418)";
        this.controlsbase.classList.add("is-hidden");
        this.vidcontainer.appendChild(this.controlsbase);

        this.playicon = htmlToElement(`
        <div class="icon centertag is-hidden" id="playicon">
            <i class=" fas fa-play fa-3x"></i>
        </div>`) as HTMLElement;
        this.controlsbase.appendChild(this.playicon);

        this.pauseicon = htmlToElement(`
        <div class="icon centertag is-hidden" id="pauseicon">
            <i class=" fas fa-pause fa-3x"></i>
        </div>`.trim()) as HTMLElement;
        this.controlsbase.appendChild(this.pauseicon);

        this.settingicon = htmlToElement(`
        <div class="icon toprighttag " id="settingicon">
            <i class=" fas fa-cog fa-2x"></i>
        </div>`.trim()) as HTMLElement;
        this.controlsbase.appendChild(this.settingicon);

        this.fullscreenicon = htmlToElement(`
        <div class="icon toplefttag" id="fullscreenicon">
            <svg style="width:48px;height:48px" viewBox="5 5 14 14">
                <path fill="currentColor" d="M5,5H10V7H7V10H5V5M14,5H19V10H17V7H14V5M17,14H19V19H14V17H17V14M10,17V19H5V14H7V17H10Z" />
            </svg>
        </div>`.trim()) as HTMLElement;
        this.controlsbase.appendChild(this.fullscreenicon);

        this.fullscreenexiticon = htmlToElement(`
        <div class="icon toplefttag is-hidden" id="exitfullscreenicon">
            <svg style="width:48px;height:48px" viewBox="5 5 14 14">
                <path fill="currentColor" d="M14,14H19V16H16V19H14V14M5,14H10V19H8V16H5V14M8,5H10V10H5V8H8V5M19,8V10H14V5H16V8H19Z" />
            </svg>
        </div>`.trim()) as HTMLElement;
        this.controlsbase.appendChild(this.fullscreenexiticon);

        let backgroundbar = document.createElement("div");
        backgroundbar.style.width = "100%";
        backgroundbar.style.position = "absolute";
        backgroundbar.style.bottom = "0px";
        backgroundbar.style.height = "0.5em";
        backgroundbar.style.backgroundColor = "rgb(133, 133, 133)";
        this.controlsbase.appendChild(backgroundbar);

        this.buffercontainer = document.createElement("div");
        this.buffercontainer.classList.add("buffer");
        this.controlsbase.appendChild(this.buffercontainer);

        this.progressdiv = htmlToElement(`<div id="progressdiv" class="has-background-primary" style="width: 0%; position: absolute; bottom: 0px; height: 0.5em;"></div>`.trim()) as HTMLElement;
        this.controlsbase.appendChild(this.progressdiv);

        this.status = document.createElement("div");
        this.status.classList.add("box");
        this.status.classList.add("is-hidden");
        this.status.style.position = "absolute";
        this.status.style.marginBottom = "1.2em";
        this.status.style.bottom = "0px";
        this.status.style.padding = "0.2em";
        this.controlsbase.appendChild(this.status);
        this.previewvid = document.createElement("video");
        this.previewvid.preload = "auto";
        this.previewvid.style.width = "8rem";
        this.status.appendChild(this.previewvid);

        this.progressbarcover = document.createElement("div");
        this.progressbarcover.style.position = "absolute";
        this.progressbarcover.style.width = "100%";
        this.progressbarcover.style.bottom = "0px";
        this.progressbarcover.style.height = "1em";
        this.progressbarcover.style.backgroundColor = "transparent";
        this.controlsbase.appendChild(this.progressbarcover);

        this.settingpanel = htmlToElement(
            ` <div id="settingpanel" class="is-hidden has-text-light"
            style="position: absolute;top: 0px; width: 100%;height: 100%;background-color: rgba(0, 0, 0, 0.637);">
            </div>
            `
        ) as HTMLElement;
        this.vidcontainer.appendChild(this.settingpanel);
        this.settingclose = htmlToElement(`            <div class="icon toprighttag " id="settingclose">
        <i class=" fas fa-times-circle fa-2x"></i>
    </div>`) as HTMLElement;
        this.settingpanel.appendChild(this.settingclose);
        let panel = htmlToElement(`
        <div class="panel" style="margin:0;padding:5px">
            <div class="panel-heading has-text-light" style="background-color: transparent; padding:0">
                Settings
            </div>
            <div class="panel-block has-text-light" style="padding:0;margin:0">
                <div class="columns is-mobile" style="width: 100%;">
                    <div class="column has-text-centered">
                        <div class="container subtitle has-text-light" style="background-color: transparent;">
                            Quality
                        </div>
                        <div id="qualityholder">

                        </div>
                        
                    </div>
                </div>
            </div>
        </div>`) as HTMLElement;
        this.settingpanel.appendChild(panel);
        this.qualityholder = panel.querySelector("#qualityholder");
        this.loadingicon = htmlToElement(`        <div class="icon centertag is-hidden" id="loadingicon">
        <i class=" fas  fa-spinner fa-pulse fa-3x fa-fw"></i>
    </div>`) as HTMLElement;
        this.vidcontainer.appendChild(this.loadingicon);

        if(this.data){
            this.exec(this.data);
        }
    }

    changedata(data) {
        this.data=data;
        this.exec(data);
    }

    exec(data) {
        if(!data){
            data=JSON.parse(this.getAttribute("data"))
        }
        if(!this.posterurl){
            this.posterurl=this.getAttribute("poster")
        }

        let vidsources: Array<videosource> = data.videoOnlyStreams;
        let audiosources: Array<audiosource> = data.audioOnlyStreams;

        let vidtag: HTMLVideoElement = this.vidtag
        let previewvid = this.previewvid;
        let audtag: HTMLAudioElement = this.audtag;

        let seekbacktime = 0;

        let controlbase = this.controlsbase;
        // let visiblecontrols = document.getElementById("viscontrols") as HTMLDivElement;

        let playicon = this.playicon;
        let pauseicon = this.pauseicon;
        let loadingicon = this.loadingicon;
        let progressbar = this.progressdiv;
        let buffercontainer = this.buffercontainer;
        let progressbarcover = this.progressbarcover;


        let settingicon = this.settingicon;
        let settingclose = this.settingclose;
        let settingpanel = this.settingpanel;
        let qualityholder = this.qualityholder;
        let status = this.status;
        let fullscreenicon = this.fullscreenicon;
        let fullscreeniconexit = this.fullscreenexiticon;

        let vidcontainr = this.vidcontainer;

        vidtag.poster=this.posterurl;

        fullscreenicon.onclick = () => {
            vidcontainr.requestFullscreen({
                navigationUI: "hide"
            });
            this.vidtag.style.maxHeight="100vh";
            fullscreenicon.classList.add("is-hidden");
            fullscreeniconexit.classList.remove("is-hidden");
        }
        fullscreeniconexit.onclick = () => {
            document.exitFullscreen();
            this.vidtag.style.maxHeight="70vh";
            fullscreeniconexit.classList.add("is-hidden");
            fullscreenicon.classList.remove("is-hidden");
        }

        function opensetting() {
            displayquality();
            controlbase.classList.add("is-hidden");
            settingpanel.classList.remove("is-hidden");
        }
        function closesetting() {
            // controlbase.classList.remove("is-hidden");
            settingpanel.classList.add("is-hidden");
        }

        settingicon.onclick = () => opensetting();

        settingclose.onclick = () => closesetting();

        playicon.onclick = () => {
            vidtag.play();
        }
        pauseicon.onclick = () => {
            vidtag.pause();
        }

        let prefquality = 420;


        let qualities_set = new Set<number>();
        for (let vid of vidsources) {
            qualities_set.add(vid.height);
        }

        function displayquality() {
            let qualities = Array.from(qualities_set);

            let prefquality_real = qualities.sort((a, b) => Math.abs(a - prefquality) - Math.abs(b - prefquality))[0];
            qualities = qualities.sort((a, b) => a - b);

            qualityholder.textContent = ''
            for (let quality of qualities) {
                let qdiv = document.createElement("div");
                qdiv.textContent = quality + "";
                qdiv.classList.add("button")
                qdiv.classList.add("is-outlined");
                qdiv.classList.add("has-text-light");
                qdiv.style.backgroundColor = "transparent";
                qdiv.style.margin = "2px";
                if (quality == prefquality_real) {
                    qdiv.classList.add("is-primary")
                }
                qdiv.onclick = () => {
                    prefquality = quality;
                    displayquality();
                    playprefquality();
                }
                qualityholder.appendChild(qdiv);
            }
        }

        console.log(vidsources)

        let hidtimer = undefined;

        vidcontainr.onmouseenter = () => {
            controlbase.classList.remove("is-hidden");
            hidtimer = setTimeout(() => controlbase.classList.add("is-hidden"), 3000)
        }
        vidcontainr.onmouseleave = () => {
            if (!controlbase.classList.contains("is-hidden")) {
                controlbase.classList.add("is-hidden");
            }
        }

        vidcontainr.onmousemove = () => {
            controlbase.classList.remove("is-hidden");
            if (hidtimer != undefined) {
                clearTimeout(hidtimer);
                hidtimer = setTimeout(() => controlbase.classList.add("is-hidden"), 3000)
            }
        }

        function changestate(state: videostates) {
            if (state == videostates.play) {
                pauseicon.classList.remove("is-hidden");
                playicon.classList.add("is-hidden");
                loadingicon.classList.add("is-hidden");

                audtag.currentTime = vidtag.currentTime;
                audtag.play();
            } else if (state == videostates.pause) {
                playicon.classList.remove("is-hidden");
                pauseicon.classList.add("is-hidden");
                loadingicon.classList.add("is-hidden");
                audtag.pause();
            } else {
                loadingicon.classList.remove("is-hidden");
                playicon.classList.add("is-hidden");
                pauseicon.classList.add("is-hidden");
                audtag.pause();
            }
        }

        function updateloader() {
            if (!loadingicon.classList.contains("is-hidden")) {
                if (vidtag.readyState == HTMLMediaElement.HAVE_ENOUGH_DATA) {
                    if (vidtag.paused) {
                        changestate(videostates.pause)
                    } else {
                        changestate(videostates.play)
                    }
                }
            }
            if (!pauseicon.classList.contains("is-hidden")) {
                if (vidtag.readyState == HTMLMediaElement.HAVE_CURRENT_DATA) {
                    if (vidtag.paused) {
                        changestate(videostates.loading)
                    } else {
                        changestate(videostates.play)
                    }
                }
            }
        }

        function updatebuffer() {
            buffercontainer.textContent = '';
            for (let bufi = 0; bufi < vidtag.buffered.length; bufi++) {
                let bufdiv = document.createElement("div");
                bufdiv.classList.add("buffersegment")
                bufdiv.classList.add("has-background-info")
                bufdiv.style.left = (vidtag.buffered.start(bufi) / vidtag.duration) * 100 + "%";
                bufdiv.style.width = ((vidtag.buffered.end(bufi) - vidtag.buffered.start(bufi)) / vidtag.duration) * 100 + "%"
                buffercontainer.appendChild(bufdiv);
            }
        }

        vidtag.oncanplay = (state) => {
            if (vidtag.paused) {
                changestate(videostates.pause)
                if (seekbacktime) {
                    vidtag.currentTime = seekbacktime;
                    vidtag.autoplay = true;
                    seekbacktime = 0
                }
            } else {
                changestate(videostates.play)
            }
        }

        vidtag.onplay = () => changestate(videostates.play)

        vidtag.onpause = () => changestate(videostates.pause)

        vidtag.onloadstart = () => changestate(videostates.loading)

        vidtag.onplaying = () => changestate(videostates.play)

        vidtag.onstalled = () => { if (vidtag.paused) changestate(videostates.loading) }

        vidtag.onseeking = () => changestate(videostates.play)

        vidtag.onwaiting = () => changestate(videostates.loading)

        // vidtag.ondurationchange = (ev) => { pro; bufferbar.max=vidtag.duration;};

        vidtag.ontimeupdate = () => {
            progressbar.style.width = (vidtag.currentTime / vidtag.duration) * 100 + "%";
            updatebuffer();
        };

        vidtag.onprogress = () => updatebuffer();

        progressbarcover.onclick = (ev) => {
            let time = (ev.offsetX / progressbarcover.getBoundingClientRect().width) * vidtag.duration;
            vidtag.currentTime = time;
        }
        progressbarcover.onmousemove = (ev) => {
            let time = (parseInt(((ev.offsetX / progressbarcover.getBoundingClientRect().width) * 100).toFixed(0)) / 100) * vidtag.duration;
            previewvid.currentTime = time;
            previewvid.play();
            previewvid.pause();
            status.style.left = clamp((ev.offsetX - status.getBoundingClientRect().width / 2), 0, progressbarcover.getBoundingClientRect().width - previewvid.width) + "px"
        }
        progressbarcover.onmouseout = (ev) => {
            previewvid.parentElement.classList.add("is-hidden")
        }
        progressbarcover.onmouseenter = (ev) => {
            previewvid.parentElement.classList.remove("is-hidden")
        }

        function clamp(num, min, max) {
            return num <= min ? min : num >= max ? max : num;
        }

        function playprefquality() {
            vidsources.sort((a, b) => (Math.abs(a.height - prefquality) - Math.abs(b.height - prefquality)) || parseInt(a.contentLength) - parseInt(b.contentLength))
            seekbacktime = vidtag.currentTime;
            for (let video of vidsources) {
                let canplay = vidtag.canPlayType(video.mimeType);
                if (canplay == "probably") {
                    vidtag.src = video.url;
                    audtag.src = audiosources[0].url;
                    audtag.load()
                    vidtag.load()
                    // vidtag.currentTime=oldtime;
                    console.log("playing: ", video);

                    previewvid.src = vidsources.sort((a, b) => parseInt(a.contentLength) - parseInt(b.contentLength))[0].url;
                    previewvid.load()


                    break;
                }
            }
        }
        playprefquality()


    }

    attributeChangedCallback(name, oldValue, newValue) {
        console.log("attr channge");
        if (name == "data" && oldValue != newValue) {
            let data = JSON.parse(newValue);
            if(this.data){
                this.data = data;
            }else{
                this.data = data;
                this.exec(this.data);
            }
        }else if(name=="poster"){
            console.log("setposter ",newValue);
            this.posterurl=newValue;
            if(this.vidtag){
                this.vidtag.poster=this.posterurl;
            }
        }else if(name=="id"){
            if(newValue!=oldValue){
                this.exec(this.data);
            }
        }
        console.log("name",name)
    }
}
customElements.define("bul-player", BulPlayer)


let holder = document.querySelector(".centertag");
// let bulplayer = document.createElement("bul-player");
// bulplayer.setAttribute("data", JSON.stringify(data.data.video))
// bulplayer.setAttribute("poster", "https://i.ytimg.com/vi/8SfbFwMpsRw/hqdefault.jpg?sqp=-oaymwEjCNACELwBSFryq4qpAxUIARUAAAAAGAElAADIQj0AgKJDeAE=&rs=AOn4CLAuB9BOtmouHrEjo4UE1hyQt084aA");
// holder.appendChild(bulplayer);