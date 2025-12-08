use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Header {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "@orig_id")]
    orig_id: Option<String>,
    #[serde(rename = "@type")]
    etype: Option<String>,
    #[serde(rename = "@opt")]
    opt: Option<String>,
    head: Head,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Head {
    #[serde(rename = "@extent")]
    extent: Option<String>,
    #[serde(rename = "@orth_orig")]
    original_orthography: Option<String>,
    #[serde(rename = "#text")]
    value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UnflattenEntry {
    header: Header,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub key: String,
    pub entry: String,
    pub body: String,
}

impl From<UnflattenEntry> for Entry {
    fn from(value: UnflattenEntry) -> Self {
        let entry = match value.header.head.value {
            Some(value) => value,
            None => betacode::converter::convert(&value.header.key),
        };

        Entry {
            key: value.header.key,
            entry: entry.trim().to_string(),
            body: value.body.trim().to_string(),
        }
    }
}

impl TryFrom<&str> for UnflattenEntry {
    type Error = crate::error::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.trim().split("</head>").collect();
        let header: Header = match serde_xml_rs::from_str(&format!("{0}</head></div2>", split[0])) {
            Ok(value) => value,
            Err(_) => {
                return Err(crate::error::Error::ParsingError(split[0].into()));
            }
        };
        let re = Regex::new("<.*?>").unwrap();
        let dirty_body = split[1];
        let clean_body = re.replace_all(dirty_body, "");
        let body = clean_body
            .replace("\n", "")
            .replace("  ", "")
            .replace("( ", "(")
            .replace(" )", ")")
            .replace(";", "; ")
            .replace(",", ", ")
            .trim()
            .to_string();
        Ok(UnflattenEntry { header, body })
    }
}

pub fn parse_tei(tei: &str) -> Vec<Entry> {
    let mut entries = vec![];
    let value = tei.trim().replace("\n", "");
    let re = Regex::new(".*<text>").unwrap();
    let removed_begining = re.replace(&value, "");
    let re = Regex::new("</text>.*").unwrap();
    let removed_ending = re.replace(&removed_begining, "");
    let re = Regex::new("<div2.*?</div2>").unwrap();
    let matches = re.find_iter(&removed_ending);
    for m in matches {
        match TryInto::<UnflattenEntry>::try_into(m.as_str()) {
            Ok(entry) => entries.push(entry.into()),
            Err(_e) => {
                warn!("{}", m.as_str());
            }
        }
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn from_tei() {
        let tei = fs::read_to_string("../assets/lsjlogeion/greatscott02.xml").unwrap();
        let entries = parse_tei(&tei);
        for entry in entries {
            if entry.entry.is_empty() {
                dbg!(entry);
            }
        }
    }
}
