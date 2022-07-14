use std::collections::HashMap;
use owned_ttf_parser::{
    OwnedFace,
    AsFaceRef,
};
use log::debug;

use crate::line::LineDescriptor;
use crate::rect;

#[derive(Debug)]
pub struct CachedFace {
    face: OwnedFace,
    cached_glyphs: HashMap<char, Option<Glyph>>
}

impl CachedFace {
    pub fn new(face: OwnedFace) -> Self {
        Self {
            face,
            cached_glyphs: HashMap::new(),
        }
    }

    pub fn from_vec(vec: Vec<u8>, index: u32) -> Result<Self, ttf_parser::FaceParsingError> {
        Ok(Self {
            face: OwnedFace::from_vec(vec, index)?,
            cached_glyphs: HashMap::new(),
        })
    }

    pub fn get_glyph(&mut self, c: char) -> Option<Glyph> {
        match self.cached_glyphs.get(&c) {
            Some(glyph) => glyph.clone(),
            None => {
                debug!("Getting glyph id for '{}'", c);
                let glyph_id = if let Some(glyph_id) = self.face.as_face_ref().glyph_index(c) {
                    glyph_id
                } else {
                    self.cached_glyphs.insert(c, None);
                    return None;
                };

                let mut glyph_outline_builder = GlyphOutlineBuilder::new();

                debug!("Generating outline for '{}'", c);
                let tight_bounding_box = if let Some(tight_bounding_box) = self.face.as_face_ref().outline_glyph(glyph_id, &mut glyph_outline_builder) {
                    tight_bounding_box
                } else {
                    self.cached_glyphs.insert(c, None);
                    return None;
                };

                let glyph = Glyph {
                    tight_bounding_box: rect::Points {
                        p1x: tight_bounding_box.x_min as f32,
                        p1y: tight_bounding_box.y_max as f32,
                        p2x: tight_bounding_box.x_max as f32,
                        p2y: tight_bounding_box.y_min as f32, 
                    },
                    on_lines: glyph_outline_builder.on_lines,
                    off_lines: glyph_outline_builder.off_lines,
                    square_curves: glyph_outline_builder.square_curves,
                    cube_curves: glyph_outline_builder.cube_curves,
                };

                self.cached_glyphs.insert(c, Some(glyph.clone()));

                Some(glyph)
            }
        }
    }
}

#[derive(Debug)]
pub struct GlyphOutlineBuilder {
    pub open_x: f32,
    pub open_y: f32,
    pub x: f32,
    pub y: f32,
    pub on_lines: Vec<LineDescriptor>,
    pub off_lines: Vec<LineDescriptor>,
    pub square_curves: Vec<SquareCurve>,
    pub cube_curves: Vec<CubeCurve>,
}

impl GlyphOutlineBuilder {
    pub fn new() -> Self {
        Self {
            open_x: 0.0,
            open_y: 0.0,
            x: 0.0,
            y: 0.0,
            on_lines: Vec::new(),
            off_lines: Vec::new(),
            square_curves: Vec::new(),
            cube_curves: Vec::new(),
        }
    }
}

