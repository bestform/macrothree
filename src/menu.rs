use macroquad::prelude::*;
use crate::{MENU_COLOR, MENU_COLOR_HOVER};

pub struct Menu {
    pub(crate) items: Vec<MenuItem>
}

pub(crate) struct MenuItem {
    pub(crate) id: i32,
    order: i32,
    title: String,
    pub(crate) font: Font,
    pub(crate) hover: bool, // intended for mouse over. Implement me!
}

impl MenuItem {
    pub async fn new(id: i32, title: &str) -> Self {
        Self {
            id,
            order: id,
            font: load_ttf_font("Resources/LASER.ttf").await,
            hover: false,
            title: String::from(title),
        }
    }

    // todo: extract to renderer module
    pub fn render(&self) {
        let font_size = 30;
        let text = &self.title;
        let text_size = measure_text(text, Some(self.font), font_size as _, 1.0);
        let font = self.font;
        let color = match self.hover {
            true => { MENU_COLOR_HOVER }
            false => { MENU_COLOR }
        };
        draw_text_ex(
            &text,
            screen_width() / 2. - text_size.0 / 2.,
            100. + self.order as f32 * (text_size.1 + 20.),
            TextParams {
                font,
                font_size,
                font_scale: 1.0,
                color,
            },
        );
        // debug
        /*
        draw_rectangle_lines(
            screen_width() / 2. - text_size.0 / 2.,
            100. + self.order as f32 * (text_size.1 + 20.) + 9.,
            text_size.0,
            text_size.1,
            1.0,
            RED
        );
        */
    }

    pub fn dimensions(&self) -> (f32, f32, f32, f32) {
        let font_size = 30; // todo: duplicate definition with render
        let text = &self.title;
        let text_size = measure_text(text, Some(self.font), font_size as _, 1.0);
        // todo: extract to method
        // todo: why are there 9 pixels offset when rendering at font size 30. Note: the offset changes with font size!
        let (x, y) = (screen_width() / 2. - text_size.0 / 2., 100. + self.order as f32 * (text_size.1 + 20.) + 9.);


        (x, y, x + text_size.0, y + text_size.1)
    }
}

impl Menu {
    pub async fn new() -> Self {
        let start_item = MenuItem::new(0, "Start").await;
        let controls_item = MenuItem::new(1, "Controls").await;
        let quit_item = MenuItem::new(2, "Quit").await;
        Self {
            items: vec![start_item, controls_item, quit_item]
        }
    }

    // todo: extract to renderer module
    pub fn render(&self) {
        clear_background(BLACK);
        for menu_item in self.items.iter() {
            menu_item.render();
        }
    }
}