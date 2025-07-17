use crate::{library::*, Config};

impl Library {
    pub fn map_custom_namespaces(&mut self, config: &Config) {
        for (ns, path) in &config.custom_namespace_mappings {
            if let Some(ns_id) = self.find_namespace(ns) {
                let ns = self.namespace_mut(ns_id);
                for typ in &mut ns.types {
                    if let Some(Type::Record(Record { name, .. })) = typ {
                        *typ = Some(Type::Basic(Basic::Typedef(format!("{path}::{name}",))));
                    }
                }
            }
        }
    }
}
