# Draw in window algorithm
In this example, we'll use three libraries/crates:
- tiny skia for drawing
- winit for window creating
- and softbuffer to transfer graphics drawed by tiny skia to the window which created by winit

After drawing shapes and put it a pixmap by tiny skia, we transfer every pixel from pixmap.data() to buffer which created by softbuffer to show it in winit window
https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/b91cb0a4-5f5d-4938-9562-29d2090a4546

