use crate::file_utils::find_matching_files;
use crate::path_utils::scan_path_executables;
use crate::trie::Trie;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::cell::RefCell;

pub struct ShellHelper {
    pub trie: Trie,
    path_executable_loaded: bool,
    last_completion_context: RefCell<Option<(String, Vec<String>)>>,
}

impl ShellHelper {
    pub fn new() -> Self {
        Self {
            trie: Trie::new(),
            path_executable_loaded: false,
            last_completion_context: RefCell::new(None),
        }
    }

    pub(crate) fn load_path_executables(&mut self) {
        if self.path_executable_loaded {
            return;
        }

        let executables = scan_path_executables();
        for exe in executables {
            self.trie.insert(&exe);
        }

        self.path_executable_loaded = true;
    }

    fn longest_common_prefix(strings: &[String]) -> String {
        if strings.is_empty() {
            return String::new();
        }

        if strings.len() == 1 {
            return strings[0].clone();
        }

        let first = &strings[0];
        let mut prefix_len = first.len();

        for s in &strings[1..] {
            prefix_len = prefix_len.min(s.len());

            for i in 0..prefix_len {
                if first.chars().nth(i) != s.chars().nth(i) {
                    prefix_len = i;
                    break;
                }
            }

            if prefix_len == 0 {
                break;
            }
        }

        first.chars().take(prefix_len).collect()
    }
}

impl Helper for ShellHelper {}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let line = &line[..pos];

        // Check if we're completing a file argument (after the first space)
        let has_space = line.contains(' ');

        if has_space {
            // File completion mode
            return self.complete_files(line);
        }

        // Command completion mode
        self.complete_commands(line)
    }
}

impl ShellHelper {
    /// Complete file arguments (triggered when input contains spaces)
    fn complete_files(&self, line: &str) -> rustyline::Result<(usize, Vec<Pair>)> {
        let word = line.split_whitespace().last().unwrap_or("");
        self.complete_with(line, word, find_matching_files(word))
    }

    /// Complete command names (triggered when input has no spaces yet)
    fn complete_commands(&self, line: &str) -> rustyline::Result<(usize, Vec<Pair>)> {
        let word = line.split_whitespace().last().unwrap_or("");
        self.complete_with(line, word, self.trie.find_completions(word))
    }

    /// Core completion logic shared by file and command completion.
    ///
    /// `line`        – the full input up to the cursor
    /// `word`        – the prefix being completed (last whitespace-delimited token)
    /// `completions` – candidate strings that already match `word`
    fn complete_with(
        &self,
        line: &str,
        word: &str,
        mut completions: Vec<String>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        completions.sort();

        if completions.len() > 1 {
            let common_prefix = Self::longest_common_prefix(&completions);

            // Extend to the longest common prefix without ambiguity
            if common_prefix.len() > word.len() {
                self.last_completion_context.borrow_mut().take();
                return Ok((
                    line.len() - word.len(),
                    vec![Pair {
                        display: common_prefix.clone(),
                        replacement: common_prefix,
                    }],
                ));
            }

            let mut last_context = self.last_completion_context.borrow_mut();
            let is_repeated = last_context
                .as_ref()
                .map(|(prev_word, _)| prev_word == word)
                .unwrap_or(false);

            return if is_repeated {
                // Second TAB press – print all candidates and redraw prompt
                println!();
                println!("{}  ", completions.join("  "));
                print!("$ {}", line);
                std::io::Write::flush(&mut std::io::stdout()).ok();
                *last_context = None;
                Ok((0, vec![]))
            } else {
                // First TAB press – ring bell and remember context
                print!("\x07");
                std::io::Write::flush(&mut std::io::stdout()).ok();
                *last_context = Some((word.to_string(), completions));
                Ok((0, vec![]))
            };
        }

        // Zero or one match – complete directly (trailing space lets the user
        // continue typing the next argument immediately)
        self.last_completion_context.borrow_mut().take();
        let candidates = completions
            .into_iter()
            .map(|c| Pair {
                replacement: format!("{} ", c),
                display: c,
            })
            .collect();

        Ok((line.len() - word.len(), candidates))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}
