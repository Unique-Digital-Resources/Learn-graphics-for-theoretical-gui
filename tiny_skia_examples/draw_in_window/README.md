# Draw in window algorithm
In this example, we'll use three libraries/crates:
- tiny skia for drawing
- winit for window creating
- and softbuffer to transfer graphics drawed by tiny skia to the window which created by winit

After drawing shapes and put it a pixmap by tiny skia, we transfer every pixel from pixmap.data() to buffer which created by softbuffer to show it in winit window



[draw_in_window.webm](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/0be4df50-3ad2-4a21-840b-1540c1f0a746)


## Usefull resources:
- https://docs.rs/tiny-skia/latest/tiny_skia/ (tiny-skia docs).
- https://github.com/RazrFalcon/tiny-skia/tree/master/examples (original examples).
- https://docs.rs/softbuffer/latest/softbuffer/ (softbuffer docs).
- https://docs.rs/winit/latest/winit/ (winit docs).
- https://users.rust-lang.org/t/how-to-draw-on-winit-window-by-2d-graphics-library/98656/ (the original topic)
