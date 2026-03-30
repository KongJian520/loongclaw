use once_cell::sync::OnceCell;
use std::{collections::BTreeMap, env, fs, path::PathBuf};

static LOCALE_MAP: OnceCell<BTreeMap<String, String>> = OnceCell::new();
static LOCALE_NAME: OnceCell<String> = OnceCell::new();

fn locale_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from("i18n/locales");
    p.push(name);
    p.set_extension("json");
    p
}

pub fn init(lang: &str) {
    if LOCALE_MAP.get().is_some() {
        return;
    }

    let path = locale_path(lang);
    let map = if let Ok(s) = fs::read_to_string(&path) {
        serde_json::from_str::<BTreeMap<String, String>>(&s).unwrap_or_default()
    } else {
        // fallback to en-US
        let fallback = locale_path("en-US");
        fs::read_to_string(&fallback)
            .ok()
            .and_then(|s| serde_json::from_str::<BTreeMap<String, String>>(&s).ok())
            .unwrap_or_default()
    };

    let _ = LOCALE_NAME.set(lang.to_string());
    let _ = LOCALE_MAP.set(map);
}

pub fn init_from_env() {
    let lang = env::var("LOONGCLAW_LANG")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "en-US".to_string());
    init(&lang);
}

pub fn tr(key: &str) -> String {
    let map = LOCALE_MAP.get_or_init(|| {
        let fallback = locale_path("en-US");
        fs::read_to_string(&fallback)
            .ok()
            .and_then(|s| serde_json::from_str::<BTreeMap<String, String>>(&s).ok())
            .unwrap_or_default()
    });
    map.get(key).cloned().unwrap_or_else(|| key.to_string())
}

pub fn tr_fmt(key: &str, vars: &[(&str, &str)]) -> String {
    let mut s = tr(key);
    for (k, v) in vars {
        s = s.replace(&format!("{{{}}}", k), v);
    }
    s
}
