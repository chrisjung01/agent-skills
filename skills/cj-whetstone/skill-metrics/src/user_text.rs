//! Text the user wrote in a window, with secrets redacted before it reaches the database.

use regex::Regex;
use std::sync::LazyLock;

pub const PROMPT: &str = "prompt";
pub const DENIAL_REASON: &str = "denial reason";
pub const OWN_WORD_ANSWER: &str = "own-word answer";

#[derive(Debug, Clone, PartialEq)]
pub struct UserText {
    /// `PROMPT`, `DENIAL_REASON` or `OWN_WORD_ANSWER`.
    pub kind: &'static str,
    pub text: String,
    /// Matches `CORRECTION` or `STRONG`.
    pub correction: bool,
    /// Matches `STRONG`.
    pub strong: bool,
}

impl UserText {
    /// `None` for text that is only whitespace.
    pub fn new(kind: &'static str, text: &str) -> Option<Self> {
        let text = text.trim();
        (!text.is_empty()).then(|| {
            let strong = STRONG.is_match(text);
            UserText {
                kind,
                text: redact(text),
                correction: strong || CORRECTION.is_match(text),
                strong,
            }
        })
    }
}

/// The old SessionEnd hook's patterns, unchanged. "nein" also approves ("nein, setz das um"), so a
/// flag marks text to read, not a verdict.
static CORRECTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(nein|nicht so|falsch|stattdessen|anders|lieber|bitte nicht|korrigier\w*|",
        r"das stimmt nicht|warum hast du|du hast \w* ?vergessen|",
        r"quatsch|zu lange|dauert (echt |so |viel )?(zu )?lange|",
        r"no|wrong|instead|don'?t|do not|actually|you forgot|why did you)\b",
    ))
    .unwrap()
});

/// "immer" alone is too common in German, so only as an instruction.
static STRONG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(nie wieder|niemals|immer (mit|ohne|zuerst|erst)|grundsätzlich|never|always)\b",
    )
    .unwrap()
});

/// Applied in order, as the old SessionEnd hook did.
static SECRETS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"gh[pousr]_[A-Za-z0-9]{20,}",
        r"github_pat_[A-Za-z0-9_]{20,}",
        r"sk-[A-Za-z0-9_\-]{20,}",
        r"AKIA[0-9A-Z]{16}",
        r"(?i)bearer\s+\S+",
        // The optional quotes catch JSON keys in tool inputs.
        r#"(?i)(password|passwort|token|secret|api[_-]?key)"?\s*[:=]\s*\S+"#,
    ]
    .iter()
    .map(|pattern| Regex::new(pattern).unwrap())
    .collect()
});

/// Long random strings; also matches long paths, so only text the user typed gets it.
static LONG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Za-z0-9+/=_\-]{40,}").unwrap());

fn redact(text: &str) -> String {
    LONG.replace_all(&redact_keys(text), "[redacted]")
        .into_owned()
}

/// The known key and token shapes, without the long-string pattern, so paths stay readable.
pub fn redact_keys(text: &str) -> String {
    SECRETS.iter().fold(text.to_owned(), |text, secret| {
        secret.replace_all(&text, "[redacted]").into_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_match_whole_words_in_any_case() {
        let flags = |text| {
            let t = UserText::new(PROMPT, text).unwrap();
            (t.correction, t.strong)
        };
        assert_eq!(flags("Nein, nicht so"), (true, false));
        assert_eq!(flags("why did you do that"), (true, false));
        assert_eq!(flags("du hast es vergessen"), (true, false));
        assert_eq!(flags("das dauert echt zu lange"), (true, false));
        assert_eq!(flags("NIEMALS force-pushen"), (true, true));
        assert_eq!(flags("always run the tests"), (true, true));
        assert_eq!(flags("grundsätzlich auf Deutsch"), (true, true));
        // Inside other words, and "immer" that is no instruction.
        assert_eq!(flags("keine Ahnung, nobody, knowing"), (false, false));
        assert_eq!(flags("wie du immer darauf kommst"), (false, false));
    }

    #[test]
    fn secrets_are_redacted() {
        let cases = [
            ("ghs_abcdefghijklmnopqrst1", "[redacted]"),
            ("github_pat_abc_defghijklmnopqrstu", "[redacted]"),
            ("key AKIAABCDEFGHIJKLMNOP", "key [redacted]"),
            ("Authorization: Bearer abc.def", "Authorization: [redacted]"),
            ("Passwort: geheim weiter", "[redacted] weiter"),
            ("api-key=x1 token = y2", "[redacted] [redacted]"),
            (r#"{"password":"hunter2"}"#, r#"{"[redacted]"#),
            (&"a".repeat(40), "[redacted]"),
            // Too short to be a key, and ordinary words.
            ("sk-short and ghp_short", "sk-short and ghp_short"),
            ("the token was fine", "the token was fine"),
        ];
        for (text, redacted) in cases {
            assert_eq!(redact(text), redacted, "{text}");
        }
    }
}
