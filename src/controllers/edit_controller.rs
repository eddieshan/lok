use std::io;
use crate::core::errors::*;

use crate::text::{keys, navigation, keys::Key};
use crate::models::editor::EditorState;
use crate::views;
use super::ActionResult;

pub fn edit<T: io::Write>(key: &Key, state: &mut EditorState) -> Result<ActionResult<T>, EditorError> {

    match key.code {
        keys::CR        => state.insert(keys::LF),
        keys::UP        => state.go_to(navigation::up),
        keys::DOWN      => state.go_to(navigation::down),
        keys::RIGHT     => state.go_to(navigation::right),
        keys::LEFT      => state.go_to(navigation::left),
        keys::HTAB      => { },
        keys::LN_START  => state.go_to(navigation::start),
        keys::LN_END    => state.go_to(navigation::end),
        keys::DEL       => state.erase(),
        keys::BS        => state.go_erase(navigation::left),
        _               => {
            if key.length == 1 {
                state.insert(key.bytes[0]);
            }
        }
    };

    Ok(ActionResult {
        view: views::edit::render,
        controller: super::edit_controller::edit
    })
}