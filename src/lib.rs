use std::{io::Write, str::FromStr};

pub fn print_backtrace() {
    format_backtrace(&BacktraceConfig::default(), std::io::stdout())
}

fn format_backtrace(config: &BacktraceConfig, mut out: impl Write) {
    let mut index = 0;
    backtrace::trace(|frame| {
        backtrace::resolve_frame(frame, |sym| {
            writeln!(out, "{:2}: {}", index, sym.name().expect("no symbol name")).unwrap();
            if let Some(filename) = sym.filename() {
                writeln!(out, "\tat {}:{}", filename.display(), sym.lineno().unwrap()).unwrap();
            }
            index += 1;
        });
        true
    });
}

#[derive(Debug, PartialEq)]
struct Pattern(String);

impl Pattern {
    fn matches(&self, frame: &str) -> bool {
        frame.contains(&self.0)
    }
}

#[derive(Debug, PartialEq)]
enum FilterClause {
    Include(Pattern),
    Exclude(Pattern),
}

impl FilterClause {
    /// Returns whether this frame should be displayed in the backtrace
    ///
    /// Returns None if the frame_name does not match this clause's pattern.
    fn should_display_frame(&self, frame_name: &str) -> Option<bool> {
        match self {
            FilterClause::Include(pattern) if pattern.matches(frame_name) => Some(true),
            FilterClause::Exclude(pattern) if pattern.matches(frame_name) => Some(false),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Filter(Vec<FilterClause>);

impl Filter {
    fn should_display_frame(&self, frame_name: &str) -> bool {
        self.0.iter().fold(true, |display, clause| {
            match clause.should_display_frame(frame_name) {
                Some(display) => display,
                None => display,
            }
        })
    }
}

impl FromStr for Filter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Filter(
            s.split(";")
                .map(|part| {
                    let mut chars = part.chars();
                    match chars.next().unwrap() {
                        '+' => FilterClause::Include(Pattern(String::from_iter(chars))),
                        '-' => FilterClause::Exclude(Pattern(String::from_iter(chars))),
                        _ => panic!(),
                    }
                })
                .collect(),
        ))
    }
}

enum BacktraceStyle {
    None,
    Short,
    Full,
}

struct BacktraceConfig {
    style: BacktraceStyle,
    filter: Filter,
}

impl Default for BacktraceConfig {
    fn default() -> Self {
        Self {
            style: BacktraceStyle::Short,
            filter: Filter(vec![]),
        }
    }
}

#[cfg(test)]
mod test;
