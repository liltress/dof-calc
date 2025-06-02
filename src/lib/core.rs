#![allow(dead_code)]

use std::fmt;
/*
c  = d/1730
H  = f*f/N*c + f
Dn = s*(H - f) / (H + s - 2f)
Df = s*(H - f) / (H - s)

H  = hyperfocal distance                     [mm]
f  = focal length                            [mm]
s  = focus distance                          [mm]
Dn = near distance for acceptable sharpness  [m]
Df = far distance for acceptable sharpness   [m]
N  = fstop                                   [-]
d  = diagonal of the sensor/print            [mm]
c  = circle of confusion                     [mm]
*/

fn pythagorean(l1: f32, l2: f32) -> f32 {
    (l1 * l1 + l2 * l2).sqrt()
}

#[derive(Debug)]
pub enum Format {
    FF135,
    HF135,
    APSC,
    MFT,
    MF6x6,
    MF6x7,
    MF6x9,
}
impl Format {
    fn get_diagonal(&self) -> f32 {
        match *self {
            Format::FF135 => pythagorean(36., 24.),
            Format::HF135 => pythagorean(18., 24.),
            Format::APSC => pythagorean(23.6, 16.8),
            Format::MFT => pythagorean(18., 13.5),
            Format::MF6x6 => pythagorean(60., 60.),
            Format::MF6x7 => pythagorean(60., 70.),
            Format::MF6x9 => pythagorean(60., 90.),
        }
    }
}
#[derive(Debug)]
pub struct Lens {
    pub name: String,
    focal_length: f32,
    focus_distance: f32,
    fstop: f32,
    pub format: Format,
}
impl Lens {
    pub fn new(fl: f32, fd: f32, f: f32, fr: Format) -> Lens {
        Lens {
            name: "-".to_string(),
            focal_length: fl,
            focus_distance: fd,
            fstop: f,
            format: fr,
        }
    }
    fn get_circle_of_confusion(&self) -> f32 {
        self.format.get_diagonal() / 1730.
    }
    fn get_hyperfocal_distance(&self) -> f32 {
        //H  = f*f/N*c + f
        ((self.focal_length * self.focal_length) / (self.fstop * self.get_circle_of_confusion()))
            + self.focal_length
    }
    pub fn get_field_of_focus(&self) -> (f32, f32) {
        //Dn = s*H / (H - (s - f))
        //Df = s*H / (H - (s + f))
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
    pub fn get_focal_length(&self) -> f32 {
        self.focal_length
    }
    pub fn get_focus_distance(&self) -> f32 {
        self.focal_length
    }
    pub fn get_fstop(&self) -> f32 {
        self.fstop
    }
}
impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {0}\nfocal length: {1}mm\nfocus distance: {2:.2}m\nfstop: {3}\ncircle of confusion: {4:.3}mm\nhyperfocal distance: {5:.2}m\nfield of focus {6:.2}-{7:.2}m",
            self.name,
            self.focal_length,
            self.focus_distance/1000.,
            self.fstop,
            self.get_circle_of_confusion(),
            self.get_hyperfocal_distance()/1000.,
            self.get_field_of_focus().0/1000.,
            self.get_field_of_focus().1/1000.
        )
    }
}
