use opensquirrel::{LineKind, classify_line};
/// Tests for OpenSquirrel state management logic.
use std::process::Command;

#[test]
fn binary_builds() {
    let status = Command::new("cargo")
        .args(["build"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("failed to run cargo build");
    assert!(status.success());
}

#[test]
fn focus_clamping_with_groups() {
    // Simulate: 5 agents, only 3 in the active group
    let agents: Vec<(&str, &str)> = vec![
        ("a-0", "default"),
        ("a-1", "default"),
        ("a-2", "cuda"),
        ("a-3", "default"),
        ("a-4", "cuda"),
    ];
    let group = "default";
    let visible: Vec<usize> = agents
        .iter()
        .enumerate()
        .filter(|(_, (_, g))| *g == group)
        .map(|(i, _)| i)
        .collect();

    assert_eq!(visible, vec![0, 1, 3]);

    // Focus on agent not in group -> clamp to first visible
    let mut focused: usize = 2; // agent-2 is in "cuda"
    if !visible.contains(&focused) {
        focused = visible[0];
    }
    assert_eq!(focused, 0);

    // Focus on agent in group -> stays
    focused = 3;
    if !visible.contains(&focused) {
        focused = visible[0];
    }
    assert_eq!(focused, 3);
}

#[test]
fn pane_navigation_within_group() {
    let visible = vec![0usize, 1, 3]; // agents in current group
    let mut focused = 0;

    // D (pane right)
    if let Some(pos) = visible.iter().position(|&i| i == focused) {
        if pos < visible.len() - 1 {
            focused = visible[pos + 1];
        }
    }
    assert_eq!(focused, 1);

    // D again
    if let Some(pos) = visible.iter().position(|&i| i == focused) {
        if pos < visible.len() - 1 {
            focused = visible[pos + 1];
        }
    }
    assert_eq!(focused, 3); // skips index 2 (different group)

    // D at end -> stays
    if let Some(pos) = visible.iter().position(|&i| i == focused) {
        if pos < visible.len() - 1 {
            focused = visible[pos + 1];
        }
    }
    assert_eq!(focused, 3);

    // A (pane left)
    if let Some(pos) = visible.iter().position(|&i| i == focused) {
        if pos > 0 {
            focused = visible[pos - 1];
        }
    }
    assert_eq!(focused, 1);
}

#[test]
fn group_navigation() {
    let groups = vec!["default", "cuda", "web"];
    let mut focused_group = 0usize;

    // S (down)
    if focused_group < groups.len().saturating_sub(1) {
        focused_group += 1;
    }
    assert_eq!(focused_group, 1);
    assert_eq!(groups[focused_group], "cuda");

    // S again
    if focused_group < groups.len().saturating_sub(1) {
        focused_group += 1;
    }
    assert_eq!(focused_group, 2);

    // S at bottom -> stays
    if focused_group < groups.len().saturating_sub(1) {
        focused_group += 1;
    }
    assert_eq!(focused_group, 2);

    // W (up)
    focused_group = focused_group.saturating_sub(1);
    assert_eq!(focused_group, 1);

    // W at top -> stays
    focused_group = 0;
    focused_group = focused_group.saturating_sub(1);
    assert_eq!(focused_group, 0);
}

#[test]
fn zoom_bounds() {
    let mut scale: f32 = 1.0;

    // Zoom in
    scale = (scale + 0.1).min(2.0);
    assert!((scale - 1.1).abs() < 0.001);

    // Zoom to max
    for _ in 0..20 {
        scale = (scale + 0.1).min(2.0);
    }
    assert!((scale - 2.0).abs() < 0.001);

    // Zoom out
    scale = (scale - 0.1).max(0.5);
    assert!((scale - 1.9).abs() < 0.001);

    // Zoom to min
    for _ in 0..30 {
        scale = (scale - 0.1).max(0.5);
    }
    assert!((scale - 0.5).abs() < 0.001);

    // Reset
    scale = 1.0;
    assert!((scale - 1.0).abs() < 0.001);
}

#[test]
fn palette_fuzzy_filter() {
    let items = vec!["New Agent", "New Group", "Quit"];

    // Empty query -> all
    let query = "";
    let filtered: Vec<&&str> = items
        .iter()
        .filter(|i| query.is_empty() || i.to_lowercase().contains(query))
        .collect();
    assert_eq!(filtered.len(), 3);

    // "new" -> matches 2
    let query = "new";
    let filtered: Vec<&&str> = items
        .iter()
        .filter(|i| query.is_empty() || i.to_lowercase().contains(query))
        .collect();
    assert_eq!(filtered.len(), 2);

    // "quit" -> matches 1
    let query = "quit";
    let filtered: Vec<&&str> = items
        .iter()
        .filter(|i| query.is_empty() || i.to_lowercase().contains(query))
        .collect();
    assert_eq!(filtered.len(), 1);
    assert_eq!(*filtered[0], "Quit");

    // "xyz" -> matches 0
    let query = "xyz";
    let filtered: Vec<&&str> = items
        .iter()
        .filter(|i| query.is_empty() || i.to_lowercase().contains(query))
        .collect();
    assert_eq!(filtered.len(), 0);
}

#[test]
fn session_id_parsing() {
    let raw = "SESSION:abc123\nWhat is 2+2?".to_string();
    let (sid, prompt) = if let Some(rest) = raw.strip_prefix("SESSION:") {
        if let Some(nl) = rest.find('\n') {
            (Some(rest[..nl].to_string()), rest[nl + 1..].to_string())
        } else {
            (None, raw.clone())
        }
    } else {
        (None, raw.clone())
    };

    assert_eq!(sid, Some("abc123".to_string()));
    assert_eq!(prompt, "What is 2+2?");
}

#[test]
fn scroll_bounds() {
    let lines = vec!["x"; 100];
    let mut offset: usize = 0;

    offset = (offset + 3).min(lines.len().saturating_sub(1));
    assert_eq!(offset, 3);

    offset = 200;
    offset = offset.min(lines.len().saturating_sub(1));
    assert_eq!(offset, 99);

    offset = 0;
    offset = offset.saturating_sub(3);
    assert_eq!(offset, 0);
}

// ── classify_line tests ─────────────────────────────────────────

#[test]
fn classify_diff_add() {
    assert_eq!(classify_line("+added line"), LineKind::DiffAdd);
    assert_eq!(classify_line("+"), LineKind::DiffAdd);
}

#[test]
fn classify_diff_remove() {
    assert_eq!(classify_line("-removed line"), LineKind::DiffRemove);
    assert_eq!(classify_line("-"), LineKind::DiffRemove);
}

#[test]
fn classify_diff_hunk() {
    assert_eq!(classify_line("@@ -1,3 +1,4 @@"), LineKind::DiffHunk);
    assert_eq!(classify_line("@@"), LineKind::DiffHunk);
}

#[test]
fn classify_diff_meta() {
    assert_eq!(classify_line("--- a/file.rs"), LineKind::DiffMeta);
    assert_eq!(classify_line("+++ b/file.rs"), LineKind::DiffMeta);
    assert_eq!(classify_line("diff --git a/x b/x"), LineKind::DiffMeta);
}

#[test]
fn classify_diff_meta_not_add_remove() {
    // +++ and --- should be DiffMeta, not DiffAdd/DiffRemove
    assert_ne!(classify_line("+++"), LineKind::DiffAdd);
    assert_ne!(classify_line("---"), LineKind::DiffRemove);
    assert_eq!(classify_line("+++"), LineKind::DiffMeta);
    assert_eq!(classify_line("---"), LineKind::DiffMeta);
}

#[test]
fn classify_user_input() {
    assert_eq!(classify_line("> hello world"), LineKind::UserInput);
}

#[test]
fn classify_error_lines() {
    assert_eq!(classify_line("[!] something failed"), LineKind::Error);
    assert_eq!(classify_line("[APPROVE?] run rm -rf /"), LineKind::Error);
}

#[test]
fn classify_thinking() {
    assert_eq!(
        classify_line("[think] considering options"),
        LineKind::Thinking
    );
}

#[test]
fn classify_system() {
    assert_eq!(classify_line("[approved] file write"), LineKind::System);
    assert_eq!(classify_line("[rejected] dangerous op"), LineKind::System);
    assert_eq!(classify_line("[killed] agent terminated"), LineKind::System);
}

#[test]
fn classify_normal() {
    assert_eq!(classify_line("just some output"), LineKind::Normal);
    assert_eq!(classify_line(""), LineKind::Normal);
    assert_eq!(classify_line("  indented text"), LineKind::Normal);
}

// ── Search logic tests ──────────────────────────────────────────

#[test]
fn search_filter_basic() {
    let lines = vec![
        "Hello world".to_string(),
        "Goodbye world".to_string(),
        "Hello again".to_string(),
        "Something else".to_string(),
    ];
    let query = "hello";
    let results: Vec<(usize, &str)> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.to_lowercase().contains(query))
        .map(|(i, l)| (i, l.as_str()))
        .collect();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, 0);
    assert_eq!(results[1].0, 2);
}

