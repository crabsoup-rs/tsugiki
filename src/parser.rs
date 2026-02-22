//! This module handles the integration of `tsugiki` with `html5ever`, and encapsulating the
//! internal implementation details of `html5ever` (such as tendrils and

use crate::attributes;
use crate::tree::NodeRef;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::{ElementFlags, NodeOrText, TreeBuilderOpts, TreeSink};
use html5ever::{self, Attribute, QualName};
use std::borrow::Cow;
use std::rc::Rc;
use tendril::stream::Utf8LossyDecoder;
use tendril::{StrTendril, TendrilSink};

#[doc(inline)]
pub use html5ever::tree_builder::QuirksMode;

/// The parser type used by this crate.
///
/// This exists in order to support incremental parsing of very large files.
pub struct Parser {
    underlying: html5ever::Parser<Sink>,
}
impl Parser {
    /// A convenience function for parsing a single string.
    pub fn one<T>(self, t: T) -> NodeRef
    where
        Self: Sized,
        T: Into<StrTendril>,
    {
        self.underlying.one(t)
    }

    /// Wrap this parser into a `TendrilSink` that accepts UTF-8 bytes.
    ///
    /// Use this when your input is bytes that are known to be in the UTF-8 encoding.
    /// Decoding is lossy, like `String::from_utf8_lossy`.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_utf8(self) -> Utf8LossyDecoder<Self> {
        Utf8LossyDecoder::new(self)
    }
}
impl TendrilSink<tendril::fmt::UTF8> for Parser {
    type Output = NodeRef;
    fn process(&mut self, t: StrTendril) {
        self.underlying.process(t)
    }
    fn error(&mut self, desc: Cow<'static, str>) {
        self.underlying.error(desc)
    }
    fn finish(self) -> Self::Output {
        self.underlying.finish()
    }
}
impl tendril4::TendrilSink<tendril4::fmt::UTF8> for Parser {
    type Output = NodeRef;
    fn process(&mut self, t: tendril4::StrTendril) {
        let converted = tendril::StrTendril::from_slice(&t);
        self.underlying.process(converted)
    }
    fn error(&mut self, desc: Cow<'static, str>) {
        self.underlying.error(desc)
    }
    fn finish(self) -> Self::Output {
        self.underlying.finish()
    }
}

/// Options for the HTML parser.
#[derive(Clone)]
pub struct ParseOpts {
    /// Options for the HTML tokenizer.
    tokenizer: TokenizerOpts,

    /// Options for the HTML tree builder.
    tree_builder: TreeBuilderOpts,

    /// A callback for HTML parse errors (which are never fatal).
    on_parse_error: Vec<Rc<dyn Fn(&str)>>,
}
impl ParseOpts {
    /// Whether to report all parse errors defined by the HTML 5 specification, at moderate
    /// performance cost.
    ///
    /// By default, this is set to `false`.
    pub fn exact_errors(mut self, value: bool) -> Self {
        self.tokenizer.exact_errors = value;
        self.tree_builder.exact_errors = value;
        self
    }

    /// Whether to discard a `U+FEFF BYTE ORDER MARK` codepoint at the start of the stream.
    ///
    /// By default, this is set to `true`.
    pub fn discard_bom(mut self, value: bool) -> Self {
        self.tokenizer.discard_bom = value;
        self
    }

    /// Controls whether the parser should assume that scripting is enabled.
    ///
    /// * If scripting is enabled then the contents of a `<noscript>` element are parsed as a
    ///   single text node.
    /// * If scripting is not enabled then the contents of a `<noscript>` element are parsed as a
    ///   normal tree of nodes.
    ///
    /// By default, this is set to `true`.
    pub fn scripting_enabled(mut self, value: bool) -> Self {
        self.tree_builder.scripting_enabled = value;
        self
    }

    /// Adds a hook to be called when an error occurs.
    ///
    /// When called multiple times, all provided hook functions are called and previous values do
    /// not override new values.
    pub fn parse_error_handler(mut self, handler: impl Fn(&str) + 'static) -> Self {
        self.on_parse_error.push(Rc::new(handler));
        self
    }
}
impl Default for ParseOpts {
    fn default() -> Self {
        // We inline the default values here, instead of using the `::default()` method, because
        // this lets us stay consistent if the defaults change down the line for html5ever.
        Self {
            tokenizer: TokenizerOpts {
                exact_errors: false,
                discard_bom: true,
                profile: false,
                initial_state: None,
                last_start_tag_name: None,
            },
            tree_builder: TreeBuilderOpts {
                exact_errors: false,
                scripting_enabled: true,
                iframe_srcdoc: false,
                drop_doctype: false,
                quirks_mode: QuirksMode::NoQuirks,
            },
            on_parse_error: Vec::new(),
        }
    }
}
impl AsRef<ParseOpts> for ParseOpts {
    fn as_ref(&self) -> &ParseOpts {
        self
    }
}

/// Parse an HTML document with html5ever and the default configuration.
pub fn parse_html() -> Parser {
    parse_html_with_options(ParseOpts::default())
}

/// Parse an HTML document with html5ever with custom configuration.
pub fn parse_html_with_options(opts: impl AsRef<ParseOpts>) -> Parser {
    let opts = opts.as_ref();
    let sink = Sink {
        document_node: NodeRef::new_document(),
        on_parse_error: opts.on_parse_error.clone(),
    };
    let html5opts = html5ever::ParseOpts {
        tokenizer: opts.tokenizer.clone(),
        tree_builder: opts.tree_builder,
    };
    Parser {
        underlying: html5ever::parse_document(sink, html5opts),
    }
}

