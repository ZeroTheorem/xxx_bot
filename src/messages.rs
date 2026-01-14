use std::sync::Arc;

use ::tera::Tera;
use tera::Context;

pub struct Messages {
    tera: Tera,
}

impl Messages {
    pub fn new() -> Arc<Messages> {
        let mut tera = Tera::default();
        tera.add_raw_template("month_message", include_str!("./texts/month_message.tera"))
            .unwrap();
        tera.add_raw_template("year_message", include_str!("./texts/year_message.tera"))
            .unwrap();
        tera.add_raw_template("all_message", include_str!("./texts/all_message.tera"))
            .unwrap();
        Arc::new(Messages { tera })
    }

    pub fn all_message(&self, count: i64) -> String {
        let mut ctx = Context::new();
        ctx.insert("count", &count);
        self.tera.render("all_message", &ctx).unwrap()
    }
    pub fn month_message(&self, count: i64) -> String {
        let mut ctx = Context::new();
        ctx.insert("count", &count);
        self.tera.render("month_message", &ctx).unwrap()
    }

    pub fn year_message(&self, count: i64) -> String {
        let mut ctx = Context::new();
        ctx.insert("count", &count);
        self.tera.render("year_message", &ctx).unwrap()
    }
}
