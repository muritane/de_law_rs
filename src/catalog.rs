use anyhow::{Context, Result, bail};
use quick_xml::{Reader, XmlVersion, events::Event};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogItem {
    pub title: String,
    pub link: String,
}

pub fn find_law(xml: &str, law_slug: &str) -> Result<CatalogItem> {
    let mut reader = Reader::from_str(xml);
    let mut current_tag: Option<String> = None;
    let mut in_item = false;
    let mut title = String::new();
    let mut link = String::new();

    loop {
        match reader.read_event().context("parsing gii-toc.xml")? {
            Event::Start(e) => {
                let name = e.name().as_ref().to_owned();
                if name == "item" {
                    in_item = true;
                    title.clear();
                    link.clear();
                } else if in_item && (name == "title" || name == "link") {
                    current_tag = Some(name);
                }
            }
            Event::Text(e) if in_item => {
                let text = e.xml_content(XmlVersion::Implicit1_0);
                match current_tag.as_deref() {
                    Some("title") => title.push_str(&text),
                    Some("link") => link.push_str(&text),
                    _ => {}
                }
            }
            Event::End(e) => {
                let name = e.name().as_ref().to_owned();
                if name == "title" || name == "link" {
                    current_tag = None;
                } else if name == "item" {
                    if link_matches_slug(&link, law_slug) {
                        return Ok(CatalogItem {
                            title: title.trim().to_owned(),
                            link: prefer_https(link.trim()),
                        });
                    }
                    in_item = false;
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    bail!("law slug '{law_slug}' was not found in the Gesetze-im-Internet catalog")
}

fn link_matches_slug(link: &str, law_slug: &str) -> bool {
    let normalized = link.trim_end_matches('/');
    let suffix = format!("/{law_slug}/xml.zip");
    normalized
        .to_ascii_lowercase()
        .ends_with(&suffix.to_ascii_lowercase())
}

pub fn prefer_https(url: &str) -> String {
    match url.strip_prefix("http://") {
        Some(rest) => format!("https://{rest}"),
        None => url.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_gg_and_upgrades_http_to_https() {
        let xml = r#"
            <items>
              <item>
                <title>Grundgesetz für die Bundesrepublik Deutschland</title>
                <link>http://www.gesetze-im-internet.de/gg/xml.zip</link>
              </item>
            </items>
        "#;

        let item = find_law(xml, "gg").unwrap();
        assert_eq!(item.title, "Grundgesetz für die Bundesrepublik Deutschland");
        assert_eq!(item.link, "https://www.gesetze-im-internet.de/gg/xml.zip");
    }
}