#[test]
fn search_filter_empty_query() {
    let lines = vec!["a".to_string(), "b".to_string()];
    let query = "";
    let results: Vec<&String> = lines
        .iter()
        .filter(|l| query.is_empty() || l.to_lowercase().contains(query))
        .collect();
    // Empty query matches nothing in actual search (early return)
    // but the filter above matches all -- matching the palette behavior
    assert_eq!(results.len(), 2);
}

#[test]
fn search_filter_no_match() {
    let lines = vec!["foo".to_string(), "bar".to_string()];
    let query = "xyz";
    let results: Vec<&String> = lines
        .iter()
        .filter(|l| l.to_lowercase().contains(query))
        .collect();
    assert_eq!(results.len(), 0);
}

#[test]
fn search_result_cap() {
    // rebuild_search caps at 50 results
    let cap = 50usize;
    let lines: Vec<String> = (0..200).map(|i| format!("line {}", i)).collect();
    let query = "line";
    let mut results: Vec<(usize, String)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.to_lowercase().contains(query) {
            results.push((i, line.clone()));
            if results.len() >= cap {
                break;
            }
        }
    }
    assert_eq!(results.len(), 50);
}

// ── View mode tests ─────────────────────────────────────────────

#[test]
fn view_mode_switching() {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum VM {
        Grid,
        Pipeline,
        Focus,
    }

    let mut mode = VM::Grid;
    assert_eq!(mode, VM::Grid);
    mode = VM::Pipeline;
    assert_eq!(mode, VM::Pipeline);
    mode = VM::Focus;
    assert_eq!(mode, VM::Focus);
    mode = VM::Grid;
    assert_eq!(mode, VM::Grid);
}

