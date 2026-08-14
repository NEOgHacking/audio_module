# audio_module
<img width="1092" height="1089" alt="image" src="https://github.com/user-attachments/assets/c517b167-ac3b-4a58-ba5d-ac5ca11dd522" />
This is a simple audio module to use headphones on the hackxpansion console, this is a simple module with a 3.5mm audio jack.

## Design
For this design i wanted to be able to connect my headphones to thee console for audio output to add an extra dimension to the games.
For this i used a PCM5102A audio chip from Texas Instruments to drive the audio output on the 3.5mm headphone jack. I tried to isolate the digital power and analog power as good as possible with a ferrite bead to prevent unwanted output from interference from the digital side.

## Schematic
This design was made in KiCad 10 and uses the standard parts. I needed to make a custom part for the audio port the PJ-342, i could use the symbol from KiCad but had to make my own footprint.
<img width="1422" height="764" alt="image" src="https://github.com/user-attachments/assets/41ea9c16-2c31-462b-b87c-63d25ca8b43e" />
This is a pretty basic schematic but it should work.

## Layout
<img width="965" height="1197" alt="image" src="https://github.com/user-attachments/assets/2bd66064-dd43-4329-be4a-5948b276633a" />
The layout was based on the hackxpansion console to fit in 1 module bay. It uses a simple 2 layer PCB with a gnd fill on top and bottom layer to make an as good as possible gnd plane for the audio chip.

## Case
The case was made in onshape using the free licence, its pretty simple and was based on an import of the PCB so referencing was easy. The PCB screws in from the bottom with 2 screws to attach it to the case.
<img width="1576" height="1047" alt="Screenshot From 2026-08-14 18-52-51" src="https://github.com/user-attachments/assets/b1d8a569-d05a-4858-8653-03e0f9436d26" />
<img width="1314" height="1042" alt="Screenshot From 2026-08-14 18-42-45" src="https://github.com/user-attachments/assets/f6f2f272-1b20-444c-bd7e-07e42d1ef310" />
Here is the [onshape link](https://cad.onshape.com/documents/23acd3255b900cc5b85fa1fc/w/7ef244823df79fb773706f50/e/46ef782e0da3cdec777ec4b8?renderMode=0&uiState=6a7f842a860458379a62dd10)

## Software
WIP
The software was made as described in the first driver tutorial. It was heavily edited to create I2S compatibility for the audio chip, this however is still not in a working state, i am working hard to get it working but it is not that easy to get working for me because i am shit in rust.

# Credits
Huge thanks to @Koeg for the YSWS and all the help with the firmware!!
