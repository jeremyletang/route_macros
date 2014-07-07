#![feature(phase)]

#[phase(plugin, link)]
extern crate route_macros;

pub trait Test {}

#[route = "/simple/route"]
fn simple_route(i: &Test) {}

#[method = "post"]
#[route = "/another/route/with/method"]
fn another_route_with_method(i: &Test) {}

#[route = "/another/route/with/*/and/:custom/:vars/from/url"]
fn another_route_with_wildcard_and_custom_vars_from_url(i: &Test) {}

fn main() {
    let routes = routes!();
    for &(_, ref r, ref m, ref var_names, ref regex) in routes.iter() {
        println!("route: {}, method: {}, vars: {}, regex: {}", r, m, var_names, regex);
    }
}