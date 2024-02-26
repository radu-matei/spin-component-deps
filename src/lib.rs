use spin_sdk::http::{IntoResponse, Response};
use spin_sdk::http_component;

mod bindings;
use bindings::example::component::add::add;

/// A simple Spin HTTP component.
#[http_component]
fn hello_world(_req: http::Request<()>) -> anyhow::Result<impl IntoResponse> {
    let x = add(123, 456);
    println!("Value is: {}", x);
    Ok(Response::new(200, "Hello, world!"))
}
