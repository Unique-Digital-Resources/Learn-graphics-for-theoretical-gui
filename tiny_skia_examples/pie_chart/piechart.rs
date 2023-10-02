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
use tiny_skia::{Paint, Pixmap, FillRule, Transform, PathBuilder, Mask};



fn main() {
    // We create a list containing data that will be represented in the pie chart, such that each item in the list:
    // (red, green, blue, alpha, value)
    let pie_slices = [(0.0,120.0,120.0,250.0,60.0),(120.0,120.0,0.0,250.0,95.0),(0.0,200.0,50.0,250.0,12.5),(0.0,20.0,150.0,250.0,12.5),(200.0,120.0,0.0,250.0,12.5),(255.0,200.0,250.0,250.0,140.0)];//165.0)];

    let mut paint = Paint::default();
    let bgcolor = (100.0,100.0,100.0,250.0);
    // Mask layer
    let mask = Mask::new(1000, 1000).unwrap();
    // Main layer
    let pixmap = Pixmap::new(1000, 1000).unwrap();
    // Drawing pie chart by using variables pie_slices, pixmap, mask, paint, bgcolor
    // and set position to (500,500), radius 300, and full value 360
    // see how function works in line 40 - 222
    pie_chart(500.0, 500.0, 300.0, &pie_slices, 360.0, pixmap, mask, paint,bgcolor); 
}



