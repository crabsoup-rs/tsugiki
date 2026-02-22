use html5ever::interface::ElemName;
use html5ever::{LocalName, Namespace, Prefix};

/// A fully qualified name (with a namespace), used to depict names of tags and attributes.
///
/// Namespaces can be used to differentiate between similar XML fragments. For example:
///
/// ```text
/// // HTML
/// <table>
///   <tr>
///     <td>Apples</td>
///     <td>Bananas</td>
///   </tr>
/// </table>
///
/// // Furniture XML
/// <table>
///   <name>African Coffee Table</name>
///   <width>80</width>
///   <length>120</length>
/// </table>
/// ```
///
/// Without XML namespaces, we can't use those two fragments in the same document
/// at the same time. However if we declare a namespace we could instead say:
///
/// ```text
///
/// // Furniture XML
/// <furn:table xmlns:furn="https://furniture.rs">
///   <furn:name>African Coffee Table</furn:name>
///   <furn:width>80</furn:width>
///   <furn:length>120</furn:length>
/// </furn:table>
/// ```
///
/// and bind the prefix `furn` to a different namespace.
///
/// For this reason we parse names that contain a colon in the following way:
///
/// ```text
/// <furn:table>
///    |    |
///    |    +- local name
///    |
///  prefix (when resolved gives namespace_url `https://furniture.rs`)
/// ```
///
/// NOTE: `Prefix`, `LocalName` and `Prefix` all implement `Deref<str>`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct QualName {
    /// The prefix of qualified (e.g. `furn` in `<furn:table>` above).
    /// Optional (since some namespaces can be empty or inferred), and
    /// only useful for namespace resolution (since different prefix
    /// can still resolve to same namespace)
    ///
    /// ```
    ///
    /// # fn main() {
    /// use tsugiki::{QualName, Namespace, LocalName, Prefix};
    ///
    /// let qual = QualName::new(
    ///     Some(Prefix::from("furn")),
    ///     Namespace::from("https://furniture.rs"),
    ///     LocalName::from("table"),
    /// );
    ///
    /// assert_eq!("furn", &qual.prefix.unwrap());
    ///
    /// # }
    /// ```
    pub prefix: Option<Prefix>,
    /// The namespace after resolution (e.g. `https://furniture.rs` in example above).
    ///
    /// ```
    /// # use tsugiki::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// # let qual = QualName::new(
    /// #    Some(Prefix::from("furn")),
    /// #    Namespace::from("https://furniture.rs"),
    /// #    LocalName::from("table"),
    /// # );
    ///
    /// assert_eq!("https://furniture.rs", &qual.ns);
    /// # }
    /// ```
    ///
    /// When matching namespaces used by HTML we can use `ns!` macro.
    /// Although keep in mind that ns! macro only works with namespaces
    /// that are present in HTML spec (like `html`, `xmlns`, `svg`, etc.).
    ///
    /// ```
    /// # #[macro_use] extern crate tsugiki;
    /// # use tsugiki::{QualName, Namespace, LocalName, Prefix};
    ///
    /// let html_table = QualName::new(
    ///    None,
    ///    ns!(html),
    ///    LocalName::from("table"),
    /// );
    ///
    /// assert!(
    ///   match html_table.ns {
    ///     ns!(html) => true,
    ///     _ => false,
    ///   }
    /// );
    ///
    /// ```
    pub ns: Namespace,
    /// The local name (e.g. `table` in `<furn:table>` above).
    ///
    /// ```
    /// # use tsugiki::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// # let qual = QualName::new(
    /// #    Some(Prefix::from("furn")),
    /// #    Namespace::from("https://furniture.rs"),
    /// #    LocalName::from("table"),
    /// # );
    ///
    /// assert_eq!("table", &qual.local);
    /// # }
    /// ```
    /// When matching local name we can also use the `local_name!` macro:
    ///
    /// ```
    /// # use tsugiki::{QualName, Namespace, LocalName, Prefix, local_name};
    ///
    /// # let qual = QualName::new(
    /// #    Some(Prefix::from("furn")),
    /// #    Namespace::from("https://furniture.rs"),
    /// #    LocalName::from("table"),
    /// # );
    ///
    /// // Initialize qual to furniture example
    ///
    /// assert!(
    ///   match qual.local {
    ///     local_name!("table") => true,
    ///     _ => false,
    ///   }
    /// );
    ///
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
    /// Basic constructor function.
    ///
    /// First let's try it for the following example where `QualName`
    /// is defined as:
    /// ```text
    /// <furn:table> <!-- namespace url is https://furniture.rs -->
    /// ```
    ///
    /// Given this definition, we can define `QualName` using strings.
    ///
    /// ```
    /// use tsugiki::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// let qual_name = QualName::new(
    ///     Some(Prefix::from("furn")),
    ///     Namespace::from("https://furniture.rs"),
    ///     LocalName::from("table"),
    /// );
    /// # }
    /// ```
    ///
    /// If we were instead to construct this element instead:
    ///
    /// ```text
    ///
    /// <table>
    ///  ^^^^^---- no prefix and thus default html namespace
    ///
    /// ```
    ///
    /// Or could define it using macros, like so:
    ///
    /// ```
    /// #[macro_use] extern crate tsugiki;
    /// use tsugiki::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// let qual_name = QualName::new(
    ///     None,
    ///     ns!(html),
    ///     local_name!("table")
    /// );
    /// # }
    /// ```
    ///
    /// Let's analyse the above example.
    /// Since we have no prefix its value is None. Second we have html namespace.
    /// In html5ever html namespaces are supported out of the box,
    /// we can write `ns!(html)` instead of typing `Namespace::from("http://www.w3.org/1999/xhtml")`.
    /// Local name is also one of the HTML elements local names, so can
    /// use `local_name!("table")` macro.
    ///
    #[inline]
    pub fn new(prefix: Option<Prefix>, ns: Namespace, local: LocalName) -> QualName {
        QualName { prefix, ns, local }
    }

    /// Take a reference of `self` as an `ExpandedName`, dropping the unresolved prefix.
    ///
    /// In XML and HTML prefixes are only used to extract the relevant namespace URI.
    /// Expanded name only contains resolved namespace and tag name, which are only
    /// relevant parts of an XML or HTML tag and attribute name respectively.
    ///
    /// In lieu of our XML Namespace example
    ///
    /// ```text
    /// <furn:table> <!-- namespace url is https://furniture.rs -->
    /// ```
    /// For it the expanded name would become roughly equivalent to:
    ///
    /// ```text
    /// ExpandedName {
    ///    ns: "https://furniture.rs",
    ///    local: "table",
    /// }
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

/// <https://www.w3.org/TR/REC-xml-names/#dt-expname>
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct ExpandedName {
    /// Namespace URL
    pub ns: Namespace,
    /// "Local" part of the name
    pub local: LocalName,
}

impl ExpandedName {
    /// Trivial constructor
    pub fn new<N: Into<Namespace>, L: Into<LocalName>>(ns: N, local: L) -> Self {
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
