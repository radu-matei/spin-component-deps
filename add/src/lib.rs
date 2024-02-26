mod bindings;
// Separating out the interface puts it in a sub-module
use crate::bindings::exports::example::component::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
