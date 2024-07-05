
use crossterm::style::Color::{AnsiValue, Green, Rgb, Yellow};
use termimad::{rgb, MadSkin, StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb {
        r: 196,
        g: 164,
        b: 132,
    });
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.set_headers_fg(Yellow);
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '▐');
    skin.quote_mark.set_fg(Rgb {
        r: 181,
        g: 101,
        b: 29,
    });
    skin.inline_code.set_fg(Rgb {
        r: 162,
        g: 105,
        b: 70,
    });
    skin.italic.set_fg(Rgb {
        r: 245,
        g: 245,
        b: 220,
    });

    skin
}