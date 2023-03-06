use crate::wasm4;

pub fn set_palette(pallete: [u32; 4]) {
    unsafe {
        *wasm4::PALETTE = pallete;
    }
}

pub fn set_draw_color(idx: u16) {
    unsafe { *wasm4::DRAW_COLORS = idx.into() }
}
