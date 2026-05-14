//! Post-processing for LLM responses.
//! Strips chain-of-thought reasoning tags that models like qwen3 emit.
//! Applied to every provider response so callers never see internal reasoning.

/// Strip `<think>...</think>` blocks from LLM responses.
/// Models like qwen3 wrap internal chain-of-thought reasoning in these tags.
/// Handles multiline content, multiple blocks, and partial/malformed tags.
pub fn strip_thinking_tags(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut remaining = text;

    while let Some(start) = remaining.find("<think>") {
        // Keep everything before the opening tag
        result.push_str(&remaining[..start]);

        // Find the matching closing tag
        let after_open = &remaining[start + 7..]; // len("<think>") == 7
        if let Some(end) = after_open.find("</think>") {
            remaining = &after_open[end + 8..]; // len("</think>") == 8
        } else {
            // Unclosed <think> — discard everything after it (the model is still "thinking")
            remaining = "";
        }
    }

    result.push_str(remaining);

    // Clean up: leftover </think> or /think fragments from partial responses
    let result = result.replace("</think>", "");
    let result = result.replace("/think", "");

    // Collapse leading whitespace left behind after stripping
    let trimmed = result.trim();
    if trimmed.is_empty() {
        String::new()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_simple_thinking_block() {
        let input = "<think>reasoning here</think>actual response";
        assert_eq!(strip_thinking_tags(input), "actual response");
    }

    #[test]
    fn strips_multiline_thinking() {
        let input = "<think>\nlong\nthinking\nprocess\n</think>\nclean output";
        assert_eq!(strip_thinking_tags(input), "clean output");
    }

    #[test]
    fn passthrough_no_tags() {
        let input = "no thinking tags here";
        assert_eq!(strip_thinking_tags(input), "no thinking tags here");
    }

    #[test]
    fn strips_partial_think_at_end() {
        let input = "some text /think";
        assert_eq!(strip_thinking_tags(input), "some text");
    }

    #[test]
    fn strips_orphan_closing_tag() {
        let input = "some text</think> more text";
        assert_eq!(strip_thinking_tags(input), "some text more text");
    }

    #[test]
    fn strips_multiple_thinking_blocks() {
        let input = "<think>first</think>hello <think>second</think>world";
        assert_eq!(strip_thinking_tags(input), "hello world");
    }

    #[test]
    fn handles_unclosed_think_tag() {
        // Model started thinking and never closed — discard everything after <think>
        let input = "good content<think>still reasoning...";
        assert_eq!(strip_thinking_tags(input), "good content");
    }

    #[test]
    fn handles_empty_thinking_block() {
        let input = "<think></think>content";
        assert_eq!(strip_thinking_tags(input), "content");
    }

    #[test]
    fn handles_only_thinking() {
        let input = "<think>just thinking, no output</think>";
        assert_eq!(strip_thinking_tags(input), "");
    }

    #[test]
    fn preserves_json_content() {
        let input = "<think>let me analyze...</think>{\"key\": \"value\", \"count\": 42}";
        assert_eq!(
            strip_thinking_tags(input),
            "{\"key\": \"value\", \"count\": 42}"
        );
    }

    #[test]
    fn real_world_extract_leak() {
        // Actual bug: qwen3 leaked "/think" into JSON values
        let input = "<think>analyzing the page</think>{\"learn_more\": \"Learn more\"}";
        assert_eq!(
            strip_thinking_tags(input),
            "{\"learn_more\": \"Learn more\"}"
        );
    }

    #[test]
    fn thinking_with_newlines_before_json() {
        let input = "<think>\nstep 1\nstep 2\n</think>\n\n{\"result\": true}";
        assert_eq!(strip_thinking_tags(input), "{\"result\": true}");
    }
}
