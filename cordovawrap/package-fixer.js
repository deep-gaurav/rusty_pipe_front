
const fs = require('fs');

let filename = 'package.json'

let rawdata = fs.readFileSync(filename);
let packages = JSON.parse(rawdata);
for(let package in packages['dependencies']){
    packages['devDependencies'][package]=packages['dependencies'][package]
}
packages['dependencies']={};
console.log(packages);

 
let data = JSON.stringify(packages);
fs.writeFileSync(filename, data);