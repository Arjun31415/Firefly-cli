# Firefly-cli

CLI for managing colors and effects for CosmicByte Firefly keyboard.

## About 
This project was done by reverse engineering the firefly software on windows. On Windows, The packets were sniffed using WireShark while running the  firefly GUI software. Then packets were analysed and the logic written in rust using `rusb` crate.  

## Features 
- [x] 10 different Effects
    - [x] Setting loop/individual color of effects

- [x] 7 different colors

- [ ] Individual key paint
- [ ] Brightness control (This can already be modified via keybind Fn + {up,down} arrow)
- [ ] Speed control (This can already be modified via keybind Fn + {left,right} arrow)

## Usage 

1. Requires user to have permission to access usb devices, otherwise sudo can be used (not recommended).
Ref: this stackoverflow [link](https://stackoverflow.com/questions/22713834/libusb-cannot-open-usb-device-permission-isse-netbeans-ubuntu)
2. git clone 
3. cargo run

## Acknowledgements 
- This stackoverflow [answer](https://stackoverflow.com/questions/37943825/send-hid-report-with-pyusb/52368526#52368526) which came in clutch, while I was breaking my head over why HIDAPI was not working.
- Low Byte Productions [Video](https://www.youtube.com/watch?v=is9wVOKeIjQ)
