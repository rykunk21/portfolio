use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::window;

// ---------------------------------------------------------------------------
// Palette types
// ---------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Shades {
    #[serde(rename = "50")]
    pub _50: String,
    #[serde(rename = "100")]
    pub _100: String,
    #[serde(rename = "200")]
    pub _200: String,
    #[serde(rename = "300")]
    pub _300: String,
    #[serde(rename = "400")]
    pub _400: String,
    #[serde(rename = "500")]
    pub _500: String,
    #[serde(rename = "600")]
    pub _600: String,
    #[serde(rename = "700")]
    pub _700: String,
    #[serde(rename = "800")]
    pub _800: String,
    #[serde(rename = "900")]
    pub _900: String,
    #[serde(rename = "950")]
    pub _950: String,
}

impl Shades {
    fn shade(&self, n: u32) -> &str {
        match n {
            50 => &self._50,
            100 => &self._100,
            200 => &self._200,
            300 => &self._300,
            400 => &self._400,
            500 => &self._500,
            600 => &self._600,
            700 => &self._700,
            800 => &self._800,
            900 => &self._900,
            950 => &self._950,
            _ => &self._500,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RawPalette(pub HashMap<String, Shades>);

// ---------------------------------------------------------------------------
// Semantic role mapping
//
// Maps your colors.json keys → semantic CSS var prefixes used in Tailwind.
// Adjust here if you rename colors in colors.json.
//
//   primary   → page calls, links, button fills
//   highlight → flame / popup accents
//   accent    → warm sand & campfire glow
//   neutral   → night sky text and backgrounds
//   surface   → beach sand plates, cards, overlays
// ---------------------------------------------------------------------------

const ROLES: &[(&str, &str)] = &[
    ("primary", "primary"),
    ("highlight", "highlight"),
    ("accent", "accent"),
    ("neutral", "neutral"),
    ("surface", "surface"),
];

const SHADES: &[u32] = &[50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

// ---------------------------------------------------------------------------
// apply_palette — call once from App, writes all CSS vars to :root
// ---------------------------------------------------------------------------

pub fn apply_palette() {
    let palette_str = include_str!("../design/colors.json");
    let raw: RawPalette = serde_json::from_str(palette_str).expect("Invalid colors.json");

    let document = window().unwrap().document().unwrap();
    let root = document
        .document_element()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    for &(key, role) in ROLES {
        match raw.0.get(key) {
            Some(shades) => {
                for &n in SHADES {
                    root.style()
                        .set_property(&format!("--color-{}-{}", role, n), shades.shade(n))
                        .unwrap_or_else(|_| {
                            web_sys::console::warn_1(
                                &format!("Failed to set --color-{}-{}", role, n).into(),
                            );
                        });
                }
            }
            None => {
                web_sys::console::warn_1(&format!("colors.json missing key '{}'", key).into());
            }
        }
    }
}
