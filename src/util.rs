use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_xml_into_vector(
    xml_string: &str,
    root_element_name: &str,
) -> Vec<HashMap<String, String>> {
    let parser = EventReader::new(xml_string.as_bytes());
    let mut stack: Vec<HashMap<String, String>> = Vec::new();
    let mut current_map: HashMap<String, String> = HashMap::new();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == root_element_name {
                    stack.push(current_map);
                    current_map = HashMap::new();
                } else {
                    current_map.insert(name.local_name, String::new());
                }
            }
            Ok(XmlEvent::Characters(content)) => {
                let key = current_map.keys().last().unwrap().clone();
                let value = current_map.get_mut(&key).unwrap();
                value.push_str(&content);
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name != root_element_name {
                    current_map.remove(&name.local_name);
                } else {
                    stack.push(current_map);
                    current_map = HashMap::new();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    stack
}
