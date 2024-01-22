use rust_hdl::core::check_error::check_all;
use rust_hdl::core::prelude::*;
use rust_hdl::fpga::toolchains::ecp5::generate_lpf;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::str::FromStr;

fn save_stdout(output: Output, dir: &PathBuf, basename: &str) -> Result<(), std::io::Error> {
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let mut out_file = File::create(dir.clone().join(format!("{}.out", basename)))?;
    write!(out_file, "{}", stdout)?;
    let mut err_file = File::create(dir.clone().join(format!("{}.err", basename)))?;
    write!(err_file, "{}", stderr)?;
    Ok(())
}

pub fn generate_bitstream<U: Block>(mut uut: U, prefix: &str) {
    uut.connect_all();
    check_all(&uut).unwrap();
    let verilog_text = generate_verilog(&uut);
    let lpf_text = generate_lpf(&uut);
    let dir = PathBuf::from_str(prefix).unwrap();
    let _ = remove_dir_all(&dir);
    let _ = create_dir_all(&dir);
    let mut v_file = File::create(dir.join("top.v")).unwrap();
    write!(v_file, "{}", verilog_text).unwrap();
    let lpf_filename = "top.lpf".to_string();
    let mut lpf_file = File::create(dir.join(lpf_filename)).unwrap();
    write!(lpf_file, "{}", lpf_text).unwrap();
    let output = Command::new("yosys")
        .current_dir(dir.clone())
        .arg(r#"-p synth_lattice -family xo2 -top top -json top.json"#)
        .arg("top.v")
        .output()
        .unwrap();
    save_stdout(output, &dir, "yosys_synth").unwrap();
    let output = Command::new("nextpnr-machxo2")
        .current_dir(dir.clone())
        .args([
            "--device",
            "LCMXO2-4000HC-4MG132C",
            "--textcfg",
            "top.cfg",
            "--lpf",
            "top.lpf",
            "--json",
            "top.json",
        ])
        .output()
        .unwrap();
    save_stdout(output, &dir, "nextpnr").unwrap();
    let output = Command::new("ecppack")
        .current_dir(dir.clone())
        .args([
            "-v",
            "--jed",
            "top.jed",
            "--jed-note",
            "DEVICE NAME:\tLCMXO2-4000HC-4CSBGA132",
            "top.cfg",
            "top.bin",
        ])
        .output()
        .unwrap();
    save_stdout(output, &dir, "ecppack").unwrap();
}
