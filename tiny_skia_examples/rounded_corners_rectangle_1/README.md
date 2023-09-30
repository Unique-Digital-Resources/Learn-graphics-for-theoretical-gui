# Rounded rectangle algorithm
To draw a rounded rectangle - which is used as a button widget in modern GUI - there are several ways Algorithms in order to do this.
In this example we will use straight lines and cubic curves to do this with Tiny-Skia.
In the file rounded_corners_rectangle_1.rs:
    
    ```
    // x, y are shape position, w is width, h is height and r is cubic curve sides length.
    //  Set limits for r, such that the sides of the curves do not intersect each other.
    //==============================================================================
    if h>w{
        if r > 0.24 * w{
            r = 0.24 * w
        }
    }
    else if h<w {
        if r > 0.24 * h{
            r = 0.24 * h
        }
    }
    else if h==w {
        if r > 0.24 * w{
            r = 0.24 * w
        }
    }
    //==============================================================================
    // Cubic curves used as corners of the rectangle.
    //==============================================================================
    let mut pb = PathBuilder::new();
    pb.move_to(x+r, y);
    pb.line_to(w-r, y);
    pb.cubic_to(w-(r/2 as f32),y,w,y+(r/2 as f32),w,y+r);
    pb.line_to(w, h-r);
    pb.cubic_to(w,h-(r/2 as f32),w-(r/2 as f32),h,w-r, h);
    pb.line_to(x+r, h);
    pb.cubic_to(x+(r/2 as f32),h,x,h-(r/2 as f32), x ,h-r);
    pb.line_to(x, y+r);
    pb.cubic_to(x,y+(r/2 as f32),x+(r/2 as f32),y, x+r, y);
    pb.close();
    ```
The following image may explain how the code works:
![rounded_corners_rectangle_alogrithm](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/9f764f7d-fa19-4e67-aa03-235d149ba353)

When we put the values ​​w=800, h=500, and r=60 in the function ```rounded_corners_rectangle_draw_1(x:f32,y:f32,w:f32,h:f32,mut r:f32) -> Path```, we get the following form:
![rounded_corners_rectangle_1_w800_h500_r60](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/e73cdcdf-540e-4652-82c7-1b390570b972)
​​
w=800, h=500, and r=120:
![rounded_corners_rectangle_1_w800_h500_r120](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/656e9355-edd8-4d2e-a7cf-4250a692a40d)

​​w=h:
![rounded_corners_rectangle_1_w_eq_h](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/54043142-1f9d-43e4-b85a-703b6398a460)

## Usefull resources:
- https://www.desmos.com/calculator/qddv8ytxpf (Interactive cubic curves to understand how to work with them).
- https://docs.rs/tiny-skia/latest/tiny_skia/ (tiny-skia docs).
- https://github.com/RazrFalcon/tiny-skia/tree/master/examples (original examples).
