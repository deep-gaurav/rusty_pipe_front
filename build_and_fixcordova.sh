set -x
cd cordovawrap
mkdir platforms
mkdir plugins
mkdir node_modules
mkdir www
cordova platform add electron
rm www/*
cp ../dist/* www/
rm www/index.html
cp fixes/index.html www/index.html
cordova build electron --release