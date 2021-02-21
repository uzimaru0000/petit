use anyhow::Result;
use reqwest::IntoUrl;

#[derive(Debug, Clone)]
pub struct Image {
    img: Vec<u8>,
}

pub enum Size {
    Char(u16),
    Px(u16),
    Percent(u16),
    Auto,
}

impl Size {
    pub fn to_string(&self) -> String {
        match self {
            Size::Char(n) => format!("{}", n),
            Size::Px(n) => format!("{}px", n),
            Size::Percent(n) => format!("{}%", n),
            Size::Auto => String::from("auto"),
        }
    }
}

pub struct Config {
    pub width: Size,
    pub height: Size,
}

impl Config {
    pub fn width(self, width: Size) -> Self {
        Self { width, ..self }
    }

    pub fn height(self, height: Size) -> Self {
        Self { height, ..self }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
        }
    }
}

impl Image {
    pub fn new(img: Vec<u8>) -> Self {
        Self { img }
    }

    pub async fn from_url<T: IntoUrl>(url: T) -> Result<Self> {
        let bytes = reqwest::get(url).await?.bytes().await?;
        Ok(Self::new(bytes.to_vec()))
    }

    pub fn view(&self) -> String {
        self.view_with_config(Config {
            width: Size::Auto,
            height: Size::Auto,
        })
    }

    pub fn view_with_config(&self, config: Config) -> String {
        let b64 = base64::encode(&self.img);
        let args = format!(
            "size={};width={};height={};inline={}",
            self.img.len(),
            config.width.to_string(),
            config.height.to_string(),
            1
        );

        format!("\x1b]1337;File={}:{}\x07", args, b64)
    }
}
