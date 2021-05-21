// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// Size of a module in m in all three dimensions

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Size {
    xsize: u32,
    ysize: u32,
    zsize: u32,
}

impl Size {
    pub fn new(xsize: u32, ysize: u32, zsize: u32) -> Size {
        Size {
            xsize,
            ysize,
            zsize,
        }
    }
}
