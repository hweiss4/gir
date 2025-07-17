use crate::{library::*, Config};

impl Library {
    pub fn preprocessing(&mut self, config: &Config) {
        self.add_glib_priority(config.work_mode);
        self.map_custom_namespaces(&config);
    }
}