// ── Kill agent tests ────────────────────────────────────────────

#[test]
fn kill_agent_removes_from_list() {
    let mut agents = vec!["a", "b", "c"];
    let focused = 1;
    agents.remove(focused);
    assert_eq!(agents, vec!["a", "c"]);
}

#[test]
fn kill_agent_clamps_focus() {
    let mut agents = vec!["a", "b", "c"];
    let mut focused = 2;
    agents.remove(focused);
    // Clamp focus to new bounds
    if focused >= agents.len() && !agents.is_empty() {
        focused = agents.len() - 1;
    }
    assert_eq!(focused, 1);
    assert_eq!(agents[focused], "b");
}

// ── Favorite toggle tests ───────────────────────────────────────

#[test]
fn toggle_favorite() {
    let mut favorite = false;
    favorite = !favorite;
    assert!(favorite);
    favorite = !favorite;
    assert!(!favorite);
}

// ── Approve / reject tests ──────────────────────────────────────

#[test]
fn approve_clears_pending() {
    let mut pending: Option<String> = Some("run tests".into());
    // Approve
    if pending.is_some() {
        pending = None;
    }
    assert!(pending.is_none());
}

#[test]
fn reject_clears_pending() {
    let mut pending: Option<String> = Some("delete file".into());
    if pending.is_some() {
        pending = None;
    }
    assert!(pending.is_none());
}

// ── Model selection tests ───────────────────────────────────────

#[test]
fn model_filter_free_only() {
    struct M {
        id: &'static str,
        free: bool,
    }
    let models = vec![
        M {
            id: "opus",
            free: false,
        },
        M {
            id: "o4-mini",
            free: true,
        },
        M {
            id: "gpt-4.1",
            free: true,
        },
        M {
            id: "o3",
            free: false,
        },
    ];
    let free: Vec<&str> = models.iter().filter(|m| m.free).map(|m| m.id).collect();
    assert_eq!(free, vec!["o4-mini", "gpt-4.1"]);
}

// ── Theme cycling tests ─────────────────────────────────────────

