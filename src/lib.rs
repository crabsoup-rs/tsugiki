/*!

Tsugiki (接ぎ木) is an HTML tree manipulation library for Rust.

*/

#![deny(missing_docs)]

#[macro_use]
extern crate html5ever;

mod attributes;
mod cell_extras;
mod names;
mod node_data_ref;
mod parser;
mod select_impl;
mod serializer;
mod tree;

pub mod iter;

pub use parser::{
    ParseOpts, Parser, parse_fragment, parse_fragment_with_options, parse_html,
    parse_html_with_options,
};

/// Contains a list of types used to handle CSS selectors.
pub mod select {
    #[doc(inline)]
    pub use crate::select_impl::{Selector, SelectorCache, SelectorSet, Specificity};
}

/// Contains types used for representing and manipulating HTML trees.
pub mod dom {
    pub use crate::attributes::{Attribute, Attributes};
    pub use crate::names::{ExpandedName, LocalName, Namespace, Prefix, QualName};
    pub use crate::node_data_ref::NodeDataRef;
    pub use crate::tree::{
        DoctypeData, DocumentData, ElementData, Node, NodeData, NodeRef, ProcessingInstructionData,
        QuirksMode, TextData,
    };

    /// Makes a const [`LocalName`] from a literal.
    ///
    /// The string given must be one of the static HTML tags supported by the underlying HTML library.
    pub use html5ever::local_name;

    /// Makes a const [`Namespace`] from a standard HTML namespace
    /// prefix.
    ///
    /// The string given must be one of the namespaces supported by the underlying HTML library.
    pub use html5ever::ns;

    /// Makes a const [`Namespace`] from a literal containing a namespace URL.
    ///
    /// The string given must be one of the namespaces supported by the underlying HTML library.
    pub use html5ever::namespace_url;

    /// Makes a const [`Prefix`] from a literal containing a standard HTML namespace
    /// prefix.
    ///
    /// The string given must be one of the namespaces supported by the underlying HTML library.
    pub use html5ever::namespace_prefix;
}

/// Reexports traits that are useful for users of this crate.
pub mod traits {
    pub use crate::iter::{ElementIterator, NodeIterator};
    #[doc(no_inline)]
    pub use tendril::TendrilSink;
}
