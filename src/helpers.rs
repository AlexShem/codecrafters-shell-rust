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
        let word = line.split_whitespace().last().unwrap_or("");

        let mut completions = self.trie.find_completions(word);
        completions.sort();

        // Handle multiple completions
        if completions.len() > 1 {
            let common_prefix = Self::longest_common_prefix(&completions);

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

            // Check if this is a repeated TAB press for the same prefix
            let is_repeated = last_context
                .as_ref()
                .map(|(prev_word, _)| prev_word == word)
                .unwrap_or(false);

            return if is_repeated {
                // Second TAB press - show all completions
                println!();
                println!("{}  ", completions.join("  "));
                print!("$ {}", line);
                std::io::Write::flush(&mut std::io::stdout()).ok();

                *last_context = None;

                // Return empty to prevent rustyline from doing anything
                Ok((0, vec![]))
            } else {
                // First TAB press - ring bell and store context
                print!("\x07");
                std::io::Write::flush(&mut std::io::stdout()).ok();

                *last_context = Some((word.to_string(), completions.clone()));

                Ok((0, vec![]))
            };
        }

        // Single completion or no completions - proceed normally
        self.last_completion_context.borrow_mut().take();

        let candidates = completions
            .into_iter()
            .map(|completion| {
                let replacement = format!("{} ", completion);
                Pair {
                    display: completion,
                    replacement,
                }
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
