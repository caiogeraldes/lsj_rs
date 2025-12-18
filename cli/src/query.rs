use crate::{Entries, Entry};
pub(crate) fn query(value: String, entries: &Entries) -> Option<&Entry> {
    entries.0.get(&value)
}
pub(crate) fn query_no_diacritics(value: String, entries: &Entries) -> Option<&Entry> {
    let value = value
        .replace("\\", "")
        .replace("/", "")
        .replace(")", "")
        .replace("(", "")
        .replace("'", "")
        .replace("*", "")
        .replace("-", "")
        .replace("[", "")
        .replace("]", "")
        .replace("<", "")
        .replace(">", "")
        .replace("|", "")
        .replace("-", "")
        .replace("=", "");
    query(value, entries)
}
