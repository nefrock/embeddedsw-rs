set project_name bsp
set xsa_file [lindex $argv 0]
set out_dir "./build/"

# settings of work space
setws -switch $out_dir

# settings of platform
platform create -name $project_name\
-hw $xsa_file\
-fsbl-target {psu_cortexr5_0} -out $out_dir
platform write

# settings of domain
domain create -name {standalone_psu_cortexr5_0} -display-name {standalone_psu_cortexr5_0} -os {standalone} -proc {psu_cortexr5_0} -runtime {cpp} -arch {32-bit} -support-app {hello_world}
domain active {zynqmp_fsbl}
domain active {zynqmp_pmufw}
domain active {standalone_psu_cortexr5_0}

# generate platform
platform active $project_name
platform generate -domains standalone_psu_cortexr5_0,zynqmp_fsbl,zynqmp_pmufw  
platform generate

