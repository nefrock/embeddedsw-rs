use std::{fs::File, io::BufReader, path::Path};

fn main() {
    // Parse spfm file
    let xspfm_path = Path::new(
        &"./embeddedsw-sys/build/bsp/export/bsp/sw/bsp.spfm",
    );
    let xspfm = XSpfm::parse(xspfm_path);

    // link static library
    println!(
        "cargo:rustc-link-search=native={}",
        "../embeddedsw-rs/embeddedsw-sys/build/bsp/export/bsp/sw/bsp/standalone_psu_cortexr5_0/bsplib/lib"
    );
    println!(
        "cargo:rustc-link-arg=-Wl,--start-group,-lc,-lgcc,-lxil,-end-group"
    );
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
                        } else if attr.name.local_name
                            == "bspLibraryPaths"
                        {
                            xspfm.bsp_lib_path = format!(
                                "{}/{}",
                                path.parent()
                                    .unwrap()
                                    .display(),
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