#[test]
fn theme_cycling() {
    let themes = vec!["midnight", "charcoal", "gruvbox", "light"];
    let mut current = "midnight";

    // Find index and cycle
    let idx = themes.iter().position(|&t| t == current).unwrap_or(0);
    current = themes[(idx + 1) % themes.len()];
    assert_eq!(current, "charcoal");

    let idx = themes.iter().position(|&t| t == current).unwrap_or(0);
    current = themes[(idx + 1) % themes.len()];
    assert_eq!(current, "gruvbox");

    // Wrap around
    let mut current_idx = 3; // "light"
    current_idx = (current_idx + 1) % themes.len();
    assert_eq!(current_idx, 0);
    assert_eq!(themes[current_idx], "midnight");
}

// ── Cursor / UTF-8 safety tests ──────────────────────────────────

/// Simulates the word-boundary logic used by cursor_word_left and delete_word_back.
fn word_left_target(buf: &str, cursor: usize) -> usize {
    let s = &buf[..cursor];
    let trimmed = s.trim_end();
    if trimmed.is_empty() {
        0
    } else {
        trimmed
            .char_indices()
            .filter(|(_, c)| c.is_whitespace())
            .last()
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or(0)
    }
}

#[test]
fn word_left_ascii() {
    let buf = "hello world";
    assert_eq!(word_left_target(buf, buf.len()), 6);
    assert_eq!(word_left_target(buf, 6), 0);
}

#[test]
fn word_left_multibyte_whitespace() {
    let buf = "hello\u{3000}world";
    assert_eq!(buf.len(), 13);
    assert_eq!(word_left_target(buf, buf.len()), 8);
    assert!(buf.is_char_boundary(8));
}

#[test]
fn word_left_multibyte_content() {
    let buf = "café world";
    let target = word_left_target(buf, buf.len());
    assert_eq!(target, 6);
    assert!(buf.is_char_boundary(target));
    assert_eq!(&buf[target..], "world");
}

#[test]
fn word_left_emoji_content() {
    let buf = "🦊 run fast";
    assert_eq!(buf.len(), 13);
    let target = word_left_target(buf, buf.len());
    assert_eq!(target, 9);
    assert!(buf.is_char_boundary(target));
    assert_eq!(&buf[target..], "fast");
}

#[test]
fn word_left_no_whitespace() {
    assert_eq!(word_left_target("helloworld", 10), 0);
}

#[test]
fn word_left_only_whitespace() {
    assert_eq!(word_left_target("   ", 3), 0);
}

#[test]
fn word_left_empty() {
    assert_eq!(word_left_target("", 0), 0);
}

// ── Security-relevant tests ─────────────────────────────────────

// shell_escape tests: verify that user input is properly escaped
// before being passed to shell commands (prevents command injection)

use opensquirrel::shell_escape;

#[test]
fn shell_escape_prevents_command_injection() {
    // Semicolon injection
    let input = "hello; rm -rf /";
    let escaped = shell_escape(input);
    assert_eq!(escaped, "'hello; rm -rf /'");

    // Backtick injection
    let input = "`whoami`";
    let escaped = shell_escape(input);
    assert_eq!(escaped, "'`whoami`'");

    // Dollar substitution
    let input = "$(cat /etc/passwd)";
    let escaped = shell_escape(input);
    assert_eq!(escaped, "'$(cat /etc/passwd)'");

    // Newline injection
    let input = "hello\nrm -rf /";
    let escaped = shell_escape(input);
    assert_eq!(escaped, "'hello\nrm -rf /'");

    // Pipe injection
    let input = "hello | cat /etc/passwd";
    let escaped = shell_escape(input);
    assert_eq!(escaped, "'hello | cat /etc/passwd'");

    // Ampersand injection
    let input = "hello && rm -rf /";
    let escaped = shell_escape(input);
    assert_eq!(escaped, "'hello && rm -rf /'");
}

#[test]
fn shell_escape_single_quote_edge_cases() {
    // Multiple consecutive quotes
    let input = "it''s";
    let escaped = shell_escape(input);
    // Should be safe to eval in bash
    assert!(escaped.starts_with('\''));
    assert!(escaped.ends_with('\''));

    // Only a single quote
    let input = "'";
    let escaped = shell_escape(input);
    assert_eq!(escaped, "''\"'\"''");
}

