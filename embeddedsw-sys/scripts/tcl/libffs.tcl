# bourd support packageの設定
platform active plt
bsp setlib -name xilffs -ver 4.5
bsp config use_lfn "1"
bsp config enable_exfat "true"
bsp config enable_multi_partition "true"
bsp config num_logical_vol "10"
bsp config stdout "psu_uart_1"
bsp write
catch {bsp regenerate}
platform generate