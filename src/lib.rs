use std::{io::Write, str::FromStr};

use backtrace::Symbol;

pub fn print_backtrace() {
    format_backtrace(&BacktraceConfig::default(), std::io::stdout())
}

fn format_backtrace(config: &BacktraceConfig, mut out: impl Write) {
    let trace = collect_backtrace();
    let mut index = 0;
    let mut show_frames = match config.style {
        BacktraceStyle::None => false,
        BacktraceStyle::Short => !trace.contains_short_end,
        BacktraceStyle::Full => true,
    };
    for (index, frame) in trace.frames.iter().enumerate() {
        if matches!(config.style, BacktraceStyle::Short) {
            if frame.name.contains("__rust_begin_short_backtrace") {
                show_frames = false;
            }
        }
        if show_frames && config.filter.should_display_frame(&frame.name) {
            writeln!(out, "{:2}: {}", index, frame.name).unwrap();
            if let Some((filename, line)) = &frame.file_position {
                writeln!(out, "\tat {}:{}", filename, line).unwrap();
            }
        }
        if matches!(config.style, BacktraceStyle::Short) {
            if frame.name.contains("__rust_end_short_backtrace") {
                show_frames = true;
            }
        }
    }
}

fn collect_backtrace() -> Backtrace {
    let mut frames = vec![];
    let mut contains_short_end = false;
    backtrace::trace(|frame| {
        let info = FrameInfo::from_frame(frame);
        if info.name.contains("__rust_end_short_backtrace") {
            contains_short_end = true;
        }
        frames.push(info);
        true
    });
    Backtrace {
        frames,
        contains_short_end,
    }
}

struct Backtrace {
    frames: Vec<FrameInfo>,
    contains_short_end: bool,
}

struct FrameInfo {
    name: String,
    /// (filename, line number)
    file_position: Option<(String, usize)>,
}

impl FrameInfo {
    fn from_frame(frame: &backtrace::Frame) -> Self {
        let mut name = "<unknown>".to_string();
        let mut file_position = None;
        backtrace::resolve_frame(frame, |sym| {
            name = sym
                .name()
                .map_or_else(|| "<unknown>".to_string(), |name| name.to_string());
            if let Some(filename) = sym.filename() {
                file_position = Some((
                    format!("{}", filename.display()),
                    sym.lineno().unwrap() as usize,
                ))
            }
        });
        Self {
            name,
            file_position,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Pattern(String);

impl Pattern {
    fn matches(&self, frame: &str) -> bool {
        frame.starts_with(&self.0)
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
            filter: Filter(vec![FilterClause::Exclude(Pattern("backtrace::".into()))]),
        }
    }
}

#[cfg(test)]
mod test;
