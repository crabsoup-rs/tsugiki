use html5ever::{QualName, local_name, ns};
use tsugiki::{Attribute, ExpandedName, NodeRef, parse_html};

#[test]
fn test_append() {
    let document = parse_html().one("<div></div>");
    let div = document.select_first("div").unwrap();
    let span = NodeRef::new_element(QualName::new(None, ns!(html), local_name!("span")), vec![]);
    div.as_node().append(span.clone());

    assert_eq!(div.as_node().first_child().unwrap(), span);
    assert_eq!(div.as_node().last_child().unwrap(), span);
    assert_eq!(span.parent().unwrap(), *div.as_node());
    assert_eq!(div.as_node().to_string(), "<div><span></span></div>");
}

#[test]
fn test_prepend() {
    let document = parse_html().one("<div><p></p></div>");
    let div = document.select_first("div").unwrap();
    let span = NodeRef::new_element(QualName::new(None, ns!(html), local_name!("span")), vec![]);
    div.as_node().prepend(span.clone());

    assert_eq!(div.as_node().first_child().unwrap(), span);
    let p = document.select_first("p").unwrap();
    assert_eq!(span.next_sibling().unwrap(), *p.as_node());
    assert_eq!(p.as_node().previous_sibling().unwrap(), span);
    assert_eq!(div.as_node().to_string(), "<div><span></span><p></p></div>");
}

#[test]
fn test_insert_after() {
    let document = parse_html().one("<div><p></p></div>");
    let p = document.select_first("p").unwrap();
    let span = NodeRef::new_element(QualName::new(None, ns!(html), local_name!("span")), vec![]);
    p.as_node().insert_after(span.clone());

    assert_eq!(p.as_node().next_sibling().unwrap(), span);
    assert_eq!(span.previous_sibling().unwrap(), *p.as_node());
    let div = document.select_first("div").unwrap();
    assert_eq!(div.as_node().last_child().unwrap(), span);
    assert_eq!(div.as_node().to_string(), "<div><p></p><span></span></div>");
}

#[test]
fn test_insert_before() {
    let document = parse_html().one("<div><p></p></div>");
    let p = document.select_first("p").unwrap();
    let span = NodeRef::new_element(QualName::new(None, ns!(html), local_name!("span")), vec![]);
    p.as_node().insert_before(span.clone());

    assert_eq!(p.as_node().previous_sibling().unwrap(), span);
    assert_eq!(span.next_sibling().unwrap(), *p.as_node());
    let div = document.select_first("div").unwrap();
    assert_eq!(div.as_node().first_child().unwrap(), span);
    assert_eq!(div.as_node().to_string(), "<div><span></span><p></p></div>");
}

#[test]
fn test_detach() {
    let document = parse_html().one("<div><p></p></div>");
    let p = document.select_first("p").unwrap();
    p.as_node().detach();

    assert!(p.as_node().parent().is_none());
    let div = document.select_first("div").unwrap();
    assert!(div.as_node().first_child().is_none());
    assert!(document.select_first("p").is_err());
    assert_eq!(div.as_node().to_string(), "<div></div>");
}

#[test]
fn test_element_data() {
    let name = QualName::new(None, ns!(html), local_name!("div"));
    let element = NodeRef::new_element(
        name.clone(),
        vec![(
            ExpandedName::new(ns!(), local_name!("class")),
            Attribute {
                prefix: None,
                value: "test".to_string(),
            },
        )],
    );

    let element_data_ref = element.as_element().unwrap();
    let element_data = element_data_ref.borrow();

    assert_eq!(element_data.name, name);
    assert_eq!(element_data.attributes.get("class").unwrap(), "test");

    drop(element_data);
    element_data_ref
        .borrow_mut()
        .attributes
        .insert("id", "main".to_string());
    assert_eq!(
        element_data_ref.borrow().attributes.get("id").unwrap(),
        "main"
    );
    assert_eq!(
        element.to_string(),
        "<div class=\"test\" id=\"main\"></div>"
    );
}

#[test]
fn test_text_data() {
    let text = NodeRef::new_text("hello");
    let text_data_ref = text.as_text().unwrap();

    assert_eq!(text_data_ref.borrow().content, "hello");
    assert_eq!(text.to_string(), "hello");

    text_data_ref.borrow_mut().content = "world".to_string();
    assert_eq!(text_data_ref.borrow().content, "world");
    assert_eq!(text.to_string(), "world");
}

#[test]
fn test_doctype_data() {
    let doctype = NodeRef::new_doctype("html", "public", "system");
    {
        let data_ref = doctype.as_doctype().unwrap();
        let data = data_ref.borrow();

        assert_eq!(data.name, "html");
        assert_eq!(data.public_id, "public");
        assert_eq!(data.system_id, "system");
    }
    assert_eq!(doctype.to_string(), "<!DOCTYPE html>");

    {
        let data_ref = doctype.as_doctype().unwrap();
        let mut data = data_ref.borrow_mut();
        data.name = "HTML".to_string();
        data.public_id = "-//W3C//DTD HTML 4.01//EN".to_string();
        data.system_id = "http://www.w3.org/TR/html4/strict.dtd".to_string();
    }

    {
        let data_ref = doctype.as_doctype().unwrap();
        let data = data_ref.borrow();
        assert_eq!(data.name, "HTML");
        assert_eq!(data.public_id, "-//W3C//DTD HTML 4.01//EN");
        assert_eq!(data.system_id, "http://www.w3.org/TR/html4/strict.dtd");
    }
}
