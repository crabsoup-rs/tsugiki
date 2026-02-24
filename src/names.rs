use html5ever::interface::ElemName;
use html5ever::{LocalName, Namespace, Prefix};

/// A fully qualified name (with a namespace), used to depict names of tags and attributes.
///
/// # Namespaces 101
///
/// Namespaces can be used to differentiate between different XML schemas that share tags with
/// the same name. For example:
///
/// ```text
/// <!-- HTML -->
/// <table>
///   <tr>
///     <td>Apples</td>
///     <td>Bananas</td>
///   </tr>
/// </table>
///
/// <!-- Furniture XML -->
/// <table>
///   <name>African Coffee Table</name>
///   <width>80</width>
///   <length>120</length>
/// </table>
/// ```
///
/// Without XML namespaces, we can't use those two fragments in the same document at the same time.
/// However, namespaces allow you to distinguish which tag you actually mean. For example:
///
/// ```text
/// <!-- Furniture XML -->
/// <furn:table xmlns:furn="https://furniture.example.com">
///   <furn:name>African Coffee Table</furn:name>
///   <furn:width>80</furn:width>
///   <furn:length>120</furn:length>
/// </furn:table>
/// ```
///
/// In this example, the `<furn:table>` tag has the namespace `furn`, which resolves to the
/// namespace URL `https://furniture.example.com`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct QualName {
    /// The prefix of qualified (e.g. `furn` in the example above).
    ///
    /// This field is optional because namespaces can be empty or inferred, and are most useful
    /// only once resolved to a namespace URL.
    ///
    /// ```
    /// # fn main() {
    /// # use tsugiki::{QualName, Namespace, LocalName, Prefix};
    /// let qual = QualName::new_prefixed("furn", "https://furniture.example.com", "table");
    /// assert_eq!("furn", &qual.prefix.unwrap());
    /// # }
    /// ```
    pub prefix: Option<Prefix>,

    /// The namespace URL after resolution (e.g. `https://furniture.example.com` in example above).
    ///
    /// ```
    /// # use tsugiki::{QualName, Namespace, LocalName, Prefix, ns};
    /// # fn main() {
    /// let qual = QualName::new_prefixed("furn", "https://furniture.example.com", "table");
    /// assert_eq!("https://furniture.example.com", &qual.ns);
    ///
    /// let qual = QualName::new("http://www.w3.org/1999/xhtml", "table");
    /// assert!(
    ///     match qual.ns {
    ///         ns!(html) => true,
    ///         _ => false,
    ///     }
    /// );
    /// # }
    /// ```
    pub ns: Namespace,

    /// The local name (e.g. `table` in `<furn:table>` above).
    ///
    /// ```
    /// # use tsugiki::{QualName, Namespace, LocalName, Prefix, local_name};
    /// # fn main() {
    /// let qual = QualName::new_prefixed("furn", "https://furniture.example.com", "table");
    /// assert_eq!("table", &qual.local);
    /// assert!(
    ///     match qual.local {
    ///         local_name!("table") => true,
    ///         _ => false,
    ///     }
    /// );
    /// # }
    /// ```
    pub local: LocalName,
}

impl ElemName for &QualName {
    #[inline(always)]
    fn ns(&self) -> &Namespace {
        &self.ns
    }

    #[inline(always)]
    fn local_name(&self) -> &LocalName {
        &self.local
    }
}

impl QualName {
    /// Constructs a qualified element name with no prefix.
    #[inline]
    pub fn new(ns: impl Into<Namespace>, local: impl Into<LocalName>) -> Self {
        QualName {
            prefix: None,
            ns: ns.into(),
            local: local.into(),
        }
    }

    /// Constructs a qualified element name with an explicit prefix.
    #[inline]
    pub fn new_prefixed(
        prefix: impl Into<Prefix>,
        ns: impl Into<Namespace>,
        local: impl Into<LocalName>,
    ) -> QualName {
        QualName {
            prefix: Some(prefix.into()),
            ns: ns.into(),
            local: local.into(),
        }
    }

    /// Take a reference of `self` as an `ExpandedName`, dropping the unresolved prefix.
    ///
    /// In XML and HTML, prefixes are only used to extract the relevant namespace URI. Expanded
    /// names only contain the resolved namespace and tag name.
    ///
    /// For example, the `<furn:table>` tag from our example would resolve to:
    ///
    /// ```
    /// # use tsugiki::{ExpandedName, Namespace, LocalName};
    /// # fn main() { let _name =
    /// ExpandedName {
    ///    ns: Namespace::from("https://furniture.example.com"),
    ///    local: LocalName::from("table"),
    /// }
    /// # ; }
    /// ```
    ///
    #[inline]
    pub fn expanded(&self) -> ExpandedName {
        ExpandedName {
            ns: self.ns.clone(),
            local: self.local.clone(),
        }
    }

    #[inline]
    pub(crate) fn from_html5ever(value: html5ever::QualName) -> Self {
        QualName {
            prefix: value.prefix,
            ns: value.ns,
            local: value.local,
        }
    }

    #[inline]
    pub(crate) fn to_html5ever(&self) -> html5ever::QualName {
        html5ever::QualName {
            prefix: self.prefix.clone(),
            ns: self.ns.clone(),
            local: self.local.clone(),
        }
    }

    #[inline]
    pub(crate) fn into_html5ever(self) -> html5ever::QualName {
        html5ever::QualName {
            prefix: self.prefix,
            ns: self.ns,
            local: self.local,
        }
    }

    #[inline]
    pub(crate) fn expanded_html5ever(&self) -> html5ever::ExpandedName<'_> {
        html5ever::ExpandedName {
            ns: &self.ns,
            local: &self.local,
        }
    }
}

/// An expanded tag name, containing only the namespace URL and local name.
///
/// This is the fully resolved form of an XML or HTML tag name, as the prefix is arbitrary and can
/// be set differently in each document.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct ExpandedName {
    /// The namespace URL of the tag.
    ///
    /// See the [QualName::ns](QualName#structfield.ns) field for more information.
    pub ns: Namespace,
    /// The local name of the tag.
    ///
    /// See the [QualName::local](QualName#structfield.local) field for more information.
    pub local: LocalName,
}

impl ExpandedName {
    /// Create a new expanded name.
    pub fn new(ns: impl Into<Namespace>, local: impl Into<LocalName>) -> Self {
        ExpandedName {
            ns: ns.into(),
            local: local.into(),
        }
    }
}
impl ElemName for ExpandedName {
    fn ns(&self) -> &Namespace {
        &self.ns
    }
    fn local_name(&self) -> &LocalName {
        &self.local
    }
}
