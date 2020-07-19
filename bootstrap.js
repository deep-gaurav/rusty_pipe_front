import './static/style.scss';

import './player.ts';

import("./pkg").then(module => {
  module.run_app();
});