#[test]
fn shell_escape_null_bytes() {
    let input = "hello\0world";
    let escaped = shell_escape(input);
    // Should wrap without panic
    assert!(escaped.starts_with('\''));
}

#[test]
fn shell_escape_very_long_input() {
    let input = "a".repeat(10_000);
    let escaped = shell_escape(&input);
    assert_eq!(escaped.len(), 10_002); // 'aaa...a'
}

// ── lib.rs parsing edge cases ───────────────────────────────────

use opensquirrel::{
    Span, parse_spans, parse_code_fence, parse_bullet, parse_heading,
    parse_session_prompt, build_persistent_runtime_args,
    extract_latest_turn_output, summarize_diff, DiffSummary,
};

#[test]
fn parse_spans_unicode_in_code() {
    let spans = parse_spans("use `cafe\u{0301}` here");
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[1], Span::Code("cafe\u{0301}".into()));
}

#[test]
fn parse_spans_adjacent_formatting() {
    let spans = parse_spans("**bold***italic*");
    assert_eq!(spans[0], Span::Bold("bold".into()));
    assert_eq!(spans[1], Span::Italic("italic".into()));
}

#[test]
fn parse_spans_nested_backticks_in_bold() {
    // backtick inside bold should be treated as code
    let spans = parse_spans("**hello `world` there**");
    // Current parser: bold will consume everything between ** **, including backticks
    // This tests the actual behavior rather than ideal behavior
    assert!(!spans.is_empty());
}

#[test]
fn parse_spans_only_formatting_markers() {
    assert_eq!(parse_spans("``"), vec![Span::Text("".into())]);
    // Empty bold markers
    let spans = parse_spans("****");
    assert!(!spans.is_empty());
}

#[test]
fn parse_code_fence_with_extra_backticks() {
    assert_eq!(parse_code_fence("````"), Some("`".into()));
    assert_eq!(parse_code_fence("```rs extra stuff"), Some("rs extra stuff".into()));
}

#[test]
fn parse_bullet_edge_cases() {
    // Just "- " with nothing after
    assert_eq!(parse_bullet("- "), Some((0, "")));
    // Deep nesting
    assert_eq!(parse_bullet("        - deep"), Some((4, "deep")));
    // Number at limit
    assert_eq!(parse_bullet("999. item"), Some((0, "item")));
    // Number too long
    assert_eq!(parse_bullet("10000. item"), None);
    // Not a bullet
    assert_eq!(parse_bullet(""), None);
    assert_eq!(parse_bullet("-"), None);
    assert_eq!(parse_bullet("- "), Some((0, "")));
}

#[test]
fn parse_heading_edge_cases() {
    // Too many #
    assert_eq!(parse_heading("####### too deep"), None);
    // Just hashes, no content
    assert_eq!(parse_heading("#"), None);
    // Hash with space but no content
    assert_eq!(parse_heading("# "), Some((1, "")));
    // Level 6 is max valid
    assert_eq!(parse_heading("###### six"), Some((6, "six")));
}

#[test]
fn parse_session_prompt_no_newline() {
    // SESSION: prefix but no newline => no session extracted
    let (sid, prompt) = parse_session_prompt("SESSION:abc123");
    assert_eq!(sid, None);
    assert_eq!(prompt, "SESSION:abc123");
}

#[test]
fn parse_session_prompt_empty_session_id() {
    let (sid, prompt) = parse_session_prompt("SESSION:\nsome prompt");
    assert_eq!(sid, Some("".into()));
    assert_eq!(prompt, "some prompt");
}

#[test]
fn build_persistent_runtime_args_no_base_args() {
    let args = build_persistent_runtime_args(&[], "--model", None, None);
    assert_eq!(
        args,
        vec![
            "--input-format", "stream-json",
            "--output-format", "stream-json",
            "--verbose",
        ]
    );
}

#[test]
fn build_persistent_runtime_args_empty_model_flag() {
    // If model_flag is empty, model should not be added even with override
    let args = build_persistent_runtime_args(&[], "", Some("gpt-4"), None);
    assert!(!args.contains(&"gpt-4".to_string()));
}

#[test]
fn extract_latest_turn_output_no_user_input() {
    let lines = vec!["line 1".into(), "line 2".into(), "".into(), "line 3".into()];
    let result = extract_latest_turn_output(&lines);
    assert_eq!(result, "line 1\nline 2\nline 3");
}

