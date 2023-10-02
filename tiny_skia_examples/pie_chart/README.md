# Pie chart algorithm
To draw a pie chart - which is used to represent date in circular sections- there are several ways Algorithms in order to do this.
In this example we will use straight lines and mask to do this with Tiny-Skia.

We have user defined inputs:
x, y are the position of the pie chart, r is radius, pie_slices is represented data, full_pie is the total/full value, mask is mask layer, paint is shader/color in pixmap.

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
Now we draw the pie, by drawing a square and dividing it into 8 equal parts, each part has an angle at the point (x, y) measuring 45 degrees.
![pie_chart_drawing_alogrithm_2](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/208a2a7f-6e77-4742-b105-4b99e0561b3d)

