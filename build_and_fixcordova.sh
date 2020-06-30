set -x
# rm -rf cordovawrap
# cordova create cordovawrap
export USE_HARD_LINKS=false
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

node package-fixer.js
node index-fixer.js
cordova build electron --verbose
cd ..
mkdir deploys
cp cordovawrap/platforms/electron/build/*.exe deploys/
cp cordovawrap/platforms/electron/build/*.AppImage deploys/
ls deploys/

# ls -R
