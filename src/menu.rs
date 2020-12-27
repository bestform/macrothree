use macroquad::prelude::*;
use crate::{MENU_COLOR, MENU_COLOR_HOVER};
use crate::menu::MenuState::START;
use MenuState::CONTINUE;

pub(crate) enum MenuState {
    START,
    CONTINUE,
}

pub struct Menu {
    pub(crate) items: Vec<MenuItem>,
    pub(crate) state: MenuState,
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

        if self.hover {
            let dim = self.dimensions();
            draw_rectangle_lines(
                dim.0 - 5.,
                dim.1 - 5.,
                dim.2 - dim.0 + 10.,
                dim.3 - dim.1 + 10.,
                3.,
                MENU_COLOR_HOVER
            );
            draw_line(
                screen_width() / 2. - 200.,
                dim.1 + (dim.3 - dim.1) / 2.,
                dim.0 - 5.,
                dim.1 + (dim.3 - dim.1) / 2.,
                2.,
                MENU_COLOR_HOVER
            );
            draw_line(
                screen_width() / 2. + 200.,
                dim.1 + (dim.3 - dim.1) / 2.,
                dim.0 + (dim.2 - dim.0) + 5.,
                dim.1 + (dim.3 - dim.1) / 2.,
                2.,
                MENU_COLOR_HOVER
            )
        }

        let font_size = 30;
        let text = &self.title;
        let text_size = self.measure();
        let font = self.font;
        let color = match self.hover {
            true => { MENU_COLOR_HOVER }
            false => { MENU_COLOR }
        };
        let pos = self.pos(&text_size);
        draw_text_ex(
            &text,
            pos.0,
            pos.1,
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
            pos.0,
            pos.1 - text_size.offset_y,
            text_size.width,
            text_size.height,
            1.0,
            RED
        );
        */
    }

    fn pos(&self, text_size: &TextDimensions) -> (f32, f32) {
        let x = screen_width() / 2. - text_size.width / 2.;
        let y = 100. + self.order as f32 * (text_size.offset_y + 20.) + text_size.offset_y;

        (x, y)
    }

    fn measure(&self) -> TextDimensions {
        let font_size = 30; // todo: duplicate definition with render
        let text_size = measure_text(&self.title, Some(self.font), font_size as _, 1.0);

        text_size
    }

    pub fn dimensions(&self) -> (f32, f32, f32, f32) {
        let text_size = self.measure();
        let (x, y) = self.pos(&text_size);

        (x, y - text_size.offset_y, x + text_size.width, y - text_size.offset_y + text_size.height)
    }
}

impl Menu {
    pub async fn new() -> Self {
        let start_item = MenuItem::new(0, "Start").await;
        let controls_item = MenuItem::new(1, "Controls").await;
        let quit_item = MenuItem::new(2, "Quit").await;
        Self {
            items: vec![start_item, controls_item, quit_item],
            state: START,
        }
    }

    // todo: extract to renderer module
    pub fn render(&self) {
        clear_background(BLACK);
        for menu_item in self.items.iter() {
            menu_item.render();
        }
    }

    pub fn handle_state(&mut self) {
        for item in self.items.iter_mut() {
            if item.id == 0 {
                // todo: this feels fragile as it depends on the menu initialization. But so does the click handler, so maybe just leave it as it is
                match self.state {
                    START => {
                        item.title = "Start".to_string();
                    }
                    CONTINUE => {
                        item.title = "Continue".to_string();
                    }
                }
            }
        }
    }
}