use crate::lexer::{Token, TokenType};

pub fn report(line: u32, location: &str, message: &str, had_err: &mut bool) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    *had_err = true;
}

pub fn lex_error(line: u32, message: &str, had_err: &mut bool) {
    report(line, "", message, had_err);
}

pub fn parse_error(token: &Token, msg: &str, had_err: &mut bool) {
    if *token.get_token_type() == TokenType::Eof {
        report(token.get_line(), " at end", msg, had_err);
    } else {
        report(
            token.get_line(),
            &format!("at '{}'", &token.get_lexeme()),
            msg,
            had_err,
        )
    }
}

pub fn runtime_error(token: &Token, msg: &str, had_runtime_err: &mut bool) {
    report(token.get_line(), "", msg, had_runtime_err);
}