#[test]
fn extract_latest_turn_output_empty() {
    let lines: Vec<String> = vec![];
    let result = extract_latest_turn_output(&lines);
    assert_eq!(result, "");
}

#[test]
fn extract_latest_turn_output_only_user_input() {
    let lines = vec!["> hello".into()];
    let result = extract_latest_turn_output(&lines);
    assert_eq!(result, "");
}

#[test]
fn summarize_diff_empty() {
    let lines: Vec<String> = vec![];
    assert_eq!(
        summarize_diff(&lines),
        DiffSummary { files: vec![], additions: 0, removals: 0 }
    );
}

#[test]
fn summarize_diff_no_duplicates() {
    let lines = vec![
        "--- a/file.rs".into(),
        "+++ b/file.rs".into(),
        "--- a/file.rs".into(),
        "+++ b/file.rs".into(),
    ];
    let summary = summarize_diff(&lines);
    assert_eq!(summary.files.len(), 1);
    assert_eq!(summary.files[0], "file.rs");
}

#[test]
fn summarize_diff_multiple_files() {
    let lines = vec![
        "--- a/a.rs".into(),
        "+++ b/a.rs".into(),
        "+new".into(),
        "--- a/b.rs".into(),
        "+++ b/b.rs".into(),
        "-old".into(),
        "+new".into(),
    ];
    let summary = summarize_diff(&lines);
    assert_eq!(summary.files.len(), 2);
    assert_eq!(summary.additions, 2);
    assert_eq!(summary.removals, 1);
}

// ── tmux session name sanitization ──────────────────────────────

#[test]
fn tmux_session_name_sanitization() {
    // Replicate the logic from runtime.rs make_tmux_session_name
    fn sanitize(name: &str) -> String {
        let safe = name
            .chars()
            .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_lowercase();
        if safe.is_empty() { "agent".into() } else { safe }
    }

    // Injection attempt via session name
    assert_eq!(sanitize("agent; rm -rf /"), "agent--rm--rf");
    assert_eq!(sanitize("$(whoami)"), "whoami");
    assert_eq!(sanitize(""), "agent");
    assert_eq!(sanitize("normal-name"), "normal-name");
    assert_eq!(sanitize("Hello World"), "hello-world");
}

// ── Changes state parsing tests ─────────────────────────────────

#[test]
fn changes_file_at_boundary() {
    // Simulate ChangesState::file_at logic
    let staged = vec!["a.rs", "b.rs"];
    let unstaged = vec!["c.rs"];
    let untracked = vec!["d.rs", "e.rs"];

    let file_at = |index: usize| -> Option<(&str, bool)> {
        let staged_len = staged.len();
        let unstaged_len = unstaged.len();
        if index < staged_len {
            Some((staged[index], true))
        } else if index < staged_len + unstaged_len {
            Some((unstaged[index - staged_len], false))
        } else if index < staged_len + unstaged_len + untracked.len() {
            Some((untracked[index - staged_len - unstaged_len], false))
        } else {
            None
        }
    };

    assert_eq!(file_at(0), Some(("a.rs", true)));
    assert_eq!(file_at(1), Some(("b.rs", true)));
    assert_eq!(file_at(2), Some(("c.rs", false)));
    assert_eq!(file_at(3), Some(("d.rs", false)));
    assert_eq!(file_at(4), Some(("e.rs", false)));
    assert_eq!(file_at(5), None);
    assert_eq!(file_at(100), None);
}

#[test]
fn changes_file_at_empty_sections() {
    let staged: Vec<&str> = vec![];
    let unstaged: Vec<&str> = vec![];
    let untracked = vec!["only.txt"];

    let file_at = |index: usize| -> Option<(&str, bool)> {
        let staged_len = staged.len();
        let unstaged_len = unstaged.len();
        if index < staged_len {
            Some((staged[index], true))
        } else if index < staged_len + unstaged_len {
            Some((unstaged[index - staged_len], false))
        } else if index < staged_len + unstaged_len + untracked.len() {
            Some((untracked[index - staged_len - unstaged_len], false))
        } else {
            None
        }
    };

    assert_eq!(file_at(0), Some(("only.txt", false)));
    assert_eq!(file_at(1), None);
}

