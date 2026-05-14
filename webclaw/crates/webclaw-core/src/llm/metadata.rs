/// Metadata header building for LLM-optimized output.
///
/// Produces `> ` prefixed lines with URL, title, author, etc.
/// Omits empty/zero fields to minimize token waste.
use crate::types::ExtractionResult;

pub(crate) fn build_metadata_header(
    out: &mut String,
    result: &ExtractionResult,
    url: Option<&str>,
) {
    let meta = &result.metadata;

    // URL: prefer explicit arg, fall back to metadata
    let effective_url = url.or(meta.url.as_deref());
    if let Some(u) = effective_url {
        out.push_str(&format!("> URL: {u}\n"));
    }
    if let Some(t) = &meta.title
        && !t.is_empty()
    {
        out.push_str(&format!("> Title: {t}\n"));
    }
    if let Some(d) = &meta.description
        && !d.is_empty()
    {
        out.push_str(&format!("> Description: {d}\n"));
    }
    if let Some(a) = &meta.author
        && !a.is_empty()
    {
        out.push_str(&format!("> Author: {a}\n"));
    }
    if let Some(d) = &meta.published_date
        && !d.is_empty()
    {
        out.push_str(&format!("> Published: {d}\n"));
    }
    if let Some(l) = &meta.language
        && !l.is_empty()
    {
        out.push_str(&format!("> Language: {l}\n"));
    }
    if meta.word_count > 0 {
        out.push_str(&format!("> Word count: {}\n", meta.word_count));
    }
}
