mod helper;

use helper::{CommandHinter, ReplHelper};
use rustyline::{
    error::ReadlineError, highlight::MatchingBracketHighlighter,
    validate::MatchingBracketValidator, CompletionType, Config, EditMode, Editor,
};
use tre::{
    diagnostic::{emit, FileId, Files},
    syntax::Parser,
    Result,
};

const PROMPT: &str = ">> ";
const COMMAND_PREFIX: &str = ":";

#[allow(unused)]
pub struct Repl<'s> {
    editor: Editor<ReplHelper>,
    prefix: &'s str,
    prompt: &'s str,
    code: String,
    count: u32,
    files: Files,
}

impl<'s> Repl<'s> {
    pub fn new() -> Self {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();
        let helper = ReplHelper {
            highlighter: MatchingBracketHighlighter::new(),
            validator: MatchingBracketValidator::new(),
            hinter: CommandHinter,
        };
        let mut editor = Editor::with_config(config);
        editor.set_helper(Some(helper));
        // TODO: Set some more key binds here. Definitely hist up / down
        // and add Vim Mode support.

        Self {
            editor,
            prefix: COMMAND_PREFIX,
            prompt: PROMPT,
            code: String::new(),
            count: 0,
            files: Files::new(),
        }
    }

    pub fn run(&mut self) -> rustyline::Result<()> {
        loop {
            let line = self.editor.readline(self.prompt);
            match line {
                Ok(line) => self.process_line(line),
                // Ctrl + C will skip the abort the current line
                // and asks for new input
                Err(ReadlineError::Interrupted) => continue,
                // Ctrl + D will exit the repl
                Err(ReadlineError::Eof) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }

    fn process_line(&mut self, line: String) {
        self.count += 1;
        match if line.starts_with(self.prefix) {
            todo!()
        } else {
            self.execute_code(line)
        } {
            Ok(_) => {}
            Err(d) => emit(&self.files, &d),
        }
    }

    fn execute_code(&mut self, code: String) -> Result<()> {
        let file = self.files.add(format!("${}", self.count), code);
        let mut parser = Parser::new(&self.files, file);
        let stmt = parser.next_stmt()?;
        println!("${} => {:#?}", self.count, stmt.into_inner());
        Ok(())
    }
}
