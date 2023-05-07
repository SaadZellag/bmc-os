#!/usr/bin/sh
killall qemu-system-i386
qemu-system-i386 -drive format=raw,file=bmc-os.img &
sleep 0.1
vncviewer :5900