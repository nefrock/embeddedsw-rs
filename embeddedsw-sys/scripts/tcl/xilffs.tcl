# bourd support packageの設定
bsp setlib -name xilffs -ver 4.5
bsp write
catch {bsp regenerate}
