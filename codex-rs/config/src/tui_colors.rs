use schemars::JsonSchema;
use schemars::r#gen::SchemaGenerator;
use schemars::schema::InstanceType;
use schemars::schema::Metadata;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::schema::StringValidation;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct TuiColors {
    #[serde(default)]
    pub foreground: Option<TuiColor>,
    #[serde(default)]
    pub muted: Option<TuiColor>,
    #[serde(default)]
    pub accent: Option<TuiColor>,
    #[serde(default)]
    pub success: Option<TuiColor>,
    #[serde(default)]
    pub warning: Option<TuiColor>,
    #[serde(default)]
    pub error: Option<TuiColor>,
    #[serde(default)]
    pub status: Option<TuiColor>,
    #[serde(default)]
    pub border: Option<TuiColor>,
    #[serde(default)]
    pub separator: Option<TuiColor>,
    #[serde(default)]
    pub user_message_bg: Option<TuiColor>,
    #[serde(default)]
    pub composer_bg: Option<TuiColor>,
    #[serde(default)]
    pub proposed_plan_bg: Option<TuiColor>,
    #[serde(default)]
    pub selection_fg: Option<TuiColor>,
    #[serde(default)]
    pub selection_bg: Option<TuiColor>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct TuiColor {
    r: u8,
    g: u8,
    b: u8,
}

impl TuiColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl TryFrom<String> for TuiColor {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for TuiColor {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let hex = value
            .strip_prefix('#')
            .ok_or_else(|| format!("expected #RRGGBB color, got {value:?}"))?;
        if hex.len() != 6 || !hex.as_bytes().iter().all(u8::is_ascii_hexdigit) {
            return Err(format!("expected #RRGGBB color, got {value:?}"));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| format!("expected #RRGGBB color, got {value:?}"))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| format!("expected #RRGGBB color, got {value:?}"))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| format!("expected #RRGGBB color, got {value:?}"))?;
        Ok(Self { r, g, b })
    }
}

impl From<TuiColor> for String {
    fn from(value: TuiColor) -> Self {
        let (r, g, b) = value.rgb();
        format!("#{r:02X}{g:02X}{b:02X}")
    }
}

impl JsonSchema for TuiColor {
    fn schema_name() -> String {
        "TuiColor".to_string()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            metadata: Some(Box::new(Metadata {
                description: Some("Hex RGB color in #RRGGBB format.".to_string()),
                ..Default::default()
            })),
            string: Some(Box::new(StringValidation {
                pattern: Some("^#[0-9A-Fa-f]{6}$".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
