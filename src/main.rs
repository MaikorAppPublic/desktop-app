#![windows_subsystem = "windows"]

mod cmdr;
mod gen;

use crate::cmdr::Cmdr;
use crate::gen::game;
use anyhow::Result;
use maikor_platform::constants::{SAVE_COUNT, SCREEN_HEIGHT, SCREEN_WIDTH};
use maikor_platform::mem::{address, sizes};
use maikor_vm_interface::VMHost;
use pixels_graphics_lib::{setup, WindowScaling};
use std::thread::sleep;
use std::time::Duration;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, mut graphics) = setup(
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        WindowScaling::AutoFixed(2),
        "Basic Example",
        &event_loop,
    )?;

    let on_save_invalidated = |slot: usize| {
        println!("Save {} invalidated", slot);
    };
    let on_halt = |error: Option<String>| {
        println!("Halt detected, error: {:?}", error);
    };

    let mut vm_host = VMHost::new(Box::new(on_save_invalidated), Box::new(on_halt)).unwrap();

    vm_host
        .vm
        .load_game(
            game(),
            &[[0; sizes::SAVE_BANK as usize]; SAVE_COUNT as usize],
        )
        .unwrap();

    vm_host.vm.init();

    let mut cmdr = Cmdr::new(vm_host);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if graphics
                .pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            cmdr.render(&mut graphics);
        }

        if input.update(&event) {
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                graphics.pixels.resize_surface(size.width, size.height);
            }

            //put your input handling code here

            if cmdr.input(&input) {
                *control_flow = ControlFlow::Exit;
            }

            window.request_redraw();
        }

        if cmdr.update() {
            println!("Halted: {:?}", cmdr.vm_host.vm.error);
            *control_flow = ControlFlow::Exit;
        }
        sleep(Duration::from_millis(1));
    });
}
