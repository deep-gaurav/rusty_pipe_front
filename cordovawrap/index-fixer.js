
const fs = require('fs');

let filename = 'www/index.html'

let rawdata = fs.readFileSync(filename);
let index = rawdata.toString();

index.replaceAll("/app","app");
index.replaceAll("/bulma","bulma");
index.replaceAll("/font","font");

console.log(index)
 
fs.writeFileSync(filename, index);