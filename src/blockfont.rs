use crate::rect::*;

pub const THICK: f32 = 0.25;
pub const SPACING: f32 = 0.25;
const R: f32 = 1.0;
const G: f32 = 1.0;
const B: f32 = 1.0;
const A: f32 = 1.0;

pub struct FillText<'a> {
    pub text: &'a str,
    pub placement_area: SizeAndCenter,
    pub resx: f32,
    pub resy: f32,
}

impl From<FillText<'_>> for Vec<RectRaw> {
    fn from(fill_text: FillText) -> Self {
        let mut rects = Vec::new();
        let chars = fill_text.text.chars();

        let mut unscaled_x = 0.0;
        let unscaled_y = 2.0;

        let mut unscaled_rects: Vec<Points> = Vec::new();

        macro_rules! points {
            ($p1x:expr, $p1y:expr, $p2x:expr, $p2y:expr) => {
                unscaled_rects.push(Points {
                    p1x: $p1x+unscaled_x,
                    p1y: $p1y,
                    p2x: $p2x+unscaled_x,
                    p2y: $p2y
                })
            };
        }

        for char in chars {
            if unscaled_x > 0.0 { unscaled_x += SPACING }

            match char {
                ' ' => {},
                ':' => {
                    points!(-THICK/2.0, 0.5+THICK/2.0, THICK/2.0, 0.5-THICK/2.0);
                    points!(-THICK/2.0, -0.5+THICK/2.0, THICK/2.0, -0.5-THICK/2.0);
                },
                '+' => {
                    points!(-1.0, THICK/2.0, 1.0, -THICK/2.0);
                    points!(-THICK/2.0, 1.0, THICK/2.0, THICK/2.0);
                    points!(-THICK/2.0, -THICK/2.0, THICK/2.0, -1.0);
                },
                '-' => {
                    points!(-1.0, THICK/2.0, 1.0, -THICK/2.0);
                },
                '.' => {
                    points!(-THICK/2.0, -1.0+THICK, THICK/2.0, -1.0);
                },
                '/' => {
                    points!(-THICK/2.0, THICK/2.0, THICK/2.0, -THICK/2.0);
                    let mut i = 1.0;
                    loop {
                        // from perspective of going up & right
                        let maxes = THICK/2.0+THICK*i;
                        if maxes > 1.0 { break; }
                        
                        points!(maxes-THICK, maxes, maxes, maxes-THICK);
                        points!(-maxes, -maxes+THICK, -maxes+THICK, -maxes);
                        
                        i += 1.0;
                    }
                },
                '0' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                '1' => {
                    points!(-THICK/2.0, 1.0, THICK/2.0, -1.0);
                },
                '2' => {
                    points!(-1.0, 1.0-THICK, -1.0+THICK, 1.0-THICK*2.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(1.0-THICK, 1.0-THICK, 1.0, -1.0+THICK*3.0);
                    points!(0.0, -1.0+THICK*3.0, 1.0-THICK, -1.0+THICK*2.0);
                    points!(-1.0+THICK, -1.0+THICK*2.0, 0.0, -1.0+THICK);
                    points!(-1.0, -1.0+THICK, 1.0, -1.0);
                }
                '3' => {
                    points!(-1.0, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(-1.0, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                '4' => {
                    points!(-1.0, 1.0, -1.0+THICK, -THICK/2.0);
                    points!(-1.0+THICK, THICK/2.0, 1.0, -THICK/2.0);
                    points!(1.0-THICK*2.0, 1.0, 1.0-THICK, THICK/2.0);
                    points!(1.0-THICK*2.0, -THICK/2.0, 1.0-THICK, -1.0);
                },
                '5' => {
                    points!(-1.0, 1.0, -1.0+THICK, -THICK/2.0);
                    points!(-1.0+THICK, 1.0, 1.0, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(-1.0, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, THICK/2.0, 1.0, -1.0);
                },
                '6' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, THICK/2.0, 1.0, -1.0);
                },
                '7' => {
                    points!(-1.0, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                '8' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                '9' => {
                    points!(-1.0, 1.0, -1.0+THICK, -THICK/2.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                'A' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, 1.0-THICK*3.0, 1.0-THICK, 1.0-THICK*4.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                'B' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, 1.0-THICK, 1.0, THICK/2.0);
                    points!(1.0-THICK, -THICK/2.0, 1.0, -1.0+THICK);
                },
                'C' => {
                    points!(-1.0, 1.0, 1.0, 1.0-THICK);
                    points!(-1.0, 1.0-THICK, -1.0+THICK, -1.0+THICK);
                    points!(-1.0, -1.0+THICK, 1.0, -1.0);
                },
                'D' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, 1.0-THICK, 1.0, -1.0+THICK);
                },
                'E' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0, -THICK/2.0);
                    points!(-1.0+THICK, -1.0+THICK, 1.0, -1.0);
                },
                'F' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0, -THICK/2.0);
                },
                'G' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK*3.0, THICK/2.0, 1.0, -THICK/2.0);
                    points!(-1.0+THICK, -1.0+THICK, THICK, -1.0);
                    points!(THICK, -THICK/2.0, THICK*2.0, -1.0);
                },
                'H' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                'I' => {
                    points!(-1.0, 1.0, 1.0, 1.0-THICK);
                    points!(-THICK/2.0, 1.0-THICK, THICK/2.0, -1.0+THICK);
                    points!(-1.0, -1.0+THICK, 1.0, -1.0);
                },
                'J' => {
                    points!(-1.0, 1.0, 1.0, 1.0-THICK);
                    points!(-THICK/2.0, 1.0-THICK, THICK/2.0, -1.0+THICK);
                    points!(-1.0, -1.0+THICK, THICK/2.0, -1.0);
                },
                'K' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);

                    let mut i = 1.0;
                    loop {
                        let maxy = THICK*i;
                        if maxy > 1.0 { break }
                        let maxx = -1.0+THICK*(i+1.0);
                        if maxx > 1.0 { break }

                        points!(maxx-THICK, maxy, maxx, maxy-THICK);
                        points!(maxx-THICK, -maxy+THICK, maxx, -maxy);

                        i += 1.0;
                    }
                },
                'L' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, -1.0+THICK, 1.0, -1.0);
                },
                'N' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);

                    points!(-THICK/2.0, THICK/2.0, THICK/2.0, -THICK/2.0);
                    let mut i = 1.0;
                    loop {
                        // from perspective of going up & right
                        let maxes = THICK/2.0+THICK*i;
                        if maxes > 1.0 { break; }
                        
                        points!(-maxes, maxes, -maxes+THICK, maxes-THICK);
                        points!(maxes-THICK, -maxes+THICK, maxes, -maxes);
                        
                        i += 1.0;
                    }
                },
                'O' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                'P' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(1.0-THICK, 1.0, 1.0, -THICK/2.0);
                },
                'Q' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK*2.0, -1.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0+THICK*2.0);
                    points!(1.0-THICK, -1.0+THICK, 1.0, -1.0);
                    points!(1.0-THICK*2.0, -1.0+THICK*2.0, 1.0-THICK, -1.0+THICK);
                    points!(1.0-THICK*3.0, -1.0+THICK*3.0, 1.0-THICK*2.0, -1.0+THICK*2.0);
                },
                'R' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, 1.0, 1.0-THICK, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(1.0-THICK, 1.0, 1.0, -THICK/2.0);

                    let part_width = (2.0-THICK*2.0)/(1.0/THICK-0.5).floor();
                    let mut i = 1.0;
                    loop {
                        let maxx = -1.0+THICK*2.0+i*part_width;
                        if maxx > 1.0 { break }
                        let miny = -THICK*(0.5+i);
                        if miny < -1.0 { break }

                        points!(maxx-part_width, miny+THICK, maxx, miny);

                        i += 1.0;
                    }
                },
                'S' => {
                    points!(-1.0, 1.0, -1.0+THICK, -THICK/2.0);
                    points!(-1.0+THICK, 1.0, 1.0, 1.0-THICK);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(-1.0, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, THICK/2.0, 1.0, -1.0);
                },
                'T' => {
                    points!(-1.0, 1.0, 1.0, 1.0-THICK);
                    points!(-THICK/2.0, 1.0-THICK, THICK/2.0, -1.0);
                },
                'U' => {
                    points!(-1.0, 1.0, -1.0+THICK, -1.0);
                    points!(-1.0+THICK, -1.0+THICK, 1.0-THICK, -1.0);
                    points!(1.0-THICK, 1.0, 1.0, -1.0);
                },
                'V' => {
                    points!(-THICK/2.0, -1.0+THICK, THICK/2.0, -1.0);

                    let part_height = THICK*2.25;
                    let mut i = 1.0;
                    loop {
                        // from perspective of going up & right
                        let maxy = -1.0+THICK+i*part_height;
                        if maxy > 1.0 { break }
                        let maxx = THICK*(0.5+i);
                        if maxx > 1.0 { break }

                        points!(maxx-THICK, maxy, maxx, maxy-part_height);
                        points!(-maxx, maxy, -maxx+THICK, maxy-part_height);
                        
                        i += 1.0;
                    }
                },
                'X' => {
                    points!(-THICK/2.0, THICK/2.0, THICK/2.0, -THICK/2.0);
                    let mut i = 1.0;
                    loop {
                        // from perspective of going up & right
                        let maxes = THICK/2.0+THICK*i;
                        if maxes > 1.0 { break; }
                        
                        points!(maxes-THICK, maxes, maxes, maxes-THICK);
                        points!(-maxes, maxes, -maxes+THICK, maxes-THICK);
                        points!(maxes-THICK, -maxes+THICK, maxes, -maxes);
                        points!(-maxes, -maxes+THICK, -maxes+THICK, -maxes);
                        
                        i += 1.0;
                    }
                },
                'Y' => {
                    points!(-1.0, 1.0, -1.0+THICK, -THICK/2.0);
                    points!(-1.0+THICK, THICK/2.0, 1.0-THICK, -THICK/2.0);
                    points!(1.0-THICK, 1.0, 1.0, -THICK/2.0);
                    points!(-THICK/2.0, -THICK/2.0, THICK/2.0, -1.0);
                },
                _ => {
                    points!(-THICK/2.0, THICK/2.0, THICK/2.0, -THICK/2.0);
                }
            };

            unscaled_x += 2.0;
        }

        let target: SizeAndCenter = FillAspect {
            placement_area: fill_text.placement_area,
            centerx: 0.0,
            centery: 0.0,
            resx: fill_text.resx,
            resy: fill_text.resy,
            aspect: unscaled_x/unscaled_y,
        }.into();
        
        for unscaled_rect in unscaled_rects {
            let size_and_center: SizeAndCenter = Points {
                p1x: (unscaled_rect.p1x+1.0-unscaled_x/2.0)*target.sx*2.0/unscaled_x,
                p1y: unscaled_rect.p1y*target.sy*2.0/unscaled_y,
                p2x: (unscaled_rect.p2x+1.0-unscaled_x/2.0)*target.sx*2.0/unscaled_x,
                p2y: unscaled_rect.p2y*target.sy*2.0/unscaled_y,
            }.into();

            rects.push(RectRaw {
                scale: [size_and_center.sx, size_and_center.sy],
                translation: [size_and_center.cx+target.cx, size_and_center.cy+target.cy],
                color: [R, G, B, A],
            })
        }

        rects
    }
}