extern crate clap;
extern crate geo_booleanop_tests;

use clap::{App, AppSettings, Arg};
use geojson::Feature;

use geo_booleanop::boolean::BooleanOp;

use geo_booleanop_tests::compact_geojson::write_compact_geojson;
use geo_booleanop_tests::data_generators::{generate_concentric_circles, generate_grid};
use geo_booleanop_tests::helper::{
    apply_operation, convert_to_feature, extract_expected_result, load_test_case, plot_generic_test_case, xy,
    TestOperation,
};

use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    /*
    #[rustfmt::skip]
    let matches = App::new("Test case runner")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("case")
                 .required(true)
                 .help("Input file"))
        .get_matches();
    */

    let a = generate_grid(-10.0, 10.0, 0.4, 21);
    let b = generate_grid(-10.4, 10.4, 0.4, 21);

    let op = TestOperation::Xor;

    let result = apply_operation(&a, &b, op);

    write_compact_geojson(
        &[
            convert_to_feature(&a, None),
            convert_to_feature(&b, None),
            convert_to_feature(&result, Some(op)),
        ],
        "newtest_grid.geoson",
    );

    let a = generate_concentric_circles(xy(0, 0), 1.0, 10.0, 2, 8);
    let b = generate_concentric_circles(xy(5, 5), 1.0, 10.0, 20, 100);

    let op = TestOperation::Xor;

    let result = apply_operation(&a, &b, op);

    write_compact_geojson(
        &[
            convert_to_feature(&a, None),
            convert_to_feature(&b, None),
            convert_to_feature(&result, Some(op)),
        ],
        "newtest_circle.geoson",
    );

    plot_generic_test_case(&"newtest_circle.geoson");
    //write_testcase(&[grid1, grid2, xor], "newtest.geoson");
}
