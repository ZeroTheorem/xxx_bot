use std::sync::Arc;

use ::tera::Tera;
use tera::Context;

use crate::database::Record;

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
        tera.add_raw_template("error_message", include_str!("./texts/error_message.tera"))
            .unwrap();
        tera.add_raw_template(
            "last_rows_message",
            include_str!("./texts/last_rows_message.tera"),
        )
        .unwrap();
        tera.add_raw_template(
            "certain_month_message",
            include_str!("./texts/certain_month_message.tera"),
        )
        .unwrap();
        tera.add_raw_template(
            "certain_year_message",
            include_str!("./texts/certain_year_message.tera"),
        )
        .unwrap();
        Arc::new(Messages { tera })
    }

    pub fn all_message(&self, count: i64) -> String {
        let mut ctx = Context::new();
        ctx.insert("count", &count);
        self.tera.render("all_message", &ctx).unwrap()
    }

    pub fn last_rows_message(&self, rows: Vec<Record>) -> String {
        let mut ctx = Context::new();
        ctx.insert("rows", &rows);
        self.tera.render("last_rows_message", &ctx).unwrap()
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
    pub fn certain_month_message(&self, count: i64) -> String {
        let mut ctx = Context::new();
        ctx.insert("count", &count);
        self.tera.render("certain_month_message", &ctx).unwrap()
    }
    pub fn certain_year_message(&self, year: i64, count: i64) -> String {
        let mut ctx = Context::new();
        ctx.insert("year", &year);
        ctx.insert("count", &count);
        self.tera.render("certain_year_message", &ctx).unwrap()
    }
    pub fn error_message(&self, error: &str) -> String {
        let mut ctx = Context::new();
        ctx.insert("error", &error);
        self.tera.render("error_message", &ctx).unwrap()
    }
}
