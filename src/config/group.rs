use std::collections::HashSet;

/// Group defines the attributes for a collection of commits to write under a single header in the changelog file.
///
#[derive(Debug)]
pub(crate) struct Group {
    /// The name of the group used as the third level heading in the change log.
    name: String,
    /// Flag to indicate if the group should be written.
    publish: bool,
    /// HashSet of conventional commit types that are part of this group
    cc_types: HashSet<String>,
}

impl Group {
    pub(crate) fn builder() -> GroupBuilder {
        GroupBuilder {
            name: String::new(),
            publish: false,
            cc_types: HashSet::new(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn publish(&self) -> bool {
        self.publish
    }

    pub(crate) fn cc_types(&self) -> Vec<&str> {
        self.cc_types.iter().map(|s| s.as_str()).collect()
    }
}

pub(crate) enum GroupBuilderError {
    NoNameSet,
    NoAssociatedTypes,
}

/// Group defines the attributes for a collection of commits to write under a single header in the changelog file.
///
#[derive(Debug)]
pub(crate) struct GroupBuilder {
    /// The name of the group used as the third level heading in the change log.
    name: String,
    /// Flag to indicate if the group should be written.
    publish: bool,
    /// HashSet of conventional commit types that are part of this group
    cc_types: HashSet<String>,
}

impl GroupBuilder {
    pub(crate) fn build(&self) -> Result<Group, GroupBuilderError> {
        if self.name.is_empty() {
            return Err(GroupBuilderError::NoNameSet);
        }
        if self.cc_types.is_empty() {
            return Err(GroupBuilderError::NoAssociatedTypes);
        }

        Ok(Group {
            name: self.name.clone(),
            publish: self.publish,
            cc_types: self.cc_types.clone(),
        })
    }

    pub(crate) fn allow_publication(&mut self) -> &mut Self {
        self.publish = true;
        self
    }

    pub(crate) fn prevent_publication(&mut self) -> &mut Self {
        self.publish = false;
        self
    }

    pub(crate) fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub(crate) fn insert_cc_type(&mut self, value: &str) -> &mut Self {
        self.cc_types.insert(value.to_string());
        self
    }

    pub(crate) fn remove_cc_type(&mut self, value: &str) -> &mut Self {
        if !self.cc_types.remove(value) {
            log::warn!("Attempt to remove {value} unsuccessful as it was not in the set");
        };
        self
    }
}
