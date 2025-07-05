use yamlpath::{Document, Query};

fn main() {
    // Example YAML
    let yaml = r#"
foo:
  bar:
    - a
    - b
"#;

    // Parse the YAML document
    let doc = Document::new(yaml).unwrap();

    // Use a JSON Pointer to select foo.bar[1] (the value "b")
    let pointer = "/foo/bar/1";
    let query = Query::from_json_pointer(pointer).unwrap();

    // Query the document for the feature at that path
    let feature = doc.query_pretty(&query).unwrap();

    // Extract the YAML source for the feature
    let value = doc.extract(&feature);

    println!("YAML at '{}': '{}'", pointer, value.trim());
}
