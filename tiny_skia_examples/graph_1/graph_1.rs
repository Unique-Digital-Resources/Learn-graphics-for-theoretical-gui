//This example was written by Abdo Mahmoud (Unique-Digital-Resources): https://github.com/Unique-Digital-Resources

use tiny_skia::{Paint, Pixmap, Stroke, FillRule, Transform, PathBuilder};



fn main() {

    let points_list:[(f32,f32); 6] = [(1.0,1.0), (2.0,2.0), (4.0,2.0), (6.0,2.0), (7.0, 5.0), (9.0,1.0)];

    let pixmap = Pixmap::new(1000, 1000).unwrap();
    graph(300.0, 300.0, 600.0,500.0 , 10, 6, &points_list, pixmap)


}

fn graph(x:f32, y:f32, w:f32, h:f32, xco:i32, yco:i32, points:&[(f32,f32)], mut pixmap:Pixmap){

    let mut point_paint = Paint::default();
    point_paint.set_color_rgba8(50, 127, 150, 255);
    point_paint.anti_alias = true;

    let mut line_paint = Paint::default();
    line_paint.set_color_rgba8(100, 100, 100, 255);
    line_paint.anti_alias = true;

    let mut circle_paint = Paint::default();
    circle_paint.set_color_rgba8(200, 100, 100, 255);
    circle_paint.anti_alias = true;

    let mut line = PathBuilder::new();
    line.move_to(x, y);
    line.line_to(x, y+h);
    line.line_to(x+w, y+h);
    let line_path = line.finish().unwrap();
    let mut line_stroke = Stroke::default();
    line_stroke.width = 10.0;
    pixmap.stroke_path(&line_path, &line_paint, &line_stroke, Transform::identity(), None);
    

    for i in 0..yco{
        // println!("resullllt {}", (y+h)/i as f32)
        let mut ycoline = PathBuilder::new();
        ycoline.move_to(x, (y+h)-h/yco as f32 * (i+1) as f32);
        ycoline.line_to(x-50.0, (y+h)-h/yco as f32 * (i+1) as f32);
        let ycopath = ycoline.finish().unwrap();
        let mut ycostroke = Stroke::default();
        ycostroke.width = 5.0;
        pixmap.stroke_path(&ycopath,&line_paint,&ycostroke, Transform::identity(),None);
    }

    for i in 0..xco{
        let mut xcoline = PathBuilder::new();
        xcoline.move_to(x + w/xco as f32 * (i+1) as f32, y+h);
        xcoline.line_to(x + w/xco as f32 * (i+1) as f32, y+h+50.0);
        let xcopath = xcoline.finish().unwrap();
        let mut xcostroke = Stroke::default();
        xcostroke.width = 5.0;
        pixmap.stroke_path(&xcopath,&line_paint,&xcostroke, Transform::identity(),None)
    }

    for i in 0..points.len()-1{
        let mut line = PathBuilder::new();
        line.move_to(x + w/xco as f32 * points[i].0, (y+h)-h/yco as f32 *(points[i].1));
        line.line_to(x + w/xco as f32 * points[i+1].0, (y+h)-h/yco as f32 *(points[i+1].1));
        let line_path = line.finish().unwrap();
        let mut line_stroke = Stroke::default();
        line_stroke.width = 5.0;
        pixmap.stroke_path(&line_path, &point_paint,&line_stroke, Transform::identity(),None)
    }

    for i in 0..points.len(){
        let mut point_circle = PathBuilder::new();
        point_circle.push_circle(x + w/xco as f32 * points[i].0, (y+h)-h/yco as f32 *(points[i].1), 10.0);
        let point_path = point_circle.finish().unwrap();
        pixmap.fill_path(&point_path, &point_paint, FillRule::Winding, Transform::identity(), None)
    }

    pixmap.save_png("graph.png").unwrap();
}