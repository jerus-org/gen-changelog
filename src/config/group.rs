use std::collections::HashSet;

/// Group defines the attributes for a collection of commits to write under a single header in the changelog file.
///
#[derive(Debug, Clone)]
pub(crate) struct Group {
    /// The name of the group used as the third level heading in the change log.
    name: String,
    /// Flag to indicate if the group should be written.
    publish: bool,
    /// HashSet of conventional commit types that are part of this group
    cc_types: HashSet<String>,
}

impl Group {
    pub(crate) fn builder() -> GroupBuilder<NoName, NoCCType> {
        GroupBuilder {
            name: NoName,
            publish: false,
            cc_types: NoCCType,
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

// states for the `name` field
#[derive(Debug, Clone)]
pub(crate) struct Name(String);
#[derive(Debug, Clone)]
pub(crate) struct NoName;

// states for the `cc_type` field
#[derive(Debug, Clone)]
pub(crate) struct CCType(HashSet<String>);
#[derive(Debug, Clone)]
pub(crate) struct NoCCType;

/// Group defines the attributes for a collection of commits to write under a single header in the changelog file.
///
#[derive(Debug, Clone)]
pub(crate) struct GroupBuilder<N, C> {
    /// The name of the group used as the third level heading in the change log.
    name: N,
    /// Flag to indicate if the group should be written.
    publish: bool,
    /// HashSet of conventional commit types that are part of this group
    cc_types: C,
}

impl GroupBuilder<Name, CCType> {
    pub(crate) fn build(self) -> Group {
        Group {
            name: self.name.0,
            publish: self.publish,
            cc_types: self.cc_types.0,
        }
    }
}

impl<N, C> GroupBuilder<N, C> {
    pub(crate) fn allow_publication(&mut self) -> &mut Self {
        self.publish = true;
        self
    }

    pub(crate) fn prevent_publication(&mut self) -> &mut Self {
        self.publish = false;
        self
    }
}

impl<C> GroupBuilder<NoName, C> {
    pub(crate) fn set_name(self, name: &str) -> GroupBuilder<Name, C> {
        let name = Name(name.to_string());

        GroupBuilder {
            name,
            publish: self.publish,
            cc_types: self.cc_types,
        }
    }
}

impl<N> GroupBuilder<N, CCType>
where
    N: std::clone::Clone,
{
    pub(crate) fn insert_cc_type(self, value: &str) -> Self {
        let mut new_group = self.clone();
        let mut set = self.cc_types.0;
        set.insert(value.to_string());
        new_group.cc_types = CCType(set);
        new_group
    }

    pub(crate) fn insert_cc_types(self, values: &[&str]) -> Self {
        let mut new_group = self.clone();
        let mut set = self.cc_types.0;

        for v in values {
            set.insert(v.to_string());
        }

        new_group.cc_types = CCType(set);
        new_group
    }

    pub(crate) fn remove_cc_type(self, value: &str) -> Self {
        let mut new_group = self.clone();
        let mut set = self.cc_types.0;
        if set.len() == 1 {
            log::warn!("cannot remove the last type from the set");
            return new_group;
        }

        if !set.remove(value) {
            log::warn!("Attempt to remove {value} unsuccessful as it was not in the set");
        };
        new_group.cc_types = CCType(set);
        new_group
    }
}

impl<N> GroupBuilder<N, NoCCType> {
    pub(crate) fn insert_cc_type(self, value: &str) -> GroupBuilder<N, CCType>
    where
        N: std::clone::Clone,
    {
        let mut set = HashSet::new();
        set.insert(value.to_string());
        GroupBuilder {
            name: self.name,
            publish: self.publish,
            cc_types: CCType(set),
        }
    }

    pub(crate) fn insert_cc_types(self, values: &[&str]) -> GroupBuilder<N, CCType>
    where
        N: std::clone::Clone,
    {
        let mut set = HashSet::new();

        for v in values {
            set.insert(v.to_string());
        }

        GroupBuilder {
            name: self.name,
            publish: self.publish,
            cc_types: CCType(set),
        }
    }
}
