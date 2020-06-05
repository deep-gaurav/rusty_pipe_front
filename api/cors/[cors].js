module.exports = (req, res) => {
    const {
      query: { cors }
    } = req;
    var uri = Buffer.from(cors, 'base64').toString('ascii');
  
    res.send(`Req ${uri}!`)
  }