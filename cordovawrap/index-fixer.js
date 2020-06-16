
const fs = require('fs');

let filename = 'www/index.html'

let rawdata = fs.readFileSync(filename);
let index = rawdata.toString();

index = index.replace("/app","app");
index = index.replace("/bulma","bulma");
index = index.replace("/font","font");

console.log(index)
 
fs.writeFileSync(filename, index);