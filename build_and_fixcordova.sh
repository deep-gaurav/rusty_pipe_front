set -x
cd cordovawrap
pwd
cordova platform add electron --verbose
rm www/*
cp dist/* www/
rm www/index.html
cp fixes/index.html www/index.html
cordova build electron --release