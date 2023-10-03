# Draw in window algorithm
In this example, we'll use three libraries/crates:
- tiny skia for drawing
- winit for window creating
- and softbuffer to transfer graphics drawed by tiny skia to the window which created by winit

After drawing shapes and put it a pixmap by tiny skia, we transfer every pixel from pixmap.data() to buffer which created by softbuffer to show it in winit window



[draw_in_window.webm](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/de1bfb69-522d-41db-8660-e374a885454f)

