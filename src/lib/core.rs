#![allow(dead_code)]

use std::fmt;

use serde::{Deserialize, Serialize};
/*
c  = d/1730 //using Zeiss formula for simplicity, more sophisticated options pending
H  = f*f/N*c + f
Dn = s*(H - f) / (H + s - 2f)
Df = s*(H - f) / (H - s)

H  = hyperfocal distance                     /[mm]
f  = focal length                            /[mm]
s  = focus distance                          /[mm]
Dn = near distance for acceptable sharpness  /[m]
Df = far distance for acceptable sharpness   /[m]
N  = fstop                                   /[-]
d  = diagonal of the sensor/print            /[mm]
c  = circle of confusion                     /[mm]
*/

fn pythagorean(l1: f32, l2: f32) -> f32 {
    (l1 * l1 + l2 * l2).sqrt()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lens {
    spec: LensSpec,
    info: LensInfo,
}

impl Lens {
    pub fn from_specs(ff: f32, fd: f32, fstop: f32) -> Lens {
        Lens {
            spec: LensSpec::new(ff, fd, fstop, Format::FF135),
            info: LensInfo {
                name: "new lens".to_string(),
                desc: None,
                artwork: None,
                mount: None,
            },
        }
    }

    pub fn with_spec(&mut self, spc: LensSpec) -> &mut Lens {
        self.spec = spc;
        self
    }
    pub fn with_fl(&mut self, fl: f32) -> &mut Lens {
        self.spec.with_fl(fl);
        self
    }
    pub fn with_fd(&mut self, fd: f32) -> &mut Lens {
        self.spec.with_fd(fd);
        self
    }
    pub fn with_fstop(&mut self, fstop: f32) -> &mut Lens {
        self.spec.with_fstop(fstop);
        self
    }
    pub fn with_format(&mut self, format: Format) -> &mut Lens {
        self.spec.with_format(format);
        self
    }

    pub fn with_name(&mut self, n: String) -> &mut Lens {
        self.info.with_name(n);
        self
    }
    pub fn with_description(&mut self, desc: String) -> &mut Lens {
        self.info.with_desc(desc);
        self
    }
    pub fn with_artwork(&mut self, art: String) -> &mut Lens {
        self.info.with_artwork(art);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensSpec {
    pub focal_length: f32,
    pub focus_distance: f32,
    pub fstop: f32,
    pub format: Format,
}
impl LensSpec {
    pub fn new(fl: f32, fd: f32, f: f32, fmt: Format) -> LensSpec {
        LensSpec {
            focal_length: fl,
            focus_distance: fd,
            fstop: f,
            format: fmt,
        }
    }

    pub fn with_fl(&mut self, fl: f32) -> &mut LensSpec {
        self.focal_length = fl;
        self
    }
    pub fn with_fd(&mut self, fd: f32) -> &mut LensSpec {
        self.focus_distance = fd;
        self
    }
    pub fn with_fstop(&mut self, f: f32) -> &mut LensSpec {
        self.fstop = f;
        self
    }
    pub fn with_format(&mut self, fmt: Format) -> &mut LensSpec {
        self.format = fmt;
        self
    }

    pub fn get_circle_of_confusion(&self) -> f32 {
        self.format.get_diagonal() / 1730.
    }
    pub fn get_hyperfocal_distance(&self) -> f32 {
        //H  = f*f/N*c + f
        ((self.focal_length * self.focal_length) / (self.fstop * self.get_circle_of_confusion()))
            + self.focal_length
    }
    pub fn get_field_of_focus(&self) -> (f32, f32) {
        //Dn = s*H / (H - (s - f))
        (
            (self.focus_distance * (self.get_hyperfocal_distance() - self.focal_length))
                / (self.get_hyperfocal_distance() + self.focus_distance - (2. * self.focal_length)),
            (self.focus_distance * (self.get_hyperfocal_distance() - self.focal_length))
                / (self.get_hyperfocal_distance() - self.focus_distance),
        )
    }
    pub fn infinity_in_focus(&self) -> bool {
        self.get_field_of_focus().1 >= self.get_hyperfocal_distance()
    }
}
impl fmt::Display for LensSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nfocal length: {0}mm\nfocus distance: {1:.2}m\nfstop: {2}\ncircle of confusion: {3:.3}mm\nhyperfocal distance: {4:.2}m\nfield of focus: {5:.2}-{6:.2}m\ninfinity in focus: {inf}",
            self.focal_length,
            self.focus_distance/1000.,
            self.fstop,
            self.get_circle_of_confusion(),
            self.get_hyperfocal_distance()/1000.,
            self.get_field_of_focus().0/1000.,
            self.get_field_of_focus().1/1000.,
            inf = {if self.infinity_in_focus() {"yes"} else {"no"} },
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    FF135,
    HF135,
    Apsc,
    Mft,
    MF6x6,
    MF6x7,
    MF6x9,
}
impl Format {
    fn get_dimensions(&self) -> (f32, f32) {
        match *self {
            Format::FF135 => (36., 24.),
            Format::HF135 => (18., 24.),
            Format::Apsc => (23.6, 16.8),
            Format::Mft => (18., 13.5),
            Format::MF6x6 => (60., 60.),
            Format::MF6x7 => (60., 70.),
            Format::MF6x9 => (60., 90.),
        }
    }

    fn get_diagonal(&self) -> f32 {
        let dim = self.get_dimensions();
        (dim.0 * dim.0 + dim.1 * dim.1).sqrt()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensInfo {
    name: String,
    desc: Option<String>,
    artwork: Option<String>,
    mount: Option<String>,
}

impl LensInfo {
    fn with_name(&mut self, n: String) -> &mut LensInfo {
        self.name = n;
        self
    }
    fn with_desc(&mut self, d: String) -> &mut LensInfo {
        self.desc = Some(d);
        self
    }
    fn with_artwork(&mut self, a: String) -> &mut LensInfo {
        self.artwork = Some(a);
        self
    }
}
