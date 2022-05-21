#![feature(adt_const_params, generic_const_exprs)]

pub struct Changes<const CHANGES: &'static [&'static str]>
where
    [(); CHANGES.len()]:,
{
    changes: [usize; CHANGES.len()],
}

impl<const CHANGES: &'static [&'static str]> Changes<CHANGES>
where
    [(); CHANGES.len()]:,
{
    pub fn combine(&mut self, other: &Self) {
        for _change in &self.changes {}
    }
}

pub fn main() {}
