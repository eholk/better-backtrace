use super::*;

#[test]
fn parse_filter_str() {
    let filter = Filter::from_str("+foo,-bar,+bar::baz").unwrap();
    assert_eq!(
        filter,
        Filter(vec![
            FilterClause::Include(Pattern("foo".into())),
            FilterClause::Exclude(Pattern("bar".into())),
            FilterClause::Include(Pattern("bar::baz".into()))
        ])
    )
}

#[test]
fn parse_config_1() {
    let config = BacktraceConfig::from_str("0,+foo").unwrap();
    assert_eq!(
        config,
        BacktraceConfig {
            style: BacktraceStyle::None,
            filter: Filter(vec![FilterClause::Include(Pattern("foo".into()))])
        }
    )
}

#[test]
fn parse_config_2() {
    let config = BacktraceConfig::from_str("+foo").unwrap();
    assert_eq!(
        config,
        BacktraceConfig {
            style: BacktraceStyle::Short,
            filter: Filter(vec![FilterClause::Include(Pattern("foo".into()))])
        }
    )
}

#[test]
fn filter_frames() {
    let filter = Filter::from_str("+foo,-bar,+bar::baz").unwrap();

    assert!(filter.should_display_frame(false, "foo"));
    assert!(filter.should_display_frame(false, "bar::baz"));
    assert!(!filter.should_display_frame(false, "baz::bar"));
}

// core::panicking::panic
// async_backtrace::bar::async_fn$0
// core::future::from_generator::impl$1::poll<async_backtrace::foo::async_fn$0>

#[test]
fn format_frame_simple() {
    assert_eq!(
        format_frame_name("core::panicking::panic"),
        "fn core::panicking::panic"
    )
}

#[test]
fn format_frame_async() {
    assert_eq!(
        format_frame_name("async_backtrace::bar::async_fn$0"),
        "async fn async_backtrace::bar"
    )
}

#[test]
fn format_frame_nested() {
    assert_eq!(
        format_frame_name(
            "core::future::from_generator::impl$1::poll<async_backtrace::foo::async_fn_env$0>"
        ),
        "fn core::future::from_generator::impl$1::poll<async fn async_backtrace::foo>"
    )
}

#[test]
fn format_frame_nested2() {
    assert_eq!(format_frame_name("async_backtrace::block_on<tuple$<>,core::future::from_generator::GenFuture<async_backtrace::foo::async_fn_env$0> >"),
    "fn async_backtrace::block_on<tuple$<>,core::future::from_generator::GenFuture<async fn async_backtrace::foo> >")
}

#[test]
fn split_brackets_flat() {
    assert_eq!(split_brackets("A<B>C"), Some(("A", "B", "C")));
    assert_eq!(split_brackets("<B>C"), Some(("", "B", "C")));
    assert_eq!(split_brackets("A<>C"), Some(("A", "", "C")));
    assert_eq!(split_brackets("A<B>"), Some(("A", "B", "")));
}

#[test]
fn split_brackets_nested() {
    assert_eq!(split_brackets("A<B<C>D>E"), Some(("A", "B<C>D", "E")));
}

#[test]
fn split_brackets_none() {
    assert_eq!(split_brackets("abc"), None);
}

#[test]
fn split_brackets_two_pairs() {
    assert_eq!(
        split_brackets("tuple$<>,core::future::from_generator::GenFuture<async_backtrace::foo::async_fn_env$0> "),
        Some(("tuple$", "", ",core::future::from_generator::GenFuture<async_backtrace::foo::async_fn_env$0> ")));
}
