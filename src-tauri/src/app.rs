use std::sync::RwLock;
use svg_sprite_parser::symbol::SvgSymbol;

#[derive(Default)]
pub struct ApplicationState {
    pub current_sprite: RwLock<Vec<SvgSymbol>>,
}
