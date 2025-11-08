use crate::path_utils::scan_path_executables;
use crate::trie::Trie;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

pub struct ShellHelper {
    pub trie: Trie,
    path_executable_loaded: bool,
}

impl ShellHelper {
    pub fn new() -> Self {
        Self {
            trie: Trie::new(),
            path_executable_loaded: false,
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

        let completions = self.trie.find_completions(word);

        let candidate = completions
            .into_iter()
            .map(|completion| {
                let replacement = format!("{} ", completion);
                Pair {
                    display: completion,
                    replacement,
                }
            })
            .collect();

        Ok((line.len() - word.len(), candidate))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}
