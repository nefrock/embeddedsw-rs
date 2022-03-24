use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    // Get XSA file path
    // let xsa_path = env!("XSA_PATH");
    let xsa_path = "../../xilinx-rust/xsa_files/zcu104.xsa";

    // // Generate bsp
    let _status = Command::new("xsct")
        .args(["./scripts/tcl/platform.tcl", &xsa_path])
        .status()
        .expect("Failed to build a bsp");

    // Get a sysroot path of armr5-none-eabi-gcc
    let sysroot = Command::new("armr5-none-eabi-gcc")
        .arg("--print-sysroot")
        .output()
        .expect("Failed to execute \"armr5-none-eabi-gcc --print-sysroot\"");

    let sysroot_path = format!(
        "{}/usr/include",
        String::from_utf8(sysroot.stdout).unwrap().trim()
    );
    println!("{:?}", sysroot_path);

    // Parse spfm file
    let xspfm_path =
        Path::new(&"./build/bsp/export/bsp/sw/bsp.spfm");
    let xspfm = XSpfm::parse(xspfm_path);

    // Get a bsp inlcude path
    let bsp_include_path = xspfm.bsp_include_path;

    // Generate Rust bindings
    let bind_builder = bindgen::Builder::default()
        .clang_args(["-target", "armv7r-none-eabihf"])
        .header("wrapper.h")
        .clang_args([
            "-I",
            &sysroot_path,
            "-I",
            &bsp_include_path,
        ])
        .blocklist_file("*/stdio.h")
        .blocklist_file("*/ctype.h")
        .blocklist_file("*/string.h")
        // .blocklist_file("*/stdarg.h")
        // .blocklist_file("*/xil_types.h")
        // .blocklist_file("*/bspconfig.h")
        // .blocklist_file("*/xparameters.h")
        // .blocklist_file("*/xparameters_ps.h")
        .use_core()
        .ctypes_prefix("cty")
        .derive_copy(false)
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate()
        .expect("Failed to generate bindings");

    let out_path =
        PathBuf::from(env::var("OUT_DIR").unwrap());
    bind_builder
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write biindings");
}

struct XSpfm {
    bsp_include_path: String,
}

impl XSpfm {
    pub fn parse(path: &Path) -> XSpfm {
        use xml::reader::{EventReader, XmlEvent};

        let mut xspfm = XSpfm {
            bsp_include_path: "".to_string(),
        };

        let file = File::open(path).unwrap();
        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    ..
                }) if name.local_name == "os" => {
                    for attr in attributes {
                        if attr.name.local_name
                            == "bspIncludePaths"
                        {
                            xspfm.bsp_include_path = format!(
                                "{}/{}",
                                path.parent()
                                    .unwrap()
                                    .display(),
                                attr.value
                            );
                        }
                    }
                }
                Err(e) => println!("{}", e),
                _ => {}
            }
        }
        xspfm
    }
}
