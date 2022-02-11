use crate::term::common::*;
use crate::buffers::piece_chain::PieceChain;

pub struct EditorState {
    pub term_info: TermInfo,
    buffer: PieceChain,
    pub text: Vec<u8>,
    pub pos: usize
}

impl EditorState {

    pub fn new(term_info: TermInfo) -> Self {
        let screen_buffer_size = term_info.screen_size.width*term_info.screen_size.height;
        let buffer_size = screen_buffer_size*10;
        let n_pieces = buffer_size/2;

        EditorState {
            term_info: term_info,
            buffer: PieceChain::with_capacity(buffer_size, n_pieces),
            text: Vec::with_capacity(screen_buffer_size),
            pos: 0
        }
    }

    pub fn insert(&mut self, val: u8) {
        self.buffer.insert(val, self.pos);
        self.pos += 1;
        self.buffer.copy_to(&mut self.text);
    }

    pub fn erase(&mut self) {
        self.buffer.erase(self.pos);
        self.buffer.copy_to(&mut self.text);        
    }

    pub fn go_erase(&mut self, mv: fn(&[u8], usize) -> usize) {
        self.go_to(mv);
        self.erase();
    }

    // pub fn go_by(&mut self, mv: fn(&[u8], usize, usize) -> usize, step: usize) {
    //     self.pos = mv(&self.text, self.pos, step);
    // }    

    pub fn go_to(&mut self, mv: fn(&[u8], usize) -> usize) {
        self.pos = mv(&self.text, self.pos);
    }
}
