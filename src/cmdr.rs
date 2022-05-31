use std::time::Instant;
use maikor_vm_interface::VMHost;
use pixels_graphics_lib::color::{BLACK, BLUE, WHITE};
use pixels_graphics_lib::drawing::PixelWrapper;
use pixels_graphics_lib::text::TextSize;
use winit::event::VirtualKeyCode;
use winit_input_helper::{TextChar, WinitInputHelper};
use crate::cmdr::Mode::*;

enum Mode {
    Help,
    History,
    Input(InputMode),
}

enum InputMode {
    ViewMemory(u16)
}

enum Overlay {
    PC,
    MemByte(u16),
    Reg(u8),
    ExtReg(u8)
}

pub struct Cmdr {
    active: bool,
    overlay: Vec<Overlay>,
    history: Vec<String>,
    mode: Mode,
    cursor_blink: (f32, bool),
    pub(crate) vm_host: VMHost,
}

impl Cmdr {
    pub fn new(vm_host: VMHost) -> Self {
        Self {
            active: false,
            overlay: vec![Overlay::PC, Overlay::MemByte(55122)],
            mode: Help,
            history: vec![],
            cursor_blink: (0.0,false),
            vm_host
        }
    }
}

impl Cmdr {
    pub fn render(&self, graphics: &mut PixelWrapper) {
        if self.active {
            self.render_cmdr(graphics);
        } else {
            self.vm_host.render(graphics.pixels.get_frame());
            for (i,item) in self.overlay.iter().enumerate() {
                match item {
                    Overlay::PC => graphics.draw_text(&format!("PC: {: >5}", self.vm_host.vm.pc), 10, 31, i as isize, TextSize::Small, WHITE),
                    Overlay::MemByte(addr) => {
                        graphics.draw_text(&format!("{}: {: >3}", addr, self.vm_host.vm.memory[*addr as usize]), 12, 30, i as isize, TextSize::Small, WHITE)
                    }
                    Overlay::Reg(_) => {}
                    Overlay::ExtReg(_) => {}
                }
            }
            graphics.draw_text(&format!("{}", self.vm_host.vm.cycles_executed), 22, 30, 22, TextSize::Small, WHITE);
        }
    }

    pub fn input(&mut self, input: &WinitInputHelper) -> bool{
        if self.active {
            self.input_cmdr(input);
        } else {
            if input.key_pressed(VirtualKeyCode::Escape) {
                return true;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                self.active = true;
                return false;
            }
            if input.key_held(VirtualKeyCode::W) {
                self.vm_host.input_state.up = true;
                self.vm_host.input_state.cached = None;
            } else {
                self.vm_host.input_state.up = false;
                self.vm_host.input_state.cached = None;
            }
        }
        return false;
    }

    pub fn update(&mut self) -> bool{
        if self.active {
            self.update_cmdr();
        } else {
            self.vm_host.execute();
            if self.vm_host.vm.halted {
                return true;
            }
        }
        false
    }
}

impl Cmdr {
    fn clear_mode(&mut self) {
        if self.history.is_empty() {
            self.mode = Help;
        } else {
            self.mode = History;
        }
    }

    fn render_cmdr(&self, graphics: &mut PixelWrapper) {
        graphics.clear(BLACK);
        graphics.draw_text_px("Memory Commander", 20, 2,2,TextSize::Normal, WHITE);

        let (cw, ch) = TextSize::get_max_characters(&TextSize::Small,graphics.width(), graphics.height());

        match &self.mode {
            Help => {
                graphics.draw_text("v) View memory contents", cw, 1,3, TextSize::Small, WHITE);
            }
            Input(submode) => match submode {
                InputMode::ViewMemory(addr) => {
                    graphics.draw_text("View memory range:", cw, 1, 4, TextSize::Small, BLUE);
                    graphics.draw_text(&format!("Start: {: >5}", addr), cw, 2,5,TextSize::Small, WHITE);
                }
            }
            History => {
                for (i, line) in self.history.iter().rev().take(23).rev().enumerate() {
                    graphics.draw_text(line, cw, 1, (3 + i) as isize, TextSize::Small, WHITE);
                }
            }
        }
    }

    fn input_cmdr(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::P) || input.key_pressed(VirtualKeyCode::Escape) {
            self.active = false;
        }
        if input.key_pressed(VirtualKeyCode::H) {
            self.mode = Help;
        }
        if input.key_pressed(VirtualKeyCode::V) {
            self.mode = Input(InputMode::ViewMemory(0))
        }
        match &mut self.mode {
            Help => {}
            History => {}
            Input(submode) => match submode {
                InputMode::ViewMemory(addr) => {
                    if input.key_pressed(VirtualKeyCode::Escape) {
                        self.clear_mode();
                        return;
                    }
                    if input.key_pressed(VirtualKeyCode::Return) {
                        let bytes_per_line = 40 / 3;
                        let addr = *addr as usize;
                        let mem = &self.vm_host.vm.memory[addr..(addr + 52)];
                        for (i, chunk) in mem.chunks(bytes_per_line).enumerate() {
                            self.history.push(format!("{: >5}: {}", addr as usize + (i * bytes_per_line), chunk.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ")));
                        }
                        self.clear_mode();
                        return;
                    }
                    for letter in input.text() {
                        match letter {
                            TextChar::Char(c) => {
                                if c.is_ascii_digit() {
                                    let mut text = addr.to_string();
                                    if text.len() < 5 {
                                        text.push(c);
                                        if let Ok(num) = u16::from_str_radix(&text, 10) {
                                            *addr = num;
                                        }
                                    }
                                }
                            }
                            TextChar::Back => {
                                let mut text = addr.to_string();
                                text.pop();
                                if let Ok(num) = u16::from_str_radix(&text, 10) {
                                    *addr = num;
                                    return;
                                }
                                *addr = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    fn update_cmdr(&mut self) {
        self.cursor_blink.0 -= 1.0;
        if self.cursor_blink.0 < 0.0 {
            self.cursor_blink = (1000.0, !self.cursor_blink.1);
        }
    }
}