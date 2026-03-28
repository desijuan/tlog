use std::fs;
use std::sync::OnceLock;

macro_rules! static_resource {
    ($name:ident, $path:literal) => {
        pub fn $name() -> &'static str {
            static RES: OnceLock<String> = OnceLock::new();
            RES.get_or_init(|| fs::read_to_string($path).expect(concat!("Unable to read ", $path)))
        }
    };
}

static_resource!(login_page_html, "static/login/index.html");
static_resource!(login_page_css, "static/login/styles.css");
static_resource!(login_page_js, "static/login/script.js");

static_resource!(worker_home_html, "static/worker/index.html");
static_resource!(worker_home_css, "static/worker/styles.css");
static_resource!(worker_home_js, "static/worker/script.js");
