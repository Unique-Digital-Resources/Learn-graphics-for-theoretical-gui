# Mouse event algorithm
Note: this example is based on [draw_in_window](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/tree/main/tiny_skia_examples/draw_in_window) example
In this example, we'll use three libraries/crates:
- tiny skia for drawing
- winit for window creating and mouse event holding
- and softbuffer to transfer graphics drawed by tiny skia to the window which created by winit

After drawing shapes and put it a pixmap by tiny skia, we transfer every pixel from pixmap.data() to buffer which created by softbuffer to show it in winit window,
We've already done that in [draw_in_window](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/tree/main/tiny_skia_examples/draw_in_window) example

Now through winit we track mouse position on the window, and if pixel in pixmap.data() at the mouse position has a diffrent color than the previous pixel color, mouse event will be trigged

This method can be used in GUI to activate an event, such as when the mouse cursor is on a button, then its color or shape changes.
Of course, there are other methods, such as the method that uses the position and area of ​​the shape to determine mathematically whether the event will be activated or not, but it may be complicated and inaccurate for non-rectangular shapes and irregular shapes (as far as I know).
[mouse_event.webm](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/8a4e6148-f5fa-4522-b284-c10d55cad816)


##  Usefull resources:
- https://docs.rs/tiny-skia/latest/tiny_skia/ (tiny-skia docs).
- https://github.com/RazrFalcon/tiny-skia/tree/master/examples (original examples).
- https://docs.rs/softbuffer/latest/softbuffer/ (softbuffer docs).
- https://docs.rs/winit/latest/winit/ (winit docs).
- https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/tree/main/tiny_skia_examples/draw_in_window (The example on which this example is based)
