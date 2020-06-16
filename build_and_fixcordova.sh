set -x
# rm -rf cordovawrap
# cordova create cordovawrap
cd cordovawrap
mkdir platforms
mkdir plugins
mkdir node_modules
mkdir www
npm install
cordova platform add electron
cordova --version
rm www/*
cp ../dist/* www/
rm www/index.html
cp fixes/index.html www/index.html
cordova build electron --release