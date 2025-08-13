#! /bin/bash

for sysdevpath in $(find /sys/bus/usb/devices/usb*/ -name dev); do
	(
		syspath="${sysdevpath%/dev}"
		devname="$(udevadm info -q name -p $syspath)"
		[[ "$devname" != "tty"* ]] && exit
		eval "$(udevadm info -q property --export -p $syspath)"
		[[ -z "$ID_USB_SERIAL_SHORT" ]] && exit
		echo "/dev/$devname - $ID_USB_SERIAL_SHORT"
	)
done

# for sysdevpath in $(find /sys/bus/usb/devices/usb*/ -name dev); do
#   (
#     syspath="${sysdevpath%/dev}"
#     devname="$(udevadm info -q name -p $syspath)"
#     echo "$devname"
#     eval "$(udevadm info -q property --export -p $syspath)"
#     [[ -z "$ID_USB_SERIAL_SHORT" ]] && exit
#     echo "/dev/$devname - $ID_USB_SERIAL_SHORT"
#   )
# done
