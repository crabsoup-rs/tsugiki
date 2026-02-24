use tsugiki::dom::{NodeRef, QuirksMode};
use tsugiki::parse_document;

#[test]
fn test_default_document_quirks() {
    let document = NodeRef::new_document();
    let document_data_ref = document.as_document().unwrap();
    // Default quirks mode is NoQuirks
    assert_eq!(
        document_data_ref.borrow().quirks_mode(),
        QuirksMode::NoQuirks
    );
    assert_eq!(document.to_string(), "");
}

#[test]
fn test_quirks_mode_no_quirks() {
    let doc = parse_document("<!DOCTYPE html><html></html>");
    assert_eq!(
        doc.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::NoQuirks
    );
}

#[test]
fn test_quirks_mode_quirks_missing_doctype() {
    let doc = parse_document("<html></html>");
    assert_eq!(
        doc.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::Quirks
    );
}

#[test]
fn test_quirks_mode_limited_quirks() {
    let doc = parse_document(
        "<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\"><html></html>",
    );
    assert_eq!(
        doc.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::LimitedQuirks
    );
}

#[test]
fn test_broken_tags_quirks() {
    // Text before DOCTYPE
    let doc = parse_document("  abc <!DOCTYPE html><html></html>");
    assert_eq!(
        doc.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::Quirks
    );
}

#[test]
fn test_multiple_html_tags() {
    let doc = parse_document("<!DOCTYPE html><html><p>1</p><html><p>2</p></html></html>");
    assert_eq!(
        doc.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::NoQuirks
    );
    // The second <html> tag should be ignored or its attributes merged, but structure-wise it doesn't create a new html element.
    assert_eq!(doc.select("html").unwrap().count(), 1);
    assert_eq!(doc.select("p").unwrap().count(), 2);
}

#[test]
fn test_broken_doctype_quirks() {
    // A malformed DOCTYPE should trigger quirks mode
    let doc = parse_document("<!DOCTYPE><html></html>");
    assert_eq!(
        doc.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::Quirks
    );

    let doc2 = parse_document("<!DOC TYPE html><html></html>");
    assert_eq!(
        doc2.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::Quirks
    );
}

#[test]
fn test_comment_before_doctype() {
    // Comments before DOCTYPE should NOT trigger quirks mode
    let doc = parse_document("<!-- comment --><!DOCTYPE html><html></html>");
    assert_eq!(
        doc.as_document().unwrap().borrow().quirks_mode(),
        QuirksMode::NoQuirks
    );
}

#[test]
fn test_nested_a_tags() {
    // <a> tags cannot be nested in HTML, the second <a> should be moved outside.
    let doc = parse_document("<!DOCTYPE html><a href='1'>1<a>2</a>3</a>");
    let a_tags = doc.select("a").unwrap().collect::<Vec<_>>();
    assert_eq!(a_tags.len(), 2);
    assert_eq!(a_tags[0].as_node().to_string(), "<a href=\"1\">1</a>");
    assert_eq!(a_tags[1].as_node().to_string(), "<a>2</a>");
}

#[test]
fn test_self_closing_div() {
    // <div> is not a void element, so <div /> is not a thing in HTML (though it is in XML).
    // The parser should treat it as an opening tag.
    let doc = parse_document("<!DOCTYPE html><div /><span></span></div>");
    // Expected structure: <div><span></span></div>
    let div = doc.select_first("div").unwrap();
    assert_eq!(div.as_node().children().count(), 1);
    assert_eq!(
        div.as_node()
            .first_child()
            .unwrap()
            .as_element()
            .unwrap()
            .borrow()
            .name
            .local
            .to_string(),
        "span"
    );
}

#[test]
fn test_table_correction() {
    // Content outside <td>/<th> in a <table> should be moved outside or wrapped.
    let doc = parse_document("<!DOCTYPE html><table><tr><td>1</td>2</tr>3</table>");
    let p = doc.select("table").unwrap().count();
    assert_eq!(p, 1);
    let html = doc.to_string();
    assert!(html.contains("23<table>"));
}

#[test]
fn test_tags_in_title() {
    // Tags inside <title> should be treated as text.
    let doc = parse_document("<!DOCTYPE html><title><span>test</span></title>");
    let title = doc.select_first("title").unwrap();
    assert_eq!(title.as_node().children().count(), 1);
    assert!(title.as_node().first_child().unwrap().as_text().is_some());
    assert_eq!(title.text_contents(), "<span>test</span>");
}

#[test]
fn test_misplaced_body_head() {
    // Multiple body/head tags should have their attributes merged and content put in one.
    let doc = parse_document(
        "<!DOCTYPE html><head><meta charset='utf-8'></head><head><title>test</title></head><body>1</body><body class='b'>2</body>",
    );
    assert_eq!(doc.select("head").unwrap().count(), 1);
    assert_eq!(doc.select("body").unwrap().count(), 1);
    let body = doc.select_first("body").unwrap();
    assert_eq!(body.as_node().to_string(), "<body class=\"b\">12</body>");
}

#[test]
fn test_template_content_parsing() {
    // <template> contents are in a document fragment.
    let doc = parse_document("<!DOCTYPE html><template><p>test</p></template>");
    let template = doc.select_first("template").unwrap();
    let template_data = template.borrow();
    let contents = template_data.template_contents.as_ref().unwrap();

    assert_eq!(contents.children().count(), 1);
    let p = contents.first_child().unwrap();
    assert_eq!(p.as_element().unwrap().borrow().name.local.to_string(), "p");
    assert_eq!(p.text_contents(), "test");
}

#[test]
fn test_script_content_not_parsed() {
    // Content of <script> (and <style>) should be treated as text, not elements.
    let doc = parse_document("<!DOCTYPE html><script><div></div></script>");
    let script = doc.select_first("script").unwrap();
    assert_eq!(script.as_node().children().count(), 1);
    assert!(script.as_node().first_child().unwrap().as_text().is_some());
    assert_eq!(script.text_contents(), "<div></div>");
}

#[test]
fn test_misplaced_table_elements() {
    // <tr> without <table> should be auto-wrapped or handled.
    let doc = parse_document("<!DOCTYPE html><tr><td>1</td></tr>");
    assert_eq!(doc.select("tr").unwrap().count(), 0);
    assert!(doc.to_string().contains("1"));
}

#[test]
fn test_unclosed_tags_nesting() {
    // Unclosed tags should be nested according to HTML spec.
    let doc = parse_document("<!DOCTYPE html><p>1<div>2");
    // <p> cannot contain <div>, so <p> should be closed.
    let p = doc.select_first("p").unwrap();
    assert_eq!(p.as_node().children().count(), 1);
    assert_eq!(p.text_contents(), "1");

    let div = doc.select_first("div").unwrap();
    assert_eq!(div.text_contents(), "2");
    assert!(p.as_node().next_sibling().is_some());
}
