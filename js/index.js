const loadApp = async module => {
  module.run_app();
};

import("../pkg").then(loadApp);
