
const fs = require('fs');

let filename = 'www/index.html'

let rawdata = fs.readFileSync(filename);
let index = rawdata.toString();

index = index.replace("/app","app");
index = index.replace("/bulma","bulma");
index = index.replace("/font","font");
index = index.replace("<body>","<body><base href=\"\">")

console.log(index)
 
fs.writeFileSync(filename, index);
