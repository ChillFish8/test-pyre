use regex::Regex;


fn parse_route(route: &'static str) {
    lazy_static! {
        static ref ROUTE_RE: Regex = Regex::new(r"<([^>]:[^>]+)>").unwrap();
    }
    let caps = ROUTE_RE.captures("2010-03-14").unwrap();

}
