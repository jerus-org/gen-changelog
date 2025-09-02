use std::collections::BTreeMap;

pub(crate) trait HeadingMgmt {
    fn add_heading(&mut self, group: &str) -> &mut Self;
    fn add_miscellaneous(&mut self) -> &mut Self {
        self
    }
    fn remove_miscellaneous(&mut self) -> &mut Self {
        self
    }
}

impl HeadingMgmt for BTreeMap<u8, String> {
    fn add_heading(&mut self, group: &str) -> &mut Self {
        let i = self.len() as u8;
        if i == u8::MAX {
            log::warn!("maximum number of groups created ({})", u8::MAX);
            self
        } else if self.iter().any(|g| g.1 == &group.to_string()) {
            self
        } else {
            if self.iter().any(|g| g.1 == &"Miscellaneous".to_string()) && group != "Miscellaneous"
            {
                self.insert(i - 1, group.to_string());
                self.insert(i, "Miscellaneous".to_string());
            } else {
                self.insert(i, group.to_string());
            }

            self
        }
    }

    fn add_miscellaneous(&mut self) -> &mut Self {
        if self.iter().any(|g| g.1 == "Miscellaneous") {
            self
        } else {
            self.add_heading("Miscellaneous");
            self
        }
    }

    fn remove_miscellaneous(&mut self) -> &mut Self {
        if self.iter().any(|g| g.1 == "Miscellaneous") {
            let key = self.len() as u8 - 1;
            self.remove(&key);
            self
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::HeadingMgmt;

    #[test]
    fn test_keep_misc_last() {
        let mut headings = BTreeMap::new();
        headings.add_heading("added");
        assert!(headings.first_entry().is_some());
        headings.add_heading("Miscellaneous");
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_eq!(last, &"Miscellaneous".to_string());
        headings.add_heading("changed");
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_eq!(last, &"Miscellaneous".to_string());
    }

    #[test]
    fn test_add_and_remove_miscellaneous() {
        let mut headings = BTreeMap::new();
        headings.add_heading("added");
        assert!(headings.first_entry().is_some());
        headings.add_miscellaneous();
        assert_eq!(headings.len(), 2);
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_eq!(last, &"Miscellaneous".to_string());
        headings.remove_miscellaneous();
        assert_eq!(headings.len(), 1);
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_ne!(last, &"Miscellaneous".to_string());
    }

    #[test]
    fn test_adding_group_idempotence() {
        let mut headings = BTreeMap::new();
        headings.add_heading("added");
        assert_eq!(headings.len(), 1);
        assert!(headings.last_entry().is_some());
        headings.add_heading("added");
        assert_eq!(headings.len(), 1);
        assert!(headings.last_entry().is_some());
    }
}
