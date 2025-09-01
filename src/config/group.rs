use std::collections::HashSet;

/// Group defines the attributes for a collection of commits to write under a
/// single header in the changelog file.
#[derive(Debug, Clone)]
pub struct Group {
    /// The name of the group used as the third level heading in the change log.
    name: String,
    /// Flag to indicate if the group should be written.
    publish: bool,
    /// HashSet of conventional commit types that are part of this group
    cc_types: HashSet<String>,
}

impl Group {
    pub fn builder() -> GroupBuilder<NoName, NoCCType> {
        GroupBuilder {
            name: NoName,
            publish: false,
            cc_types: NoCCType,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn publish(&self) -> bool {
        self.publish
    }

    pub fn cc_types(&self) -> Vec<&str> {
        self.cc_types.iter().map(|s| s.as_str()).collect()
    }

    pub fn set_publish(&mut self) -> &mut Self {
        self.publish = true;
        self
    }

    pub fn set_no_publish(&mut self) -> &mut Self {
        self.publish = false;
        self
    }

    pub(crate) fn new_with_name_types_and_publish_flag(
        name: &str,
        type_list: &[&str],
        publish: bool,
    ) -> Group {
        let mut cc_types = HashSet::new();

        for value in type_list {
            cc_types.insert(value.to_string());
        }

        Group {
            name: name.to_string(),
            publish,
            cc_types,
        }
    }
}

pub enum GroupBuilderError {
    NoNameSet,
    NoAssociatedTypes,
}

// states for the `name` field
#[derive(Debug, Clone)]
pub struct Name(String);
#[derive(Debug, Clone)]
pub struct NoName;

// states for the `cc_type` field
#[derive(Debug, Clone)]
pub struct CCType(HashSet<String>);
#[derive(Debug, Clone)]
pub struct NoCCType;

/// Group defines the attributes for a collection of commits to write under a
/// single header in the changelog file.
#[derive(Debug, Clone)]
pub struct GroupBuilder<N, C> {
    /// The name of the group used as the third level heading in the change log.
    name: N,
    /// Flag to indicate if the group should be written.
    publish: bool,
    /// HashSet of conventional commit types that are part of this group
    cc_types: C,
}

impl GroupBuilder<Name, CCType> {
    pub fn build(self) -> Group {
        Group {
            name: self.name.0,
            publish: self.publish,
            cc_types: self.cc_types.0,
        }
    }
}

impl<N, C> GroupBuilder<N, C> {
    pub fn allow_publication(&mut self) -> &mut Self {
        self.publish = true;
        self
    }

    pub fn prevent_publication(&mut self) -> &mut Self {
        self.publish = false;
        self
    }
}

impl<C> GroupBuilder<NoName, C> {
    pub fn set_name(self, name: &str) -> GroupBuilder<Name, C> {
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
    pub fn insert_cc_type(self, value: &str) -> Self {
        let mut new_group = self.clone();
        let mut set = self.cc_types.0;
        set.insert(value.to_string());
        new_group.cc_types = CCType(set);
        new_group
    }

    pub fn insert_cc_types(self, values: &[&str]) -> Self {
        let mut new_group = self.clone();
        let mut set = self.cc_types.0;

        for v in values {
            set.insert(v.to_string());
        }

        new_group.cc_types = CCType(set);
        new_group
    }

    pub fn remove_cc_type(self, value: &str) -> Self {
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
    pub fn insert_cc_type(self, value: &str) -> GroupBuilder<N, CCType>
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

    pub fn insert_cc_types(self, values: &[&str]) -> GroupBuilder<N, CCType>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_simple_group() {
        let mut group_builder = Group::builder();
        group_builder.allow_publication();
        let group_builder = group_builder.set_name("test");
        let group_builder = group_builder.insert_cc_type("value");
        let group = group_builder.build();

        assert_eq!(group.name, String::from("test"));
        assert!(group.cc_types.contains("value"));
    }

    #[test]
    fn build_multi_value_individually_group() {
        let group_builder = Group::builder();
        assert!(!group_builder.publish);
        let mut group_builder = group_builder.set_name("test");
        group_builder.allow_publication();
        let group_builder = group_builder.insert_cc_type("one");
        let group_builder = group_builder.insert_cc_type("two");
        let group = group_builder.build();

        assert_eq!(group.name, String::from("test"));
        assert!(group.publish());
        assert!(group.cc_types.contains("one"));
        assert!(group.cc_types.contains("two"));
    }

    #[test]
    fn build_multi_value_list_group() {
        let group_builder = Group::builder();
        assert!(!group_builder.publish);
        let group_builder = group_builder.set_name("test");
        assert!(!group_builder.publish);
        let mut group_builder = group_builder.insert_cc_types(&["one", "two", "three"]);
        group_builder.allow_publication();
        let group = group_builder.build();

        assert_eq!(group.name, String::from("test"));
        assert!(group.publish());
        assert!(group.cc_types.contains("one"));
        assert!(group.cc_types.contains("two"));
        assert!(group.cc_types.contains("three"));
    }
}