/// Parse an HTML fragment with html5ever and the default configuration.
pub fn parse_fragment(ctx_name: QualName, ctx_attr: Vec<Attribute>) -> Parser {
    parse_fragment_with_options(ParseOpts::default(), ctx_name, ctx_attr)
}

/// Parse an HTML fragment with html5ever with custom configuration.
pub fn parse_fragment_with_options(
    opts: impl AsRef<ParseOpts>,
    ctx_name: QualName,
    ctx_attr: Vec<Attribute>,
) -> Parser {
    let opts = opts.as_ref();
    let sink = Sink {
        document_node: NodeRef::new_document(),
        on_parse_error: opts.on_parse_error.clone(),
    };
    let html5opts = html5ever::ParseOpts {
        tokenizer: opts.tokenizer.clone(),
        tree_builder: opts.tree_builder,
    };
    Parser {
        underlying: html5ever::parse_fragment(sink, html5opts, ctx_name, ctx_attr, true),
    }
}

/// Receives new tree nodes during parsing.
struct Sink {
    document_node: NodeRef,
    on_parse_error: Vec<Rc<dyn Fn(&str)>>,
}

impl TreeSink for Sink {
    type Output = NodeRef;
    type ElemName<'a> = attributes::ExpandedName;

    fn finish(self) -> NodeRef {
        self.document_node
    }

    type Handle = NodeRef;

    #[inline]
    fn parse_error(&self, message: Cow<'static, str>) {
        for handler in &self.on_parse_error {
            handler(&message)
        }
    }

    #[inline]
    fn get_document(&self) -> NodeRef {
        self.document_node.clone()
    }

    #[inline]
    fn set_quirks_mode(&self, mode: QuirksMode) {
        self.document_node
            .as_document()
            .unwrap()
            .borrow_mut()
            .set_quirks_mode(mode)
    }

    #[inline]
    fn same_node(&self, x: &NodeRef, y: &NodeRef) -> bool {
        x == y
    }

    #[inline]
    fn elem_name<'a>(&self, target: &'a NodeRef) -> attributes::ExpandedName {
        let name = &target.as_element().unwrap().borrow().name;
        attributes::ExpandedName {
            ns: name.ns.clone(),
            local: name.local.clone(),
        }
    }

    #[inline]
    fn create_element(
        &self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> NodeRef {
        NodeRef::new_element(
            name,
            attrs.into_iter().map(|attr| {
                let Attribute {
                    name: QualName { prefix, ns, local },
                    value,
                } = attr;
                let value = String::from(value);
                (
                    attributes::ExpandedName { ns, local },
                    attributes::Attribute { prefix, value },
                )
            }),
        )
    }

    #[inline]
    fn create_comment(&self, text: StrTendril) -> NodeRef {
        NodeRef::new_comment(text)
    }

    #[inline]
    fn create_pi(&self, target: StrTendril, data: StrTendril) -> NodeRef {
        NodeRef::new_processing_instruction(target, data)
    }

    #[inline]
    fn append(&self, parent: &NodeRef, child: NodeOrText<NodeRef>) {
        match child {
            NodeOrText::AppendNode(node) => parent.append(node),
            NodeOrText::AppendText(text) => {
                if let Some(last_child) = parent.last_child() {
                    if let Some(existing) = last_child.as_text() {
                        existing.borrow_mut().content.push_str(&text);
                        return;
                    }
                }
                parent.append(NodeRef::new_text(text))
            }
        }
    }

    #[inline]
    fn append_before_sibling(&self, sibling: &NodeRef, child: NodeOrText<NodeRef>) {
        match child {
            NodeOrText::AppendNode(node) => sibling.insert_before(node),
            NodeOrText::AppendText(text) => {
                if let Some(previous_sibling) = sibling.previous_sibling() {
                    if let Some(existing) = previous_sibling.as_text() {
                        existing.borrow_mut().content.push_str(&text);
                        return;
                    }
                }
                sibling.insert_before(NodeRef::new_text(text))
            }
        }
    }

    #[inline]
    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        self.document_node
            .append(NodeRef::new_doctype(name, public_id, system_id))
    }

    #[inline]
    fn add_attrs_if_missing(&self, target: &NodeRef, attrs: Vec<Attribute>) {
        let element = target.as_element().unwrap();
        let attributes = &mut element.borrow_mut().attributes;

        for Attribute {
            name: QualName { prefix, ns, local },
            value,
        } in attrs
        {
            attributes
                .map
                .entry(attributes::ExpandedName { ns, local })
                .or_insert_with(|| {
                    let value = String::from(value);
                    attributes::Attribute { prefix, value }
                });
        }
    }

    #[inline]
    fn remove_from_parent(&self, target: &NodeRef) {
        target.detach()
    }

    #[inline]
    fn reparent_children(&self, node: &NodeRef, new_parent: &NodeRef) {
        // FIXME: Can this be done more effciently in rctree,
        // by moving the whole linked list of children at once?
        for child in node.children() {
            new_parent.append(child)
        }
    }

    #[inline]
    fn mark_script_already_started(&self, _node: &NodeRef) {
        // FIXME: Is this useful outside of a browser?
    }

    #[inline]
    fn get_template_contents(&self, target: &NodeRef) -> NodeRef {
        target
            .as_element()
            .unwrap()
            .borrow()
            .template_contents
            .clone()
            .unwrap()
    }

    fn append_based_on_parent_node(
        &self,
        element: &NodeRef,
        prev_element: &NodeRef,
        child: NodeOrText<NodeRef>,
    ) {
        if element.parent().is_some() {
            self.append_before_sibling(element, child)
        } else {
            self.append(prev_element, child)
        }
    }
}
