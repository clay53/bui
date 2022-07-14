use crate::{
    rect::{
        SizeAndCenter,
        Points, FillAspect,
    },
    line::{
        LineRaw,
    },
    ttf::{
        CachedFace,
        Glyph,
    },
};

pub fn compute_unfit_chars(face: &mut CachedFace, text: &str, curve_line_count: usize) -> (Vec<LineRaw>, Points, Vec<Points>) {
    let mut lines = Vec::new();
    let chars = text.chars();

    let mut unscaled_xmin = None;
    let mut unscaled_xmax = None;

    let mut unscaled_ymin = None;
    let mut unscaled_ymax = None;

    let mut unfit_char_bounds = Vec::new();

    for c in chars {
        let glyph = if let Some(glyph) = face.get_glyph(c) {
            glyph
        } else {
            match c {
                ' ' => Glyph { // TODO: figure out what this should actually be
                    tight_bounding_box: Points {
                        p1x: 0.0,
                        p1y: 0.0,
                        p2x: 200.0,
                        p2y: 0.0,
                    },
                    on_lines: Vec::with_capacity(0),
                    off_lines: Vec::with_capacity(0),
                    square_curves: Vec::with_capacity(0),
                    cube_curves: Vec::with_capacity(0),
                },
                _ => continue
            }
        };

        let offsetx = if let Some(offsetx) = unscaled_xmax {
            offsetx
        } else {
            0.0
        }-glyph.tight_bounding_box.p1x+50.0; // TODO: Figure out what spacing should actually be.

        for line in glyph.on_lines.iter().chain(glyph.off_lines.iter()) {
            lines.push(LineRaw {
                p1: [line.p1x+offsetx, line.p1y],
                p2: [line.p2x+offsetx, line.p2y]
            });
        }

        for quad_curve in glyph.square_curves.iter() {
            let mut quad_lines = quad_curve.split_as_lines(curve_line_count).iter().map(|line| -> LineRaw {
                LineRaw {
                    p1: [line.p1x+offsetx, line.p1y],
                    p2: [line.p2x+offsetx, line.p2y]
                }
            }).collect();
            lines.append(&mut quad_lines);
        }

        for cube_curve in glyph.cube_curves.iter() {
            let mut cube_lines = cube_curve.split_as_lines(curve_line_count).iter().map(|line| -> LineRaw {
                LineRaw {
                    p1: [line.p1x+offsetx, line.p1y],
                    p2: [line.p2x+offsetx, line.p2y],
                }
            }).collect();
            lines.append(&mut cube_lines);
        }

        let char_xmin = offsetx+glyph.tight_bounding_box.p1x; // p1 is always top-left
        if let Some(current_xmin) = unscaled_xmin {
            if char_xmin < current_xmin {
                unscaled_xmin = Some(char_xmin);
            }
        } else {
            unscaled_xmin = Some(char_xmin);
        }

        let char_xmax = offsetx+glyph.tight_bounding_box.p2x; // p2 is always bottom-right
        if let Some(current_xmax) = unscaled_xmax {
            if char_xmax > current_xmax {
                unscaled_xmax = Some(char_xmax);
            }
        } else {
            unscaled_xmax = Some(char_xmax);
        }

        let char_ymin = glyph.tight_bounding_box.p2y;
        if let Some(current_ymin) = unscaled_ymin {
            if char_ymin < current_ymin {
                unscaled_ymin = Some(char_ymin);
            }
        } else {
            unscaled_ymin = Some(char_ymin);
        }

        let char_ymax = glyph.tight_bounding_box.p1y;
        if let Some(current_ymax) = unscaled_ymax {
            if char_ymax > current_ymax {
                unscaled_ymax = Some(char_ymax);
            }
        } else {
            unscaled_ymax = Some(char_ymax);
        }

        unfit_char_bounds.push(Points {
            p1x: char_xmin,
            p1y: char_ymax,
            p2x: char_xmax,
            p2y: char_ymin,
        });
    }

    if unscaled_xmax.is_none() {
        return (Vec::with_capacity(0), Points { p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0 }, Vec::new())
    } else {
        return (
            lines,
            Points {
                p1x: unscaled_xmin.unwrap(),
                p1y: unscaled_ymax.unwrap(),
                p2x: unscaled_xmax.unwrap(),
                p2y: unscaled_ymin.unwrap(),
            },
            unfit_char_bounds
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointTransform {
    pub sx: f32,
    pub sy: f32,
    pub offsetx: f32,
    pub offsety: f32,
}

pub fn compute_square_transform(bounds: Points, placement_area: SizeAndCenter, resx: f32, resy: f32) -> PointTransform {
    let width = bounds.p2x-bounds.p1x;
    let height = bounds.p1y-bounds.p2y;

    let target: SizeAndCenter = FillAspect {
        placement_area: placement_area,
        centerx: 0.0,
        centery: 0.0,
        resx,
        resy,
        aspect: width/height
    }.into();
    
    let sx = target.sx/width*2.0;
    let sy = target.sy/height*2.0;
    let offsetx = -(bounds.p1x+width/2.0)*sx+target.cx;
    let offsety = -(bounds.p2y+height/2.0)*sy+target.cy;

    PointTransform {
        sx,
        sy,
        offsetx,
        offsety
    }
}

pub fn transform_lines(lines: &mut Vec<LineRaw>, transform: PointTransform) {
    for line in lines {
        line.p1[0] = line.p1[0]*transform.sx+transform.offsetx;
        line.p1[1] = line.p1[1]*transform.sy+transform.offsety;
        line.p2[0] = line.p2[0]*transform.sx+transform.offsetx;
        line.p2[1] = line.p2[1]*transform.sy+transform.offsety;
    }
}

pub fn transform_points(points: &mut Points, transform: PointTransform) {
    points.p1x = points.p1x*transform.sx+transform.offsetx;
    points.p1y = points.p1y*transform.sy+transform.offsety;
    points.p2x = points.p2x*transform.sx+transform.offsetx;
    points.p2y = points.p2y*transform.sy+transform.offsety;
}

pub fn transform_points_vec(points: &mut Vec<Points>, transform: PointTransform) {
    for points in points {
        transform_points(points, transform);
    }
}