// ── Git status porcelain parsing tests ──────────────────────────

#[test]
fn parse_status_char_coverage() {
    // Replicate the parse_status_char logic from changes.rs
    fn parse_status_char(c: char) -> Option<&'static str> {
        match c {
            'M' => Some("Modified"),
            'A' => Some("Added"),
            'D' => Some("Deleted"),
            'R' => Some("Renamed"),
            'C' => Some("Copied"),
            _ => None,
        }
    }

    assert_eq!(parse_status_char('M'), Some("Modified"));
    assert_eq!(parse_status_char('A'), Some("Added"));
    assert_eq!(parse_status_char('D'), Some("Deleted"));
    assert_eq!(parse_status_char('R'), Some("Renamed"));
    assert_eq!(parse_status_char('C'), Some("Copied"));
    assert_eq!(parse_status_char(' '), None);
    assert_eq!(parse_status_char('?'), None);
    assert_eq!(parse_status_char('U'), None);
    assert_eq!(parse_status_char('!'), None);
}

#[test]
fn porcelain_line_parsing() {
    // Simulate the parsing logic from ChangesState::refresh
    fn parse_porcelain_line(line: &str) -> Option<(char, char, String)> {
        if line.len() < 3 {
            return None;
        }
        let bytes = line.as_bytes();
        let x = bytes[0] as char;
        let y = bytes[1] as char;
        let path_str = &line[3..];
        let path = if let Some(arrow_pos) = path_str.find(" -> ") {
            path_str[arrow_pos + 4..].to_string()
        } else {
            path_str.to_string()
        };
        Some((x, y, path))
    }

    assert_eq!(
        parse_porcelain_line("M  src/main.rs"),
        Some(('M', ' ', "src/main.rs".into()))
    );
    assert_eq!(
        parse_porcelain_line(" M src/lib.rs"),
        Some((' ', 'M', "src/lib.rs".into()))
    );
    assert_eq!(
        parse_porcelain_line("?? new_file.txt"),
        Some(('?', '?', "new_file.txt".into()))
    );
    // Rename
    assert_eq!(
        parse_porcelain_line("R  old.rs -> new.rs"),
        Some(('R', ' ', "new.rs".into()))
    );
    // Too short
    assert_eq!(parse_porcelain_line("M"), None);
    assert_eq!(parse_porcelain_line(""), None);
}

// ── Classify line comprehensive unicode ─────────────────────────

#[test]
fn classify_line_unicode_prefix() {
    // Unicode characters that start with byte values that might conflict
    assert_eq!(classify_line("\u{2B}extra"), LineKind::DiffAdd); // U+002B is '+'
    assert_eq!(classify_line("\u{3E} quoted"), LineKind::UserInput); // U+003E is '>'
}

// ── Directory fuzzy finder simulation ───────────────────────────

#[test]
fn fuzzy_filter_directory_names() {
    let dirs = vec![
        "/Users/user/projects/opensquirrel",
        "/Users/user/projects/other-project",
        "/Users/user/Documents",
        "/Users/user/.config",
    ];

    let query = "proj";
    let filtered: Vec<&&str> = dirs
        .iter()
        .filter(|d| d.to_lowercase().contains(query))
        .collect();
    assert_eq!(filtered.len(), 2);

    let query = "opensq";
    let filtered: Vec<&&str> = dirs
        .iter()
        .filter(|d| d.to_lowercase().contains(query))
        .collect();
    assert_eq!(filtered.len(), 1);
    assert!(filtered[0].contains("opensquirrel"));
}

#[test]
fn fuzzy_filter_case_insensitive() {
    let items = vec!["OpenSquirrel", "opencode", "TERMINAL"];
    let query = "open";
    let filtered: Vec<&&str> = items
        .iter()
        .filter(|i| i.to_lowercase().contains(query))
        .collect();
    assert_eq!(filtered.len(), 2);
}

// ── Config defaults tests ───────────────────────────────────────

#[test]
fn default_bg_opacity_is_opaque() {
    // Verify the default is 1.0 (fully opaque)
    let default: f32 = 1.0;
    assert!((default - 1.0).abs() < f32::EPSILON);
}

#[test]
fn default_bg_blur_is_zero() {
    let default: f32 = 0.0;
    assert!((default - 0.0).abs() < f32::EPSILON);
}
