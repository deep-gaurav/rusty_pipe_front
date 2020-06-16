cd cordovawrap
cordova platform add electron
rm www/*
cp dist/* www/
rm www/index.html
cp fixes/index.html www/index.html
cordova build electron --release