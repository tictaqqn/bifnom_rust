extern crate bifnom;
use bifnom::parse::{neighbor_parse, triangles_parse, result_parse};
use bifnom::projection::run;
use std::path::Path;
use ndarray::prelude::*;

#[test]
fn parse_and_run() {
    let neighbors = neighbor_parse(&Path::new("Input_armadillo/nei/neighbor_points_00000.txt"));
    let tris = triangles_parse(&Path::new("Input_armadillo/tri/triangles_00000.txt"));
    let tris_next = triangles_parse(&Path::new("Input_armadillo/tri/triangles_00001.txt"));
    let feats = result_parse(&Path::new("RIFNOM_TAVE015_TVAR005_TANG025_TDIFF12_TTRACK6/result_RIFNOM_RAD2_ANG10_R5.txt"));
    let (prj, qs) = run(&feats[&0], &neighbors, &tris, &tris_next);
    println!("{}", Array::from(prj));
    println!("{}",  Array::from(qs));
}