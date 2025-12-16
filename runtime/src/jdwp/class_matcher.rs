use crate::jdwp::EventRequestId;
use std::collections::HashMap;

pub struct ClassPatternMatcher {
    exact: HashMap<String, Vec<EventRequestId>>,
    prefix: Vec<(String, EventRequestId)>,
    suffix: Vec<(String, EventRequestId)>,
}

impl ClassPatternMatcher {
    pub fn new() -> Self {
        Self {
            exact: HashMap::new(),
            prefix: Vec::new(),
            suffix: Vec::new(),
        }
    }

    pub fn add(&mut self, pattern: String, request_id: EventRequestId) {
        if pattern.ends_with('*') {
            self.prefix
                .push((pattern[..pattern.len() - 1].to_string(), request_id));
        } else if pattern.starts_with('*') {
            self.suffix.push((pattern[1..].to_string(), request_id));
        } else {
            self.exact.entry(pattern).or_default().push(request_id);
        }
    }

    pub fn remove(&mut self, request_id: EventRequestId) {
        for ids in self.exact.values_mut() {
            ids.retain(|id| *id != request_id);
        }
        self.prefix.retain(|(_, id)| *id != request_id);
        self.suffix.retain(|(_, id)| *id != request_id);
    }

    pub fn matches(&self, class_name: &str) -> Option<Vec<EventRequestId>> {
        let mut ids: Option<Vec<EventRequestId>> = None;

        if let Some(exact_ids) = self.exact.get(class_name) {
            ids.get_or_insert_with(Vec::new).extend(exact_ids);
        }

        for (prefix, id) in &self.prefix {
            if class_name.starts_with(prefix) {
                ids.get_or_insert_with(Vec::new).push(*id);
            }
        }

        for (suffix, id) in &self.suffix {
            if class_name.ends_with(suffix) {
                ids.get_or_insert_with(Vec::new).push(*id);
            }
        }

        ids
    }
}
