module.exports = (req, res) => {
    const {
      query: { cors }
    } = req;
    var uri = atob(cors);
  
    res.send(`Req ${uri}!`)
  }