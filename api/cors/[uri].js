module.exports = (req, res) => {
    const {
      query: { uri }
    } = req;
    // var reuri = atob(uri);
  
    res.send(`Req ${uri}!`)
  }