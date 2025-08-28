use std::collections::BTreeMap;

pub(crate) trait GroupMgmt {
    fn add_group(&mut self, group: &str) -> &mut Self;
    fn add_miscellaneous(&mut self) -> &mut Self;
    fn remove_miscellaneous(&mut self) -> &mut Self;
}

impl GroupMgmt for BTreeMap<u8, String> {
    fn add_group(&mut self, group: &str) -> &mut Self {
        let i = self.len() as u8;
        if i == u8::MAX {
            log::warn!("maximum number of groups created ({})", u8::MAX);
            self
        } else if self.iter().any(|g| g.1 == &group.to_string()) {
            self
        } else {
            if self.iter().any(|g| g.1 == &"misc".to_string()) {
                self.insert(i - 1, group.to_string());
                self.insert(i, "misc".to_string());
            } else {
                self.insert(i, group.to_string());
            }

            self
        }
    }

    fn add_miscellaneous(&mut self) -> &mut Self {
        if self.iter().any(|g| g.1 == "misc") {
            self
        } else {
            self.add_group("misc");
            self
        }
    }

    fn remove_miscellaneous(&mut self) -> &mut Self {
        if self.iter().any(|g| g.1 == "misc") {
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

    use super::GroupMgmt;

    #[test]
    fn test_keep_misc_last() {
        let mut headings = BTreeMap::new();
        headings.add_group("added");
        assert!(headings.first_entry().is_some());
        headings.add_group("misc");
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_eq!(last, &"misc".to_string());
        headings.add_group("changed");
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_eq!(last, &"misc".to_string());
    }

    #[test]
    fn test_add_and_remove_miscellaneous() {
        let mut headings = BTreeMap::new();
        headings.add_group("added");
        assert!(headings.first_entry().is_some());
        headings.add_miscellaneous();
        assert_eq!(headings.len(), 2);
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_eq!(last, &"misc".to_string());
        headings.remove_miscellaneous();
        assert_eq!(headings.len(), 1);
        assert!(headings.last_entry().is_some());
        let last = headings.last_entry().unwrap();
        let last = last.get();
        assert_ne!(last, &"misc".to_string());
    }

    #[test]
    fn test_adding_group_idempotence() {
        let mut headings = BTreeMap::new();
        headings.add_group("added");
        assert_eq!(headings.len(), 1);
        assert!(headings.last_entry().is_some());
        headings.add_group("added");
        assert_eq!(headings.len(), 1);
        assert!(headings.last_entry().is_some());
    }
}
