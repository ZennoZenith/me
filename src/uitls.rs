// pub static CONFIGURATION: Lazy<Settings> = Lazy::new(Settings::default);

use once_cell::sync::Lazy;
use tera::Tera;

pub static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);
    // tera.register_filter("do_nothing", do_nothing_filter);
    tera
});
