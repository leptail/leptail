#[derive(Debug, Clone, PartialEq)]
pub enum TWColor {
    Slate,
    Gray,
    Zinc,
    Neutral,
    Stone,
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose
}

impl TWColor {
    pub fn tw_string(&self) -> String {
        match self {
            TWColor::Slate => String::from("slate"),
            TWColor::Gray => String::from("gray"),
            TWColor::Zinc => String::from("zinc"),
            TWColor::Neutral => String::from("neutral"),
            TWColor::Stone => String::from("stone"),
            TWColor::Red => String::from("red"),
            TWColor::Orange => String::from("orange"),
            TWColor::Amber => String::from("amber"),
            TWColor::Yellow => String::from("yellow"),
            TWColor::Lime => String::from("lime"),
            TWColor::Green => String::from("green"),
            TWColor::Emerald => String::from("emerald"),
            TWColor::Teal => String::from("teal"),
            TWColor::Cyan => String::from("cyan"),
            TWColor::Sky => String::from("sky"),
            TWColor::Blue => String::from("blue"),
            TWColor::Indigo => String::from("indigo"),
            TWColor::Violet => String::from("violet"),
            TWColor::Purple => String::from("purple"),
            TWColor::Fuchsia => String::from("fuchsia"),
            TWColor::Pink => String::from("pink"),
            TWColor::Rose => String::from("rose"),
        }
    }
    pub fn make_shade(&self, prefix: &str, shade: Palette) -> String {
        format!("{}-{}-{}", prefix, self.tw_string(), shade.tw_string())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Palette{
    S50,
    S100,
    S200,
    S300,
    S400,
    S500,
    S600,
    S700,
    S800,
    S900,
    S950,
}
impl Palette {
    pub fn tw_string(&self) -> String {
        match self {
            Palette::S50 => String::from("50"),
            Palette::S100 => String::from("100"),
            Palette::S200 => String::from("200"),
            Palette::S300 => String::from("300"),
            Palette::S400 => String::from("400"),
            Palette::S500 => String::from("500"),
            Palette::S600 => String::from("600"),
            Palette::S700 => String::from("700"),
            Palette::S800 => String::from("800"),
            Palette::S900 => String::from("900"),
            Palette::S950 => String::from("950"),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TWColorPalette {
    color: TWColor,
    palette: Palette
}

impl TWColorPalette {
    
    pub fn from(color: TWColor, palette: Palette) -> Self {
        TWColorPalette { color: color, palette: palette }
    }

    pub fn tw_string(&self) -> String {
        format!("{}-{}", self.color.tw_string(), self.palette.tw_string())
    }
}
