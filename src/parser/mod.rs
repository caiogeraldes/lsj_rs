use serde::{Deserialize, Serialize, de};

#[derive(include_assets::AssetEnum)]
#[archive(base_path = "assets/lsjlogeion")]
pub(crate) enum Asset {
    #[asset(path = "greatscott02.xml")]
    A,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Header {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "@orig_id")]
    orig_id: String,
    #[serde(rename = "@type")]
    etype: String,
    #[serde(rename = "@opt")]
    opt: String,
    head: Head,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Head {
    #[serde(rename = "@extent")]
    extent: String,
    #[serde(rename = "@orth_orig")]
    original_orthography: String,
    #[serde(rename = "#text")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UnflattenEntry {
    header: Header,
    body: String,
}

impl TryFrom<&str> for UnflattenEntry {
    type Error = crate::error::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.trim().split("</head>").collect();
        let header: Header =
            serde_xml_rs::from_str(&format!("{0}</head></div2>", split[0])).unwrap();
        let re = regex::Regex::new("<.*?>").unwrap();
        let dirty_body = split[1];
        let clean_body = re.replace_all(&dirty_body, "");
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

// fn deserialize_entry<'de, D>(deserializer: D) -> Result<Entry, D::Error>
// where
//     D: de::Deserializer<'de>,
// {
//     let s: &str = de::Deserialize::deserialize(deserializer)?;
//     match s.try_into() {
//         Ok(entry) => return Ok(entry),
//         Err(e) => return Err(de::Error::custom(format!("{e:?}"))),
//     }
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let value = r#"<div2 id="crossa)a/baktoi" orig_id="n7" key="a)a/baktoi" type="gloss" opt="n" > <head extent="full" lang="greek" opt="n" orth_orig="ἀάβακτοι·" > ἀάβακτοι </head> <foreign lang="greek">ἀβλαβεῖς,</foreign> <author>Hsch.</author>; cf. <orth extent="full" lang="greek" opt="n">ἀάβηκτον·</orth> <foreign lang="greek">μέλαν, ἀβλαβές,</foreign> <title>Et.Gud.</title> ( <sense id="n7.0" n="A" level="1" opt="n"> <cit> <quote lang="greek">-βυκτον</quote> <author>Cyr.</author> </cit>) </sense> </div2>"#;
        let entry: UnflattenEntry = value.try_into().unwrap();
        dbg!(entry);
    }
}
