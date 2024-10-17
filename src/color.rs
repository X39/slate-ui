#[derive(Copy, Clone, Debug)]
pub struct Color {
    rgba: u32,
}
impl Color {
    pub fn red(&self) -> u8 {
        (self.rgba >> (8 * 3)) as u8
    }
    pub fn green(&self) -> u8 {
        (self.rgba >> (8 * 2)) as u8
    }
    pub fn blue(&self) -> u8 {
        (self.rgba >> (8 * 1)) as u8
    }
    pub fn alpha(&self) -> u8 {
        (self.rgba >> (8 * 0)) as u8
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        let rgba = ((red as u32) << (8 * 3))
            | ((green as u32) << (8 * 2))
            | ((blue as u32) << (8 * 1))
            | 0xFF;
        Color { rgba }
    }
}

impl Color {
    pub const BLACK: Color = Color{rgba: 0x000000FF};
    pub const WHITE: Color = Color{rgba: 0xFFFFFFFF};
    pub const RED: Color = Color{rgba: 0xFF0000FF};
    pub const GREEN: Color = Color{rgba: 0x0000FFFF};
    pub const BLUE: Color = Color{rgba: 0x0000FFFF};
    pub const TRANSPARENT: Color = Color { rgba: 0x00000000 };
    pub const YELLOW: Color = Color { rgba: 0xFFFF00FF };
    pub const CYAN: Color = Color { rgba: 0x00FFFFFF };
    pub const MAGENTA: Color = Color { rgba: 0xFF00FFFF };
    pub const ORANGE: Color = Color { rgba: 0xFFA500FF };
    pub const PURPLE: Color = Color { rgba: 0x800080FF };
    pub const GRAY: Color = Color { rgba: 0x808080FF };
    pub const LIGHT_GRAY: Color = Color { rgba: 0xD3D3D3FF };
    pub const DARK_GRAY: Color = Color { rgba: 0xA9A9A9FF };
    pub const DIM_GRAY: Color = Color { rgba: 0x696969FF };
    pub const GAINSBORO: Color = Color { rgba: 0xDCDCDCFF };
    pub const LIGHT_SLATE_GRAY: Color = Color { rgba: 0x778899FF };
    pub const BROWN: Color = Color { rgba: 0xA52A2AFF };
    pub const PINK: Color = Color { rgba: 0xFFC0CBFF };
    pub const LIME: Color = Color { rgba: 0x00FF00FF };
    pub const OLIVE: Color = Color { rgba: 0x808000FF };
    pub const MAROON: Color = Color { rgba: 0x800000FF };
    pub const NAVY: Color = Color { rgba: 0x000080FF };
    pub const TEAL: Color = Color { rgba: 0x008080FF };
    pub const SILVER: Color = Color { rgba: 0xC0C0C0FF };
    pub const GOLD: Color = Color { rgba: 0xFFD700FF };
    pub const BEIGE: Color = Color { rgba: 0xF5F5DCFF };
    pub const IVORY: Color = Color { rgba: 0xFFFFF0FF };
    pub const AQUA: Color = Color { rgba: 0x00FFFFFF };
    pub const CORAL: Color = Color { rgba: 0xFF7F50FF };
    pub const SALMON: Color = Color { rgba: 0xFA8072FF };
    pub const KHAKI: Color = Color { rgba: 0xF0E68CFF };
    pub const CHARCOAL: Color = Color { rgba: 0x36454FFF };
    pub const INDIGO: Color = Color { rgba: 0x4B0082FF };
    pub const VIOLET: Color = Color { rgba: 0xEE82EEFF };
    pub const BRONZE: Color = Color { rgba: 0xCD7F32FF };
    pub const CRIMSON: Color = Color { rgba: 0xDC143CFF };
    pub const SLATE_BLUE: Color = Color { rgba: 0x6A5ACDFF };
    pub const MINT_CREAM: Color = Color { rgba: 0xF5FFFAFF };
    pub const MIDNIGHT_BLUE: Color = Color { rgba: 0x191970FF };
    pub const CORNFLOWER_BLUE: Color = Color { rgba: 0x6495EDFF };
    pub const SEASHELL: Color = Color { rgba: 0xFFF5EEFF };
    pub const BISQUE: Color = Color { rgba: 0xFFE4C4FF };
    pub const FLORAL_WHITE: Color = Color { rgba: 0xFFFAF0FF };
    pub const INDIAN_RED: Color = Color { rgba: 0xCD5C5CFF };
    pub const LAVENDER: Color = Color { rgba: 0xE6E6FAFF };
    pub const DARK_OLIVE_GREEN: Color = Color { rgba: 0x556B2FFF };
    pub const CHOCOLATE: Color = Color { rgba: 0xD2691EFF };
    pub const PERU: Color = Color { rgba: 0xCD853FFF };
    pub const FIREBRICK: Color = Color { rgba: 0xB22222FF };
    pub const LIGHT_CORAL: Color = Color { rgba: 0xF08080FF };
    pub const ROSY_BROWN: Color = Color { rgba: 0xBC8F8FFF };
    pub const PLUM: Color = Color { rgba: 0xDDA0DDFF };
    pub const ROYAL_BLUE: Color = Color { rgba: 0x4169E1FF };
    pub const STEEL_BLUE: Color = Color { rgba: 0x4682B4FF };
    pub const PEACH_PUFF: Color = Color { rgba: 0xFFDAB9FF };
    pub const DARK_ORCHID: Color = Color { rgba: 0x9932CCFF };
    pub const PALE_TURQUOISE: Color = Color { rgba: 0xAFEEEEFF };
    pub const SLATE_GRAY: Color = Color { rgba: 0x708090FF };
    pub const LIGHT_SKY_BLUE: Color = Color { rgba: 0x87CEFAFF };
    pub const MEDIUM_VIOLET_RED: Color = Color { rgba: 0xC71585FF };
    pub const YELLOW_GREEN: Color = Color { rgba: 0x9ACD32FF };
    pub const HONEYDEW: Color = Color { rgba: 0xF0FFF0FF };
    pub const LAVENDER_BLUSH: Color = Color { rgba: 0xFFF0F5FF };
    pub const MISTY_ROSE: Color = Color { rgba: 0xFFE4E1FF };
    pub const OLD_LACE: Color = Color { rgba: 0xFDF5E6FF };
    pub const LIGHT_SALMON: Color = Color { rgba: 0xFFA07AFF };
    pub const SADDLE_BROWN: Color = Color { rgba: 0x8B4513FF };
    pub const DARK_SALMON: Color = Color { rgba: 0xE9967AFF };
    pub const LIGHT_CYAN: Color = Color { rgba: 0xE0FFFFFF };
    pub const DARK_SEA_GREEN: Color = Color { rgba: 0x8FBC8FFF };
    pub const MEDIUM_AQUAMARINE: Color = Color { rgba: 0x66CDAAFF };
    pub const MEDIUM_PURPLE: Color = Color { rgba: 0x9370DBFF };
    pub const MEDIUM_SEA_GREEN: Color = Color { rgba: 0x3CB371FF };
    pub const SANDY_BROWN: Color = Color { rgba: 0xF4A460FF };
    pub const ORCHID: Color = Color { rgba: 0xDA70D6FF };
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_to_rgb() {
        let color = Color { rgba: 0xFF00AA00 }; // Red = 255, Green = 0, Blue = 170
        assert_eq!(color.to_rgb(), (0xFF, 0x00, 0xAA));

        let color = Color { rgba: 0x80008000 }; // Red = 128, Green = 0, Blue = 128
        assert_eq!(color.to_rgb(), (0x80, 0x00, 0x80));

        let color = Color { rgba: 0x004D4D00 }; // Red = 0, Green = 77, Blue = 77
        assert_eq!(color.to_rgb(), (0x00, 0x4D, 0x4D));
    }

