mod component;

pub use component::Component;

struct Product {
    parts: Box<[Component]>,
}
