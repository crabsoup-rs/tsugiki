### Project Design: Tsugiki

Tsugiki is an HTML manipulation library for Rust, designed for efficient tree traversal and modification, primarily leveraging the `html5ever` parser. It is a fork of the Kuchiki library.

#### Core Abstractions

##### `NodeRef`
- **Definition**: A wrapper around `Rc<Node>`, providing a handle to a node within the tree.
- **Role**: It acts as the primary interface for users to interact with nodes, providing methods for tree manipulation (append, prepend, etc.) and traversal.
- **Ownership**: Since it uses `Rc`, multiple `NodeRef` instances can point to the same node, facilitating tree-wide operations without strict ownership constraints.

##### `Node` and `NodeData`
- **`Node`**: Represents an individual node in the DOM tree, containing pointers to its parent, children, and siblings.
- **`NodeData`**: An enum representing the specific type of node (e.g., `Element`, `Text`, `Comment`, `Document`).
- **Data Encapsulation**: Most variant data is wrapped in `RefCell` (e.g., `RefCell<ElementData>`), enabling interior mutability.

##### `NodeDataRef<T>`
- **Purpose**: A specialized smart pointer that holds a `NodeRef` while dereferencing to a specific component within that node (e.g., `ElementData`).
- **Safety**: It ensures the underlying `Node` remains alive while providing direct access to its data, typically through a `RefCell` borrow.

#### Tree Structure and Navigation

- **Bidirectional Links**: Nodes maintain references to parents, children, and siblings, allowing for versatile traversal in any direction.
- **Iterators**: Tsugiki provides a rich set of iterators (implemented in `src/iter.rs`) for various traversal patterns:
  - `ancestors` / `inclusive_ancestors`
  - `descendants` / `inclusive_descendants`
  - `following_siblings` / `preceding_siblings`
  - `children`
  - `traverse` (pre-order and post-order)

#### Parsing and Serialization

- **`html5ever` Integration**: Parsing (in `src/parser.rs`) is handled by implementing `html5ever::tree_builder::TreeSink`, which constructs the Tsugiki tree from parser events.
- **Serialization**: The library supports serializing nodes back to HTML, typically using `html5ever`'s serialization traits (see `src/serializer.rs`).

#### CSS Selectors

- **Support**: Integrated through the `selectors` crate (in `src/select_impl.rs`).
- **Functionality**: Users can compile CSS selectors and use them to filter node iterators or find specific elements within a subtree.

#### Performance and Low-Level Details

- **`cell_extras.rs`**: Contains low-level optimizations using `unsafe` to allow limited access to values within `Cell<Option<Rc<T>>>` or `Cell<Option<Weak<T>>>` without moving them, which is critical for efficient tree link management.
- **Non-recursive Drop**: `Node` implements a custom `Drop` trait with a non-recursive approach to avoid stack overflows when dropping deep trees.

#### Codestyle

- **Standard Rust Formatting**: Follows standard `rustfmt` conventions.
- **Naming**: Use `CamelCase` for types and `snake_case` for functions, methods, and variables.
- **Documentation**: Use triple-slash (`///`) comments for public APIs and double-slash (`//`) for internal notes. Follow KDoc-style documentation patterns.
- **Linting**: Strict linting is enforced via `#![deny(missing_docs)]` in `src/lib.rs`.
- **Explicit Imports**: Prefer explicit imports over glob imports, especially for external crates.
- **Interior Mutability Patterns**: Use `borrow()` and `borrow_mut()` on `RefCell` judiciously, ensuring borrows are as short-lived as possible to avoid runtime panics.
- **Performance-Conscious**: Avoid unnecessary allocations; prefer passing references or using `Cow` where appropriate.
