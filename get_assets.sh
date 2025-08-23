#! /bin/bash
if [ ! -d "assets" ]; then
  mkdir "assets"
fi

if [ ! -d "assets/game" ]; then
  mkdir "assets/game"
fi

if [ ! -d "assets/ui" ]; then
  mkdir "assets/ui"
fi

wget "https://dwarffortresswiki.org/images/e/e1/16x16-RogueYun-AgmEdit.png" -O "assets/game/RogueYun-AgmEdit.png" -q
wget "https://dwarffortresswiki.org/images/d/d2/Anikki_square_16x16.png" -O "assets/game/Anikki.png" -q
wget "https://dwarffortresswiki.org/images/2/20/Yayo_c64_1280x400_83b157.png" -O "assets/game/Yayo.png" -q
wget "https://dwarffortresswiki.org/images/6/63/VGA8x16.png" -O "assets/ui/Vga.png" -q