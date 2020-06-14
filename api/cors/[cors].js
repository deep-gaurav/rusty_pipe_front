const allowCors = fn => async (req, res) => {
  res.setHeader('Access-Control-Allow-Credentials', true)
  res.setHeader('Access-Control-Allow-Origin', '*')
  // another option
  // res.setHeader('Access-Control-Allow-Origin', req.header.origin);
  res.setHeader('Access-Control-Allow-Methods', 'GET,OPTIONS')
  res.setHeader(
    'Access-Control-Allow-Headers',
    'X-CSRF-Token, X-Requested-With, Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version, x-youtube-client-name, x-youtube-client-version'
  )
  if (req.method === 'OPTIONS') {
    res.status(200).end()
    return
  }
  return await fn(req, res)
}


function request(uri,options,res){
  const https = require('https');
  const url = require("url");
  

  console.log("Request headers");
  console.log(options);
  let req = https.request(uri,options,(resp)=>{
    console.log('statusCode:', resp.statusCode);
    console.log('headers:', resp.headers);
    let data = '';

    if(resp.statusCode > 300 && resp.statusCode < 400 && resp.headers.location){
      if (url.parse(resp.headers.location).hostname) {
        uri = resp.headers.location;
      } else {
        uri = "https://"+url.parse(uri).host+resp.headers.location;
      }
      request(uri,options,res);
    }else{
      resp.on('data', (chunk) => {
        data += chunk;
      });
      resp.on('end', () => {
        res.send(data);
      });
    }


  });

  req.end();
}

const handler = (req, res) => {
    const {
      query: { cors }
    } = req;
    var uri = Buffer.from(decodeURIComponent(cors), 'base64').toString('ascii');

    var header = {};
    var cphead = ["x-youtube-client-name","x-youtube-client-version","range"];
    for(head of cphead){
      if(req.headers[head]){
        header[head]=req.headers[head]
      }
    }
    if(header["range"]){
      if(header["range"].endsWith("-")){
        var r = header["range"];
        r = r+ (parseInt(r.split("=")[1].split("-")[0])+2000000).toString();
        header["range"]=r;
      }
    }
    // if(req.headers["x-youtube-client-name"]){
    //     header["x-youtube-client-name"]=req.headers["x-youtube-client-name"];
    // }
    // if(req.headers["x-youtube-client-version"]){
    //     header["x-youtube-client-version"]=req.headers["x-youtube-client-version"];
    // }
    // if(req.headers["range"]){
    //   header["range"]=req.headers["range"];
    // }

    const https = require('https');
    var options = {
        headers : header
    }

    console.log(options);

    request(uri,options,res);

    // https.get(uri, options, (resp) => {
    //   let data = '';
    //   var copyheaders = ["Content-Length","Content-Type","Content-Range"]

    //   resp.on('data', (chunk) => {
    //     for(head of copyheaders){
    //       if(resp.headers[head]){
    //         res.setHeader(head,resp.headers[head])
    //       }
    //     }
    //     data += chunk;
    //   });
    //   resp.on('end', () => {
    //     // for(head in resp.headers){
    //     //   res.setHeader(head,resp.headers[head]);
    //     // }
    //     for(head of copyheaders){
    //       if(resp.headers[head]){
    //         res.setHeader(head,resp.headers[head])
    //       }
    //     }
    //     console.log(resp.headers);

    //     res.send(data);
    //   });
    // }).on("error", (err) => {
    //   res.send(err.message)
    // });
  }

  module.exports = allowCors(handler)