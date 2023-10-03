// NOTES:
//==============================================================================
//- This example was written by Abdo Mahmoud (Unique-Digital-Resources): https://github.com/Unique-Digital-Resources

//- This example was written based on the example in:
//  https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/tree/main/tiny_skia_examples/draw_in_window

//- The following libraries were used in this example:
//       tiny-skia : https://github.com/RazrFalcon/tiny-skia
//       softbuffer : https://github.com/rust-windowing/softbuffer
//       winit : https://github.com/rust-windowing/winit
//       Go to the links to find out the original authors of each library, the license and more examples if availabe.

//- You do not have to mention this source and its author or provide credit to the author when you use it for any use (personally,
//   commercially, etc.), but it will be appreciated by you, and mentioning this source and the project:
//   https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/tree/main
//   will help increase the number of potential contributors to the project, 
//   and help More programmers and learners in their learning
//==============================================================================

// Import libraries
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap,Transform, PremultipliedColorU8};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();

    let mut last_pixel = PremultipliedColorU8::from_rgba(0, 0, 0, 0);
    let mut _current_pixel = PremultipliedColorU8::from_rgba(0, 0, 0, 0);

    let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
    let mut pixmap = Pixmap::new(width, height).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();


                pixmap = Pixmap::new(width, height).unwrap();
                pixmap.fill(Color::WHITE);
                
                
                let path = PathBuilder::from_circle(
                    (width / 2) as f32,
                    (height / 2) as f32,
                    (width.min(height) / 3) as f32,
                )
                .unwrap();
                let mut paint = Paint::default();
                paint.set_color_rgba8(0, 128, 128, 255);
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::EvenOdd,
                    Transform::identity(),
                    None,
                );
        







                
                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4] as u32) << 16;
                }
                buffer.present().unwrap();
            }

            

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::CursorMoved { position,device_id , modifiers } 
                    => {
                        let x = position.x as u32;
                        let y = position.y as u32;
                        
                        _current_pixel = pixmap.pixel(x, y);
                        if last_pixel != _current_pixel{
                            last_pixel = _current_pixel;
                            println!("mouse pixel: {last_pixel:?}")
                        };
                    }
                _ => {}
            }_ => {}



        }
    });
}


// Usefull resources:
// https://docs.rs/tiny-skia/latest/tiny_skia/ (tiny-skia docs).
// https://github.com/RazrFalcon/tiny-skia/tree/master/examples (original examples).
// https://docs.rs/softbuffer/latest/softbuffer/ (softbuffer docs).
// https://docs.rs/winit/latest/winit/ (winit docs).
// https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/tree/main/tiny_skia_examples/draw_in_window (The example on which this example is based)