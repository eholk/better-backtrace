use super::*;

#[test]
fn parse_filter_str() {
    let filter = Filter::from_str("+foo;-bar;+bar::baz").unwrap();
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
fn filter_frames() {
    let filter = Filter::from_str("+foo;-bar;+bar::baz").unwrap();

    assert!(filter.should_display_frame("foo"));
    assert!(filter.should_display_frame("bar::baz"));
    assert!(!filter.should_display_frame("baz::bar"));
}