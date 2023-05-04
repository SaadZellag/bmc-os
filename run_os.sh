#!/usr/bin/sh
killall qemu-system-x86_64
qemu-system-x86_64 -drive format=raw,file=bmc-os.img &
sleep 0.1
vncviewer :5900