    #[test]
    fn test_from_rgb() {
        let color = Color::from_rgb(255, 0, 170); // Red = 255, Green = 0, Blue = 170
        assert_eq!(color.rgba, 0xFF00AAFF);

        let color = Color::from_rgb(128, 0, 128); // Red = 128, Green = 0, Blue = 128
        assert_eq!(color.rgba, 0x800080FF);

        let color = Color::from_rgb(0, 77, 77); // Red = 0, Green = 77, Blue = 77
        assert_eq!(color.rgba, 0x004D4DFF);
    }

    #[test]
    fn test_red() {
        let color = Color { rgba: 0xFF000000 }; // Red component is 255
        assert_eq!(color.red(), 0xFF);

        let color = Color { rgba: 0x80000000 }; // Red component is 128
        assert_eq!(color.red(), 0x80);

        let color = Color { rgba: 0x4D000000 }; // Red component is 77
        assert_eq!(color.red(), 0x4D);

        let color = Color { rgba: 0x4DAAAAAA }; // Red component is 77
        assert_eq!(color.red(), 0x4D);
    }

    #[test]
    fn test_green() {
        let color = Color { rgba: 0x00FF0000 }; // Green component is 255
        assert_eq!(color.green(), 0xFF);

        let color = Color { rgba: 0x00800000 }; // Green component is 128
        assert_eq!(color.green(), 0x80);

        let color = Color { rgba: 0x004D0000 }; // Green component is 77
        assert_eq!(color.green(), 0x4D);

        let color = Color { rgba: 0xAA4DAAAA }; // Green component is 77
        assert_eq!(color.green(), 0x4D);
    }

    #[test]
    fn test_blue() {
        let color = Color { rgba: 0x0000AA00 }; // Blue component is 255
        assert_eq!(color.blue(), 0xFF);

        let color = Color { rgba: 0x00008000 }; // Blue component is 128
        assert_eq!(color.blue(), 0x80);

        let color = Color { rgba: 0x00004D00 }; // Blue component is 77
        assert_eq!(color.blue(), 0x4D);

        let color = Color { rgba: 0xAAAA4DAA }; // Blue component is 77
        assert_eq!(color.blue(), 0x4D);
    }

    #[test]
    fn test_alpha() {
        let color = Color { rgba: 0x000000FF }; // Alpha component is 255
        assert_eq!(color.alpha(), 0xFF);

        let color = Color { rgba: 0x00000080 }; // Alpha component is 128
        assert_eq!(color.alpha(), 0x80);

        let color = Color { rgba: 0x0000004D }; // Alpha component is 77
        assert_eq!(color.alpha(), 0x4D);

        let color = Color { rgba: 0xAAAAAA4D }; // Alpha component is 77
        assert_eq!(color.alpha(), 0x4D);
    }
}