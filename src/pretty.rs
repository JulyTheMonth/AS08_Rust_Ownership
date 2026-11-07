use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    // TODO: Implement pretty printing for lambda calculus terms.
    match term {
        Term::Var(var) => format!("{var}"),
        Term::Abs(param, body) => format!("λ{param}. {}", pretty_print(body)),
        Term::App(t1, t2) => {
            let left = match t1.as_ref() {
                Term::Abs(..) => format!("({})", pretty_print(t1)),
                _ => pretty_print(t1),
            };
            let right = match t2.as_ref() {
                Term::Var(_) => pretty_print(t2),
                _ => format!("({})", pretty_print(t2)),
            };
            format!("{left} {right}")
        }
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}
