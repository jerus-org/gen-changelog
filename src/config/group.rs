/// Group defines the attributes for a collection of commits to write under a single header in the changelog file.
///
#[derive(Debug)]
pub(crate) struct Group {
    /// The name of the group used as the third level heading in the change log.
    name: String,
    /// Flag to indicate if the group should be written.
    write: bool,
}

impl Group {
    pub(crate) fn new(name: &str, write: bool) -> Self {
        Group {
            name: name.to_string(),
            write,
        }
    }
}
