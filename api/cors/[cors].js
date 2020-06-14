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

const handler = (req, res) => {
    const {
      query: { cors }
    } = req;
    var uri = Buffer.from(decodeURIComponent(cors), 'base64').toString('ascii');

    var header = {};
    if(req.headers["x-youtube-client-name"]){
        header["x-youtube-client-name"]=req.headers["x-youtube-client-name"];
    }
    if(req.headers["x-youtube-client-version"]){
        header["x-youtube-client-version"]=req.headers["x-youtube-client-version"];
    }
    if(req.headers["Range"]){
      header["Range"]=req.headers["Range"];
    }

    const https = require('https');
    var options = {
        headers : header
    }
    https.get(uri, options, (resp) => {
      let data = '';
      var copyheaders = ["Content-Length","Content-Type","Content-Range"]

      resp.on('data', (chunk) => {
        for(head of copyheaders){
          if(resp.headers[head]){
            res.setHeader(head,resp.headers[head])
          }
        }
        data += chunk;
      });
      resp.on('end', () => {
        // for(head in resp.headers){
        //   res.setHeader(head,resp.headers[head]);
        // }
        for(head of copyheaders){
          if(resp.headers[head]){
            res.setHeader(head,resp.headers[head])
          }
        }
        console.log(resp.headers);

        res.send(data);
      });
    }).on("error", (err) => {
      res.send(err.message)
    });
  }

  module.exports = allowCors(handler)