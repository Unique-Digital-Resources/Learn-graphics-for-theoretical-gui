// NOTES:
//==============================================================================
//- This example was written by Abdo Mahmoud (Unique-Digital-Resources): https://github.com/Unique-Digital-Resources

//- The following libraries were used in this example:
//       tiny-skia : https://github.com/RazrFalcon/tiny-skia
//       Go to the links to find out the original authors of each library, the license and more examples if availabe.

//- You do not have to mention this source and its author or provide credit to the author when you use it for any use (personally,
//   commercially, etc.), but it will be appreciated by you, and mentioning this source and the project:
//   https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/tree/main
//   will help increase the number of potential contributors to the project, 
//   and help More programmers and learners in their learning
//==============================================================================

// Import libraries
use tiny_skia::{Paint, Pixmap, FillRule, Transform, Path, PathBuilder};



fn main() {
    
    // Create paint (the color/shader used to fill drawed shape) then create pixmap.
    // Draw the shape using rounded_corners_rectangle_draw_1 function on the pixmap. 
    // Go to line 39 - 74 to understand how rounded_corners_rectangle_draw_1 works.

    //==============================================================================

    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 127, 0, 200);
    paint.anti_alias = true;

    let mut pixmap = Pixmap::new(1000, 1000).unwrap();


    pixmap.fill_path(
        &rounded_corners_rectangle_draw_1(250.0, 250.0, 800.0, 500.0, 60.0),
        &paint,
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );

    pixmap.save_png("rounded_corners_rectangle.png").unwrap();

    //==============================================================================
}





fn rounded_corners_rectangle_draw_1(x:f32,y:f32,w:f32,h:f32,mut r:f32) -> Path
{
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
    // See 
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
    //==============================================================================
    
    // This function returns the shape as a Path to be a parameter for pixmap.fill_path function in line 30.
    let path = pb.finish().unwrap();
    return path;
}

// Usefull resources:
// https://www.desmos.com/calculator/qddv8ytxpf (Interactive cubic curves to understand how to work with them).
// https://docs.rs/tiny-skia/latest/tiny_skia/ (tiny-skia docs).
// https://github.com/RazrFalcon/tiny-skia/tree/master/examples (original examples).
