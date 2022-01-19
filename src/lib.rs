use std::{io::Write, str::FromStr};

mod panic_handler;

pub use panic_handler::install_panic_handler;

pub fn print_backtrace() {
    format_backtrace(&BacktraceConfig::default(), std::io::stdout())
}

fn format_backtrace(config: &BacktraceConfig, mut out: impl Write) {
    let trace = collect_backtrace();
    let mut show_frames = match config.style {
        BacktraceStyle::None => false,
        BacktraceStyle::Short => !trace.contains_short_end,
        BacktraceStyle::Full => true,
    };
    let mut index = 0;
    for (real_index, frame) in trace.frames.iter().enumerate() {
        if matches!(config.style, BacktraceStyle::Short)
            && frame.name.contains("__rust_begin_short_backtrace")
        {
            show_frames = false;
        }
        if config.filter.should_display_frame(show_frames, &frame.name) {
            writeln!(
                out,
                "{:2} [{:2}]: {}",
                index,
                real_index,
                format_frame_name(&frame.name)
            )
            .unwrap();
            if let Some((filename, line)) = &frame.file_position {
                writeln!(out, "\tat {}:{}", filename, line).unwrap();
            }
            index += 1;
        }
        if matches!(config.style, BacktraceStyle::Short)
            && frame.name.contains("__rust_end_short_backtrace")
        {
            show_frames = true;
        }
    }
}

/// Decodes compiler generated name cruft into something more useful
fn format_frame_name(name: &str) -> String {
    let is_async;
    let name = if let Some((name, _generator)) = name.split_once("::generator$") {
        is_async = true;
        name
    } else {
        is_async = false;
        name
    };
    format!("{}fn {}", if is_async { "async " } else { "" }, name)
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
    fn should_display_frame(&self, default: bool, frame_name: &str) -> bool {
        self.0.iter().fold(default, |display, clause| {
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
            s.split(',')
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

#[derive(Debug, PartialEq)]
enum BacktraceStyle {
    None,
    Short,
    Full,
}

impl FromStr for BacktraceStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::None),
            "1" => Ok(Self::Short),
            "full" => Ok(Self::Full),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct BacktraceConfig {
    style: BacktraceStyle,
    filter: Filter,
}

impl FromStr for BacktraceConfig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(BacktraceConfig::default());
        }

        if !s.starts_with('-') && !s.starts_with('+') {
            if let Some((style, filters)) = s.split_once(",") {
                return Ok(BacktraceConfig {
                    style: BacktraceStyle::from_str(style)?,
                    filter: Filter::from_str(filters)?,
                });
            }
        }
        Ok(BacktraceConfig {
            filter: Filter::from_str(s)?,
            ..Default::default()
        })
    }
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
