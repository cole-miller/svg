use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;

use crate::node::{Node, Value};

/// A CDATA section.
#[derive(Clone, Debug)]
pub struct CData {
    content: String,
}

impl CData {
    /// Create a node.
    #[inline]
    pub fn new<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        CData {
            content: content.into(),
        }
    }
}

impl fmt::Display for CData {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "<![CDATA[ {} ]]>", self.content)
    }
}

impl Node for CData {
    #[inline]
    fn append<T>(&mut self, _: T)
    where
        T: Node,
    {
    }

    #[inline]
    fn assign<T, U>(&mut self, _: T, _: U)
    where
        T: Into<String>,
        U: Into<Value>,
    {
    }
}

impl super::NodeDefaultHash for CData {
    #[inline]
    fn default_hash(&self, state: &mut DefaultHasher) {
        self.content.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::CData;

    #[test]
    fn cdata_display() {
        let cdata = CData::new("valid");
        assert_eq!(cdata.to_string(), "<![CDATA[ valid ]]>");

        let cdata = CData::new("invalid ]]>");
        assert_eq!(cdata.to_string(), "<![CDATA[ invalid ]]> ]]>");
    }
}
