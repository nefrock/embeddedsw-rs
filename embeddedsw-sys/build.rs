use std::{
    env,
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    // Get XSA file path
    let xsa_path = env!("XSA_PATH");
    // let xsa_path = "/home/kikemori/rust/xilinx-rust/xsa_files/zcu104.xsa";

    // Gen platform script
    let mut platform = Platform::new();
    let platform_path = Path::new("./scripts/tcl/platform.tcl");

    platform.push_feature(FeatureKind::Base);

    // extra library such as xilffs
    #[cfg(feature = "xilffs")]
    platform.push_feature(FeatureKind::Xilffs);

    platform
        .gen_tcl_scripts(platform_path)
        .expect("Failed to generate tcl script");

    // Generate bsp
    let _status = Command::new("xsct")
        .args([&platform_path.display().to_string(), &xsa_path.to_string()])
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
    let xspfm_path = Path::new(&"./build/bsp/export/bsp/sw/bsp.spfm");
    let xspfm = XSpfm::parse(xspfm_path);

    // Get a bsp inlcude path and bsp lib path
    let bsp_include_path = Path::new(&xspfm.bsp_include_path);
    let bsp_lib_path = Path::new(&xspfm.bsp_lib_path);

    // Get a path to xpseudo_asm_armclangs.h
    let xpseudo_asm_armclang_path = "./build/bsp/zynqmp_fsbl/zynqmp_fsbl_bsp/psu_cortexr5_0/libsrc/standalone_v7_5/src/arm/cortexr5/armclang/";

    // Generate Rust bindings
    let bind_builder = bindgen::Builder::default()
        .clang_args(["-target", "armv7r-none-eabihf"])
        .header("wrapper_base.h")
        .clang_args([
            "-I",
            &sysroot_path,
            "-I",
            &bsp_include_path.display().to_string(),
            "-I",
            &xpseudo_asm_armclang_path,
        ])
        .blocklist_file("*/stdio.h")
        .blocklist_file("*/ctype.h")
        .blocklist_file("*/string.h")
        // .blocklist_file("*/stdarg.h")
        // .blocklist_file("*/xil_types.h")
        // .blocklist_file("*/bspconfig.h")
        // .blocklist_file("*/xparameters.h")
        // .blocklist_file("*/xparameters_ps.h")
        // .fit_macro_constants(true)
        .use_core()
        .ctypes_prefix("cty")
        .derive_copy(false)
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        });

    #[cfg(feature = "xilffs")]
    let bind_builder = bind_builder.header("wrapper_xilffs.h");

    let bind_builder = bind_builder
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bind_builder
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write biindings");

    // Get a embedded-sys root path
    let pwd = Command::new("pwd")
        .output()
        .expect("Failed to exectute pwd command");
    let emb_sys_root_path =
        String::from_utf8(pwd.stdout).unwrap().trim().to_string();
    println!("{}", emb_sys_root_path);

    // Link static library
    println!(
        "cargo:rustc-link-search=native={}/{}",
        emb_sys_root_path,
        bsp_lib_path.strip_prefix("./").unwrap().display()
    );
    println!(
        "cargo:rustc-link-arg=-Wl,--start-group,-lc,-lgcc,-lxil,-end-group"
    );

    // re-run if build.rs is changed
    println!("cargo:rerun-if-changed=build.rs");
}

struct XSpfm {
    bsp_include_path: String,
    bsp_lib_path: String,
}

impl XSpfm {
    pub fn parse(path: &Path) -> XSpfm {
        use xml::reader::{EventReader, XmlEvent};

        let mut xspfm = XSpfm {
            bsp_include_path: "".to_string(),
            bsp_lib_path: "".to_string(),
        };

        let file = File::open(path).unwrap();
        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if name.local_name == "os" => {
                    for attr in attributes {
                        if attr.name.local_name == "bspIncludePaths" {
                            xspfm.bsp_include_path = format!(
                                "{}/{}",
                                path.parent().unwrap().display(),
                                attr.value
                            );
                        } else if attr.name.local_name == "bspLibraryPaths" {
                            xspfm.bsp_lib_path = format!(
                                "{}/{}",
                                path.parent().unwrap().display(),
                                attr.value
                            )
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

#[derive(Debug, Copy, Clone)]
enum FeatureKind {
    Base,
    Xilffs,
}

impl FeatureKind {
    fn get_path<'a>(self) -> &'a Path {
        match self {
            FeatureKind::Base => Path::new("./scripts/tcl/base.tcl"),
            FeatureKind::Xilffs => Path::new("./scripts/tcl/xilffs.tcl"),
        }
    }
}

struct Platform {
    contents: String,
}

impl Platform {
    fn new() -> Self {
        Self {
            contents: "".to_string(),
        }
    }

    fn push_feature(&mut self, feature: FeatureKind) {
        let contents = fs::read_to_string(feature.get_path()).expect(&format!(
            "Faild to open a feature {} file",
            feature.get_path().display()
        ));
        self.contents.push_str(&contents)
    }

    fn gen_tcl_scripts(&mut self, path: &Path) -> Result<(), io::Error> {
        self.contents.push_str("platform generate");

        let mut file = File::create(path)
            .expect(&format!("Failed to open {}", path.display()));
        file.write_all(self.contents.as_bytes())
    }
}
