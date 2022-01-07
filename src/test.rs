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
