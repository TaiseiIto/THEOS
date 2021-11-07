#!/bin/sh

printf "\t.set\tvolume_serial_number,\t0x%x\n" $(date +%s) > volume_serial_number.s

