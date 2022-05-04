#![windows_subsystem = "windows"]

use anyhow::Result;
use maikor_vm_core::constants::graphics::{SCREEN_HEIGHT, SCREEN_WIDTH};
use maikor_vm_interface::VMHost;
use pixels_graphics_lib::{setup, WindowScaling};
use std::thread::sleep;
use std::time::Duration;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<()> {
    let mut rendered = false;
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, mut graphics) = setup(
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        WindowScaling::AutoFixed(2),
        "Basic Example",
        &event_loop,
    )?;

    let mut vm_host = VMHost::new();
    vm_host.pop_test();
    vm_host.run();

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

            if !rendered {
                rendered = true;
                vm_host.render(graphics.pixels.get_frame());
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                graphics.pixels.resize_surface(size.width, size.height);
            }

            //put your input handling code here

            window.request_redraw();
        }

        sleep(Duration::from_millis(1));
    });
}
