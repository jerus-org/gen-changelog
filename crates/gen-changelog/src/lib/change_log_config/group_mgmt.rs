use std::collections::HashMap;

use crate::change_log_config::group::Group;

pub(crate) trait GroupMgmt {
    fn add_group(&mut self, group: Group) -> &mut Self;
    fn remove_group(&mut self, key: &str) -> &mut Self;
    fn set_to_publish(&mut self, group_name: &str) -> &mut Self;
    fn unset_to_publish(&mut self, group_name: &str) -> &mut Self;
}

impl GroupMgmt for HashMap<String, Group> {
    fn add_group(&mut self, group: Group) -> &mut Self {
        let key = group.name().to_string();
        let value = group;
        self.insert(key, value);
        self
    }

    fn remove_group(&mut self, key: &str) -> &mut Self {
        self.remove(key);
        self
    }

    fn set_to_publish(&mut self, group_name: &str) -> &mut Self {
        if let Some(group) = self.get_mut(group_name) {
            group.set_publish();
        } else {
            log::warn!("group to publish was not found")
        }

        self
    }

    fn unset_to_publish(&mut self, group_name: &str) -> &mut Self {
        if let Some(group) = self.get_mut(group_name) {
            group.unset_publish();
        } else {
            log::warn!("group to publish was not found")
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::GroupMgmt;
    use crate::change_log_config::group::Group;

    fn create_added_group() -> Group {
        let gb = Group::builder();
        let gb = gb.set_name("added");
        let gb = gb.insert_cc_type("feat");
        gb.build()
    }

    fn create_misc_group() -> Group {
        let gb = Group::builder();
        let gb = gb.set_name("miscellaneous");
        let gb = gb.insert_cc_type("misc");
        gb.build()
    }

    #[test]
    fn test_keep_misc_last() {
        let added_group = create_added_group();
        let added_key = added_group.name().to_string();
        let misc_group = create_misc_group();
        let misc_key = misc_group.name().to_string();
        let mut groups = HashMap::new();
        groups.add_group(added_group);
        assert!(groups.contains_key(&added_key));
        groups.add_group(misc_group);
        assert!(groups.contains_key(&misc_key));
    }

    #[test]
    fn test_adding_group_idempotence() {
        let added_group = create_added_group();
        let added_key = added_group.name().to_string();
        let misc_group = create_misc_group();
        let mut groups = HashMap::new();
        groups.add_group(added_group.clone());
        groups.add_group(misc_group);
        assert_eq!(groups.len(), 2);
        groups.add_group(added_group.clone());
        assert_eq!(groups.len(), 2);

        groups.remove_group(&added_key);
        assert_eq!(groups.len(), 1);
        groups.add_group(added_group.clone());
        assert_eq!(groups.len(), 2);
    }
}
