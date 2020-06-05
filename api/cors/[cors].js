module.exports = (req, res) => {
    const {
      query: { cors }
    } = req;
    var uri = Buffer.from(cors, 'base64').toString('ascii');
  


    const https = require('https');

    https.get(uri, (resp) => {
      let data = '';

      // A chunk of data has been recieved.
      resp.on('data', (chunk) => {
        data += chunk;
      });

      // The whole response has been received. Print out the result.
      resp.on('end', () => {
        res.send(data);
      });

    }).on("error", (err) => {
      res.send(err.message)
    });



  }