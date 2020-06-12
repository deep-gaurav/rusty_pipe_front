const { execSync } = require('child_process');
const Bundler = require('parcel-bundler');
const Path = require('path');
const chokidar = require('chokidar');
const dotenv = require('dotenv');
dotenv.config();
const express = require("express");
const http = require('http');
const path = require("path").posix;


const entryFiles = Path.join(__dirname, 'index.html');

const buildType = process.argv[2];

const options = {
    outDir: './dist', 
    // outFile: 'index.html',
    // publicUrl: '/',
    // watch: buildType !=='production',
    minify: buildType === 'production',
  };

(async () => {
    const bundler = new Bundler(entryFiles, options);

    if (buildType!=='production'){

        chokidar.watch(['./crate/src', './crate/Cargo.toml']).on('change', async (event, path) => {
            console.log(`there are new changes in '${path}'. Start to rebuild rustwasm sources`);
    
            bundler.bundle();
    
            bundler.hmr.broadcast({
                type: 'reload'
            });
        });
    }

    bundler.on('buildStart', () => {
        const prevtBuildFile = Path.join(__dirname, './wasm_pack_cmd');
        console.log(`running: ${prevtBuildFile}`);
        execSync(`${prevtBuildFile} ${buildType === 'production' ? '' : '--dev'}`, {stdio: 'inherit'});
    });

    if(buildType!=='production'){
        // await bundler.serve(process.env.SERVER_ADDRESS || 1234);
        let app = express();
        app.use(bundler.middleware());
        app.get("*", (req, res) =>
            res.sendFile(path.join(__dirname + "/dist/index.html"))
        );

        http.createServer(app).listen(3000, () => {
            console.log(`http server started on port 3000`);
        });
    }else{
        await bundler.bundle();
    }
})();