impl ttf_parser::OutlineBuilder for GlyphOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        debug!("move_to {{ x: {}, y: {} }}", x , y);

        self.open_x = x;
        self.open_y = y;

        self.x = x;
        self.y = y;
    }
    
    fn line_to(&mut self, x: f32, y: f32) {
        debug!("line_to {{ x: {}, y: {} }}", x , y);

        let line_category = if self.x > x || self.y < y { // depends on line having length
            &mut self.on_lines
        } else {
            &mut self.off_lines
        };
        line_category.push(LineDescriptor {
            p1x: self.x,
            p1y: self.y,
            p2x: x,
            p2y: y,
        });

        self.x = x;
        self.y = y;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        debug!("quad_to {{ x1: {}, y1: {}, x: {}, y: {} }}", x1 , y1, x, y);
        
        self.square_curves.push(SquareCurve {
            p1x: self.x,
            p1y: self.y,
            c1x: x1,
            c1y: y1,
            p2x: x,
            p2y: y,
        });
        
        self.x = x;
        self.y = y;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        debug!("curve_to {{ x1: {}, y1: {}, x2: {}, y2: {}, x: {}, y: {} }}", x1 , y1, x2, y2, x, y);
        
        self.cube_curves.push(CubeCurve {
            p1x: self.x,
            p1y: self.y,
            c1x: x1,
            c1y: y1,
            c2x: x2,
            c2y: y2,
            p2x: x,
            p2y: y,
        });

        self.x = x;
        self.y = y;
    }

    fn close(&mut self) {
        debug!("close");

        if self.x != self.open_x || self.y != self.open_y {
            debug!("Creating closing line");
            self.line_to(self.open_x, self.open_y);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Glyph {
    pub tight_bounding_box: rect::Points,
    pub on_lines: Vec<LineDescriptor>,
    pub off_lines: Vec<LineDescriptor>,
    pub square_curves: Vec<SquareCurve>,
    pub cube_curves: Vec<CubeCurve>
}

#[derive(Debug, Clone)]
pub struct SquareCurve {
    pub p1x: f32,
    pub p1y: f32,
    pub c1x: f32,
    pub c1y: f32,
    pub p2x: f32,
    pub p2y: f32,
}

impl SquareCurve {
    pub fn split_as_lines(&self, parts: usize) -> Vec<LineDescriptor> {
        let mut lines = Vec::with_capacity(parts);
        
        let part_size = 1.0/parts as f32;

        for point_index in 0..parts {
            let point_indexf = point_index as f32;

            let beginning = point_indexf*part_size;
            let beginning_complement = 1.0-beginning;

            let end = (point_indexf+1.0)*part_size;
            let end_complement = 1.0-end;

            let p1x = beginning_complement*beginning_complement*self.p1x+2.0*beginning*beginning_complement*self.c1x+beginning*beginning*self.p2x;
            let p1y = beginning_complement*beginning_complement*self.p1y+2.0*beginning*beginning_complement*self.c1y+beginning*beginning*self.p2y;

            let p2x = end_complement*end_complement*self.p1x+2.0*end*end_complement*self.c1x+end*end*self.p2x;
            let p2y = end_complement*end_complement*self.p1y+2.0*end*end_complement*self.c1y+end*end*self.p2y;

            lines.push(LineDescriptor {
                p1x,
                p1y,
                p2x,
                p2y
            });
        }
        
        lines
    }
}

#[derive(Debug, Clone)]
pub struct CubeCurve {
    pub p1x: f32,
    pub p1y: f32,
    pub c1x: f32,
    pub c1y: f32,
    pub c2x: f32,
    pub c2y: f32,
    pub p2x: f32,
    pub p2y: f32,
}

impl CubeCurve {
    pub fn split_as_lines(&self, parts: usize) -> Vec<LineDescriptor> {
        let mut lines = Vec::with_capacity(parts);
        
        let part_size = 1.0/parts as f32;

        for point_index in 0..parts {
            let point_indexf = point_index as f32;

            let beginning = point_indexf*part_size;
            let beginning_complement = 1.0-beginning;

            let end = (point_indexf+1.0)*part_size;
            let end_complement = 1.0-end;

            let p1x = 
                beginning_complement*beginning_complement*beginning_complement*self.p1x
                + 3.0*beginning*beginning_complement*beginning_complement*self.c1x
                + 3.0*beginning*beginning*beginning_complement*self.c2x
                + beginning*beginning*beginning*self.p2x;
            let p1y = 
                beginning_complement*beginning_complement*beginning_complement*self.p1y
                + 3.0*beginning*beginning_complement*beginning_complement*self.c1y
                + 3.0*beginning*beginning*beginning_complement*self.c2y
                + beginning*beginning*beginning*self.p2y;

            let p2x = 
                end_complement*end_complement*end_complement*self.p1x
                + 3.0*end*end_complement*end_complement*self.c1x
                + 3.0*end*end*end_complement*self.c2x
                + end*end*end*self.p2x;
            let p2y = 
                end_complement*end_complement*end_complement*self.p1y
                + 3.0*end*end_complement*end_complement*self.c1y
                + 3.0*end*end*end_complement*self.c2y
                + end*end*end*self.p2y;

            lines.push(LineDescriptor {
                p1x,
                p1y,
                p2x,
                p2y
            });
        }
        
        lines
    }
}