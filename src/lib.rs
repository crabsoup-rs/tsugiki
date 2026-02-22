/*!

Tsugiki (接ぎ木) is an HTML tree manipulation library for Rust.

*/

#![deny(missing_docs)]

#[macro_use]
extern crate html5ever;

mod attributes;
mod cell_extras;
pub mod iter;
mod names;
mod node_data_ref;
mod parser;
mod select;
mod serializer;
mod tree;

pub use attributes::{Attribute, Attributes};
pub use names::{ExpandedName, QualName};
pub use node_data_ref::NodeDataRef;
pub use parser::{
    ParseOpts, Parser, parse_fragment, parse_fragment_with_options, parse_html,
    parse_html_with_options,
};
pub use select::{Selector, SelectorCache, Selectors, Specificity};
pub use tree::{
    DoctypeData, DocumentData, ElementData, Node, NodeData, NodeRef, ProcessingInstructionData,
    QuirksMode, TextData,
};

#[doc(inline)]
pub use html5ever::{LocalName, Namespace, Prefix, local_name, namespace_prefix, ns};

/// This module contains a number of traits that are useful when using Tsugiki.
/// It can be used with:
///
/// ```rust
/// use tsugiki::traits::*;
/// ```
pub mod traits {
    pub use crate::iter::{ElementIterator, NodeIterator};
    pub use tendril::TendrilSink;
}
