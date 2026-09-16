use std::collections::BTreeMap;

use anyhow::{Context, Result, bail};
use quick_xml::{Reader, XmlVersion, events::BytesStart, events::Event};

use crate::snapshot::DocumentMetadata;

#[derive(Debug, Clone)]
pub struct Inspection {
    pub doctype: Option<String>,
    pub root_name: Option<String>,
    pub root_attributes: BTreeMap<String, String>,
    pub element_counts: BTreeMap<String, usize>,
    pub attribute_counts: BTreeMap<String, usize>,
    pub enbez_values: Vec<String>,
}

pub fn parse_document_metadata(
    xml: &str,
    law_slug: &str,
    xml_filename: &str,
) -> Result<DocumentMetadata> {
    let mut reader = Reader::from_str(xml);
    let mut result = DocumentMetadata {
        law_slug: law_slug.to_owned(),
        xml_filename: xml_filename.to_owned(),
        ..DocumentMetadata::default()
    };

    let mut capture: Option<String> = None;
    let mut captured = String::new();

    loop {
        match reader.read_event().context("parsing source XML metadata")? {
            Event::Start(e) => {
                let name = e.name().as_ref().to_owned();

                if name == "dokumente" {
                    result.doknr = attr_value(&e, "doknr")?;
                    result.builddate = attr_value(&e, "builddate")?;
                }

                if should_capture(&name, &result) {
                    capture = Some(name.to_owned());
                    captured.clear();
                }
            }
            Event::Text(e) => {
                if capture.is_some() {
                    captured.push_str(&e.xml_content(XmlVersion::Implicit1_0));
                }
            }
            Event::End(e) => {
                if capture.as_deref() == Some(e.name().as_ref()) {
                    let value = captured.trim().to_owned();
                    if !value.is_empty() {
                        match capture.as_deref() {
                            Some("jurabk") if result.jurabk.is_none() => {
                                result.jurabk = Some(value)
                            }
                            Some("ausfertigung-datum") if result.ausfertigung_datum.is_none() => {
                                result.ausfertigung_datum = Some(value)
                            }
                            Some("langue") if result.title.is_none() => result.title = Some(value),
                            Some("standkommentar") if result.stand.is_none() => {
                                result.stand = Some(value)
                            }
                            _ => {}
                        }
                    }
                    capture = None;
                    captured.clear();
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(result)
}

fn should_capture(name: &str, current: &DocumentMetadata) -> bool {
    match name {
        "jurabk" => current.jurabk.is_none(),
        "ausfertigung-datum" => current.ausfertigung_datum.is_none(),
        "langue" => current.title.is_none(),
        "standkommentar" => current.stand.is_none(),
        _ => false,
    }
}

fn attr_value(start: &BytesStart<'_>, wanted: &str) -> Result<Option<String>> {
    for attribute in start.attributes() {
        let attribute = attribute.context("parsing XML attribute")?;
        if attribute.key.as_ref() == wanted {
            return Ok(Some(attribute.value.as_ref().to_owned()));
        }
    }
    Ok(None)
}

pub fn referenced_dtd_url(xml: &str) -> Result<String> {
    let doctype_start = xml
        .find("<!DOCTYPE")
        .context("source XML contains no DOCTYPE declaration")?;
    let after_doctype = &xml[doctype_start..];
    let doctype_end = after_doctype
        .find('>')
        .context("DOCTYPE declaration is not terminated")?;
    let declaration = &after_doctype[..=doctype_end];

    let system_pos = declaration
        .find("SYSTEM")
        .context("DOCTYPE declaration contains no SYSTEM identifier")?;
    let after_system = declaration[system_pos + "SYSTEM".len()..].trim_start();
    let quote = after_system
        .chars()
        .next()
        .filter(|c| *c == '"' || *c == '\'')
        .context("DOCTYPE SYSTEM identifier is not quoted")?;
    let rest = &after_system[quote.len_utf8()..];
    let end = rest
        .find(quote)
        .context("DOCTYPE SYSTEM identifier has no closing quote")?;

    Ok(crate::catalog::prefer_https(&rest[..end]))
}

pub fn inspect(xml: &str) -> Result<Inspection> {
    let mut reader = Reader::from_str(xml);
    let mut inspection = Inspection {
        doctype: None,
        root_name: None,
        root_attributes: BTreeMap::new(),
        element_counts: BTreeMap::new(),
        attribute_counts: BTreeMap::new(),
        enbez_values: Vec::new(),
    };

    let mut in_enbez = false;
    let mut enbez = String::new();

    loop {
        match reader.read_event().context("parsing source XML")? {
            Event::Start(e) => {
                observe_element(&e, &mut inspection)?;

                if inspection.root_name.is_none() {
                    inspection.root_name = Some(e.name().as_ref().to_owned());
                    for attribute in e.attributes() {
                        let attribute = attribute.context("parsing root XML attribute")?;
                        inspection.root_attributes.insert(
                            attribute.key.as_ref().to_owned(),
                            attribute.value.as_ref().to_owned(),
                        );
                    }
                }

                if e.name().as_ref() == "enbez" {
                    in_enbez = true;
                    enbez.clear();
                }
            }
            Event::Empty(e) => observe_element(&e, &mut inspection)?,
            Event::Text(e) if in_enbez => {
                enbez.push_str(&e.xml_content(XmlVersion::Implicit1_0));
            }
            Event::End(e) if e.name().as_ref() == "enbez" => {
                let value = enbez.trim();
                if !value.is_empty() {
                    inspection.enbez_values.push(value.to_owned());
                }
                in_enbez = false;
                enbez.clear();
            }
            Event::DocType(e) => {
                inspection.doctype = Some(e.xml_content(XmlVersion::Implicit1_0).into_owned());
            }
            Event::Eof => break,
            _ => {}
        }
    }

    if inspection.root_name.is_none() {
        bail!("XML document contains no root element");
    }

    Ok(inspection)
}

fn observe_element(start: &BytesStart<'_>, inspection: &mut Inspection) -> Result<()> {
    let name = start.name().as_ref().to_owned();
    *inspection.element_counts.entry(name.clone()).or_default() += 1;

    for attribute in start.attributes() {
        let attribute = attribute.context("parsing XML attribute")?;
        let key = format!("{}@{}", name, attribute.key.as_ref());
        *inspection.attribute_counts.entry(key).or_default() += 1;
    }

    Ok(())
}

pub fn print_inspection(inspection: &Inspection) {
    println!(
        "root: {}",
        inspection.root_name.as_deref().unwrap_or("<none>")
    );
    if !inspection.root_attributes.is_empty() {
        println!("root attributes:");
        for (name, value) in &inspection.root_attributes {
            println!("  {name} = {value}");
        }
    }
    if let Some(doctype) = &inspection.doctype {
        println!("doctype: {doctype}");
    }

    let total_elements: usize = inspection.element_counts.values().sum();
    println!("elements: {total_elements}");
    println!(
        "distinct element names: {}",
        inspection.element_counts.len()
    );
    println!(
        "norm elements: {}",
        inspection.element_counts.get("norm").copied().unwrap_or(0)
    );
    println!("enbez values: {}", inspection.enbez_values.len());

    println!("\nmost common elements:");
    let mut elements: Vec<_> = inspection.element_counts.iter().collect();
    elements.sort_by(|(name_a, count_a), (name_b, count_b)| {
        count_b.cmp(count_a).then_with(|| name_a.cmp(name_b))
    });
    for (name, count) in elements.into_iter().take(30) {
        println!("  {count:>6}  {name}");
    }

    println!("\nmost common attributes:");
    let mut attributes: Vec<_> = inspection.attribute_counts.iter().collect();
    attributes.sort_by(|(name_a, count_a), (name_b, count_b)| {
        count_b.cmp(count_a).then_with(|| name_a.cmp(name_b))
    });
    for (name, count) in attributes.into_iter().take(30) {
        println!("  {count:>6}  {name}");
    }

    println!("\nfirst enbez values:");
    for value in inspection.enbez_values.iter().take(30) {
        println!("  {value}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE dokumente SYSTEM "http://www.gesetze-im-internet.de/dtd/1.01/gii-norm.dtd">
<dokumente builddate="20260506174650" doknr="BJNR000010949">
  <norm builddate="20260506174650" doknr="BJNR000010949">
    <metadaten>
      <jurabk>GG</jurabk>
      <ausfertigung-datum manuell="ja">1949-05-23</ausfertigung-datum>
      <langue>Grundgesetz für die Bundesrepublik Deutschland</langue>
      <standangabe checked="ja">
        <standkommentar>Zuletzt geändert durch ...</standkommentar>
      </standangabe>
    </metadaten>
  </norm>
  <norm doknr="x"><metadaten><enbez>Präambel</enbez></metadaten></norm>
  <norm doknr="y"><metadaten><enbez>Art 1</enbez></metadaten></norm>
</dokumente>"#;

    #[test]
    fn extracts_dtd_reference() {
        assert_eq!(
            referenced_dtd_url(SAMPLE).unwrap(),
            "https://www.gesetze-im-internet.de/dtd/1.01/gii-norm.dtd"
        );
    }

    #[test]
    fn parses_document_metadata() {
        let metadata = parse_document_metadata(SAMPLE, "gg", "BJNR000010949.xml").unwrap();
        assert_eq!(metadata.doknr.as_deref(), Some("BJNR000010949"));
        assert_eq!(metadata.builddate.as_deref(), Some("20260506174650"));
        assert_eq!(metadata.jurabk.as_deref(), Some("GG"));
        assert_eq!(metadata.ausfertigung_datum.as_deref(), Some("1949-05-23"));
        assert_eq!(
            metadata.title.as_deref(),
            Some("Grundgesetz für die Bundesrepublik Deutschland")
        );
    }

    #[test]
    fn inspection_counts_structure() {
        let result = inspect(SAMPLE).unwrap();
        assert_eq!(result.root_name.as_deref(), Some("dokumente"));
        assert_eq!(result.element_counts.get("norm"), Some(&3));
        assert_eq!(result.enbez_values, vec!["Präambel", "Art 1"]);
    }
}
