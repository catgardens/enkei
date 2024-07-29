use cursive::view::View;
use cursive::{Vec2, XY};

use crate::{board::Board, item::Item};

use super::App;

impl View for Item {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print(XY::zero(), &self.name);
    }
    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(self.name.len(), 1)
    }
}

impl View for Board {
    fn draw(&self, printer: &cursive::Printer) {
        let mut offset = Vec2::zero();
        self.items.iter().for_each(|item| {
            item.draw(&printer.offset(offset));
            offset = offset.map_y(|v| v + item.clone().required_size(printer.size).y);
        });
    }
}

impl View for App {
    fn draw(&self, printer: &cursive::Printer) {
        self.board.draw(printer);
    }
    fn required_size(&mut self, _constraint: cursive::Vec2) -> Vec2 {
        Vec2::new(90, 20)
    }
}
