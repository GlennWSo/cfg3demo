mod component;

pub use component::Component;
use three_d::{egui::Ui, Context};

pub struct Product {
    name: Box<str>,
    parts: Box<[Component]>,
}

impl<'a> Product {
    fn new(name: &str, parts: Box<[Component]>) -> Self {
        Self {
            name: name.into(),
            parts,
        }
    }

    pub fn init(&mut self, ctx: &Context) {
        for part in self.parts.iter_mut() {
            part.init(ctx);
        }
    }

    pub fn placeholder() -> Self {
        let parts = [
            Component::placeholder(),
            Component::placeholder(),
            Component::placeholder(),
        ]
        .into();
        Self::new("The Chair (tm)", parts)
    }

    pub fn objects(&'a self) -> impl Iterator<Item = &'a (dyn three_d::Object + 'a)> {
        self.parts.iter().filter_map(move |part| part.object())
    }

    pub fn add_controls(&mut self, ui: &mut Ui) {
        ui.heading(self.name.as_ref());
        ui.add_space(10.0);
        for part in self.parts.iter_mut() {
            part.add_controls(ui)
        }
    }

    pub fn update(&mut self) {
        for part in self.parts.iter_mut() {
            part.update();
        }
    }
}