fn pie_chart(x: f32, y: f32, r: f32, pie_slices: &[(f32, f32, f32, f32, f32)], full_pie: f32, mut pixmap: Pixmap, mut mask: Mask, mut paint:Paint, bgcolor: (f32, f32, f32, f32)){
    // x, y are the position of the pie chart, r is radius, pie_slices is represented data, full_pie is the total/full value, mask is mask layer, paint is shader/color in pixmap
    
    // Set starting point position and starting degree
    let mut last_point_x = x;
    let mut last_point_y = y-r;
    let mut used_degree = 0.0;
    // Draw and  paint background circle
    let mut bgcircle = PathBuilder::new();
    bgcircle.push_circle(x, y , r);
    let mut bgpaint = Paint::default();
    bgpaint.set_color_rgba8(bgcolor.0 as u8, bgcolor.1 as u8, bgcolor.2 as u8, bgcolor.3 as u8);
    bgpaint.anti_alias = true;
    let bgpath = bgcircle.finish().unwrap();
    pixmap.fill_path(
        &bgpath,
        &bgpaint,
        FillRule::EvenOdd,
        Transform::identity(),
        None
    );

    
    for slice in 0..pie_slices.len(){  
        
        let red = pie_slices[slice].0;
        let green = pie_slices[slice].1;
        let blue = pie_slices[slice].2;
        let alpha = pie_slices[slice].3;
        let ratio = pie_slices[slice].4/full_pie;
        let degree = 360.0*ratio + used_degree;
        
        let mut new_point_x = 0.0;
        let mut new_point_y = 0.0;
        

        // We use the condition with respect to the degree value 
        //to determine the location of the new point on one of the sides of the square
        //==============================================================================
        if degree > 0.0 && degree <= 45.0{
            new_point_x = x + (r * (degree/45.0));
            new_point_y = y-r
        }
        else if degree > 45.0 && degree <= 90.0 {
           new_point_x = x+r;
           new_point_y = y - (r * (90.0-degree )/45.0);
        }
        else if degree > 90.0 && degree <= 135.0{
            new_point_x = x+r;
            new_point_y = y + (r * (degree-90.0)/45.0 );
        } 
        else if degree > 135.0 && degree <= 180.0{
            new_point_x = x+r - (r * (degree-135.0)/45.0 );
            new_point_y = y+r;
        } 
        else if degree > 180.0 && degree <= 225.0 {
            new_point_x = x - (r * (degree-180.0)/45.0 );
            new_point_y = y+r;
        }
        else if  degree > 225.0 && degree <= 270.0{
            new_point_x = x - r;
            new_point_y = y+r - (r * (degree-225.0)/45.0 )
        }
        else if  degree > 270.0 && degree <= 315.0{
            new_point_x = x- r;
            new_point_y = y - (r * (degree-270.0)/45.0 );
        }
        else if  degree > 315.0 && degree <= 360.0{
            new_point_x = x-r + (r * (degree-315.0)/45.0 );
            new_point_y = y-r
        }
        //==============================================================================
        


        
        let mut pb = PathBuilder::new();
        pb.move_to(x , y );
        pb.line_to(last_point_x , last_point_y );



        // We use the condition regarding the degree value to determine how 
        //to draw and close the part of the square
        //==============================================================================
        if used_degree>=0.0 && used_degree<45.0 {
            if degree > 45.0 && degree <= 135.0{
                pb.line_to(x+r , last_point_y );
            }
            else if degree > 135.0 && degree <= 225.0{
                pb.line_to(x+r , y-r );
                pb.line_to(x+r , y+r );
                pb.line_to(last_point_x , y+r );
            }
            else if degree > 225.0 && degree <= 315.0{
                pb.line_to(x+r , y-r );
                pb.line_to(x+r , y+r );
                pb.line_to(x-r , y+r );
                pb.line_to(x-r , last_point_y );
            }
            else if degree > 315.0 && degree <= 360.0{
                pb.line_to(x+r , y-r );
                pb.line_to(x+r , y+r );
                pb.line_to(x-r , y+r );
                pb.line_to(x-r , y-r );
                pb.line_to(last_point_x , y-r );
            }
        }
        else if  used_degree>=45.0 && used_degree<135.0 {
            if degree > 135.0 && degree <= 225.0{
                pb.line_to(x+r , y+r );
                pb.line_to(last_point_x , y+r );
            }
            else if degree > 225.0 && degree <= 315.0{
                pb.line_to(x+r , y+r );
                pb.line_to(x-r , y+r );
                pb.line_to(x-r , last_point_y );
            }
            else if degree > 315.0 && degree <= 360.0{
                pb.line_to(x+r , y+r );
                pb.line_to(x-r , y+r );
                pb.line_to(x-r , y-r );
                pb.line_to(last_point_x , y-r );
            }
        }
        else if used_degree>=135.0 && used_degree<225.0 {
            if degree > 225.0 && degree <= 315.0{
                pb.line_to(x-r , y+r );
                pb.line_to(x-r , last_point_y );
            }
            else if degree > 315.0 && degree <= 360.0{
                pb.line_to(x-r , y+r );
                pb.line_to(x-r , y-r );
            }
        }
        else if  used_degree>=225.0 && used_degree<315.0{
            if degree > 315.0 && degree <= 360.0{
                pb.line_to(x-r , y+r );
                pb.line_to(x-r , y-r );
                pb.line_to(last_point_x , y-r );
            }

        }
        //==============================================================================

 

        
        pb.line_to(new_point_x , new_point_y );



        // Draw a mask layer and use it to cut the square and turn it into a circle
        //==============================================================================
        let mut mpb = PathBuilder::new();
        mpb.push_circle(x , y , r );
        let mut mpaint = Paint::default();
        mpaint.set_color_rgba8(0, 0, 0, 255);
        mpaint.anti_alias = true;
        let path = pb.finish().unwrap();
        let mpath = mpb.finish().unwrap();
        mask.fill_path(&path, FillRule::EvenOdd, true, Transform::default());
        mask.intersect_path(&mpath,FillRule::EvenOdd, true, Transform::default());
        //==============================================================================



        
        last_point_x = new_point_x;
        last_point_y = new_point_y;
        used_degree = degree;
  
        
        paint.set_color_rgba8(red as u8,green as u8, blue as u8, alpha as u8);
        paint.anti_alias = true;
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::EvenOdd,
            Transform::identity(),
            Some(&mask),
        );
        pixmap.save_png("piechart.png").unwrap();
    }
}

// Usefull resources:
// https://docs.rs/tiny-skia/latest/tiny_skia/ (tiny-skia docs).
// https://github.com/RazrFalcon/tiny-skia/tree/master/examples (original examples).
