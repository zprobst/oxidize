use tera::{Tera,Context};

pub struct TemplateRenderer {
    glob: String,
    tera: Tera,
}

impl TemplateRenderer {
    pub fn new(glob: &str) -> Result<TemplateRenderer, ()> {
        match Tera::new(glob) {
            Ok(t) => Ok(TemplateRenderer {
                glob: glob.to_string(),
                tera: t,
            }),
            Err(_e) => Err(()),
        }
    }

    pub fn reload(&mut self) -> Result<(), ()> {
        self.tera.full_reload().map_err(|_| ())
    }

    pub fn render(&self, template_name: &str) -> Result<String, ()> {
        // TODO: Pass conext in through from the view.
        let mut context = Context::new();
        self.tera.render(template_name, &context).map_err(|_| ())
    }
}
