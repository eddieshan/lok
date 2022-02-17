use std::ops::Range;
use crate::core::geometry::*;
use crate::buffers::piece_chain::PieceChain;
use crate::core::collections::Search;
use crate::text::{keys::*, nav::*};

pub struct Region {
    pub start: usize,
    pub pos: usize
}

pub struct Cursor {
    pub abs: Position,
    pub rel: Position
}

pub struct TextLayout<'a> {
    pub text: &'a [u8],
    pub cursor: Cursor,
    pub lines_range: Range<usize>
}

impl Region {
    pub fn update(&mut self, text: &[u8], new_pos: usize, page_size: usize) {
        if new_pos > self.start && (&text[self.start..new_pos]).at_least(LF, page_size + 1) {
            self.start = text.rpos_n(LF, page_size - 1, new_pos).map_or(0, |v| v + 1);
        } else if new_pos < self.start {
            self.start = text.rpos(LF, new_pos).unwrap_or(0);
        }

        self.pos = new_pos;
    }
}

pub struct EditorState {
    pub window_size: Size,
    buffer: PieceChain,
    pub text: Vec<u8>,
    pub region: Region
}

impl EditorState {

    pub fn new(window_size: Size) -> Self {
        let screen_buffer_size = window_size.area();
        let buffer_size = screen_buffer_size*10;
        let n_pieces = buffer_size/2;

        EditorState {
            window_size: window_size,
            buffer: PieceChain::with_capacity(buffer_size, n_pieces),
            text: Vec::with_capacity(screen_buffer_size),
            region: Region { start: 0, pos: 0 }
        }
    }

    fn refresh(&mut self) {
        self.text.clear();
        for chunk in &self.buffer {
            self.text.extend_from_slice(chunk);
        }
    }

    pub fn insert(&mut self, val: u8) {
        self.buffer.insert(val, self.region.pos);
        self.refresh();
        self.region.update(&self.text, self.region.pos + 1, self.window_size.height);
    }

    pub fn erase(&mut self) {
        self.buffer.erase(self.region.pos);
        self.refresh();
        self.region.update(&self.text, self.region.pos, self.window_size.height);
    }

    pub fn go_erase(&mut self, mv: fn(&[u8], usize) -> usize) {
        self.go_to(mv);
        self.erase();
    }

    pub fn go_to(&mut self, mv: fn(&[u8], usize) -> usize) {
        let new_pos = mv(&self.text, self.region.pos);
        self.region.update(&self.text, new_pos, self.window_size.height);
    }

    pub fn layout(&self, page_size: usize) -> TextLayout {
        let end = self.text
            .as_slice()
            .pos_n(LF, page_size, self.region.start)
            .unwrap_or(self.text.len());

        let clipped_text = &self.text[self.region.start..end];
        let start_line = (&self.text[..self.region.start]).count(LF) + 1;
        let end_line = start_line + clipped_text.count(LF) + 1;

        TextLayout {
            text: clipped_text,
            cursor: Cursor {
                abs: (&self.text[..self.region.pos]).last_pos(),
                rel: (&self.text[self.region.start..self.region.pos]).last_pos()
            },
            lines_range: start_line..end_line            
        }
    }
}