# Pie chart algorithm
To draw a pie chart - which is used to represent data in circular sections- there are several ways Algorithms in order to do this.
In this example we will use straight lines and mask to do this with Tiny-Skia.

We have user defined inputs:
x, y are the position of the pie chart, r is radius, pie_slices is represented data: (red, green, blue, alpha, value), full_pie is the total/full value, mask is mask layer, paint is shader/color in pixmap.

First, we draw a complete circle that serves as the background or the complementary part of the pie chart if the total data is less than the full value full_pie
```rust
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
```
![piechart_bg](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/77642f9b-9254-4e58-aed0-d108eac77e69)


Now we draw the pie, by drawing a square and dividing it into 8 equal parts, each part has an angle at the point (x, y) measuring 45 degrees.
![pie_chart_drawing_alogrithm_2](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/208a2a7f-6e77-4742-b105-4b99e0561b3d)

A line will be drawn whose starting point is (x,y) and whose ending point is (last_point_x , last_point_y ).
```rust
        pb.move_to(x , y );
        pb.line_to(last_point_x , last_point_y );
```
The ending point (last_point_x , last_point_y) is predetermined given the degree of the previous segment along with the degree of the current segment, and it will be on one of the sides of the square.
```rust
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
```
Then we draw other lines to close the section of the square
```rust
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
```
For example: When we draw the first segment whose degree is 60, the following values are as follows:
```
last_point_x = x, last_point_y = y-r, used_degree = 0.0, degree = 60.0
```
Note : degree = 360.0 * (the value from pie_slices's item / full_pie ) + used_degree = 
360.0 * (60.0 / 360.0) + 0.0 = 60
, full_pie = 360.0
In this case:
```rust
        else if degree > 45.0 && degree <= 90.0 {
           new_point_x = x+r;
           new_point_y = y - (r * (90.0-degree )/45.0);
        }
```
```rust
if used_degree>=0.0 && used_degree<45.0 {
            if degree > 45.0 && degree <= 135.0{
                pb.line_to(x+r , last_point_y );
            }
```
![pie_chart_drawing_alogrithm_3](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/6352a74b-9dfc-4a30-8956-26c2eede9a54)

The variables are then assigned values ​​that are used to plot the next section
```
last_point_x = new_point_x = x+r, last_point_y = new_point_y = y - (r * (90.0-degree )/45.0), used_degree = degree = 60.0
```

When we draw the second segment whose degree is 95, the following values are as follows:
```
last_point_x = x+r, last_point_y = y - (r * (90.0-60.0 )/45.0), used_degree = 60.0,
degree = 360.0 * (the value from pie_slices's item / full_pie ) + used_degree = 360.0 * (95.0 / 360) + 60.0 = 155.0
```
In this case:
```rust
        else if degree > 135.0 && degree <= 180.0{
            new_point_x = x+r - (r * (degree-135.0)/45.0 );
            new_point_y = y+r;
        } 
``````
```rust
        else if  used_degree>=45.0 && used_degree<135.0 {
            if degree > 135.0 && degree <= 225.0{
                pb.line_to(x+r , y+r );
                pb.line_to(last_point_x , y+r );
            }
```
![pie_chart_drawing_alogrithm_4](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/3491afd6-1cd0-4ba7-9f12-0ad840b0a8a3)

The variables are then assigned values ​​that are used to plot the next section
```
last_point_x = new_point_x = x+r - (r * (degree-135.0)/45.0 ), last_point_y = new_point_y = y+r, used_degree = degree = 155.0
```

And so on.
We will end up with this form:
![piechart_square](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/2250bd15-03db-48e1-8e60-9568c509784d)
We see that there is a missing part because the sum of the data values ​​is less than the full value specified by the user. The back circle will appear as if it is the missing part.

Finally, we draw another circle that will be used as a mask to cut the square and turn it into a circle:
```rust
let mut mpb = PathBuilder::new();
        mpb.push_circle(x , y , r );
        let mut mpaint = Paint::default();
        mpaint.set_color_rgba8(0, 0, 0, 255);
        mpaint.anti_alias = true;
        let path = pb.finish().unwrap();
        let mpath = mpb.finish().unwrap();
        mask.fill_path(&path, FillRule::EvenOdd, true, Transform::default());
        mask.intersect_path(&mpath,FillRule::EvenOdd, true, Transform::default());
```
```rust
        paint.set_color_rgba8(red as u8,green as u8, blue as u8, alpha as u8);
        paint.anti_alias = true;
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::EvenOdd,
            Transform::identity(),
            Some(&mask),
        );
```
We get the final output:

![piechart](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/5b3fada0-ccfe-4168-8f85-3c182e44a45b)
