use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Language {
    En = 1,
    Chs = 2,
    Cht = 3,
    Fr = 4,
    De = 5,
    Es = 6,
    Pt = 7,
    Ru = 8,
    Jp = 9,
    Kr = 10,
    Th = 11,
    Vi = 12,
    Id = 13,
    Tr = 14,
    It = 15,
    Unknown = 999,
}

impl Language {
    pub fn from_locale() -> Self {
        let lower = get_locale().unwrap_or_default().to_string();

        if lower.is_empty() {
            return Language::Unknown;
        }

        // ---- Chinese ----
        if lower.starts_with("zh") {
            if lower.contains("tw") || lower.contains("hk") || lower.contains("mo") {
                return Language::Cht;
            }
            return Language::Chs;
        }

        if lower.starts_with("de") {
            return Language::De;
        }
        if lower.starts_with("en") {
            return Language::En;
        }
        if lower.starts_with("es") {
            return Language::Es;
        }
        if lower.starts_with("fr") {
            return Language::Fr;
        }
        if lower.starts_with("id") {
            return Language::Id;
        }
        if lower.starts_with("it") {
            return Language::It;
        }
        if lower.starts_with("ja") || lower.starts_with("jp") {
            return Language::Jp;
        }
        if lower.starts_with("ko") || lower.starts_with("kr") {
            return Language::Kr;
        }
        if lower.starts_with("pt") {
            return Language::Pt;
        }
        if lower.starts_with("ru") {
            return Language::Ru;
        }
        if lower.starts_with("th") {
            return Language::Th;
        }
        if lower.starts_with("tr") {
            return Language::Tr;
        }
        if lower.starts_with("vi") {
            return Language::Vi;
        }

        Language::Unknown
    }
}

impl From<u32> for Language {
    fn from(value: u32) -> Self {
        match value {
            1 => Language::En,
            2 => Language::Chs,
            3 => Language::Cht,
            4 => Language::Fr,
            5 => Language::De,
            6 => Language::Es,
            7 => Language::Pt,
            8 => Language::Ru,
            9 => Language::Jp,
            10 => Language::Kr,
            11 => Language::Th,
            12 => Language::Vi,
            13 => Language::Id,
            14 => Language::Tr,
            15 => Language::It,
            _ => Language::Unknown,
        }
    }
}

#[cfg(target_os = "windows")]
fn get_locale() -> Option<String> {
    match Command::new("powershell")
        .args(["-command", "(Get-WinSystemLocale).Name"])
        .output()
    {
        Ok(output) => {
            let locale = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if locale.is_empty() {
                None
            } else {
                Some(locale)
            }
        }
        Err(_) => None,
    }
}

#[cfg(not(target_os = "windows"))]
fn get_locale() -> Option<String> {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(val) = std::env::var(key) {
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}
