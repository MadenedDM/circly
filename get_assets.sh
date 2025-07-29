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

if [[ $1 == "-h" ]]; then
    echo "Valid commands are:"
    echo "  -h"
    echo "  anikki"
    echo "  yayo"
    echo "  yun-agm"
    echo "yayo is the default option"
elif [[ $1 == "yun-agm" ]]; then
    wget "https://dwarffortresswiki.org/images/e/e1/16x16-RogueYun-AgmEdit.png" -O "assets/game/RogueYun-AgmEdit.png" -q
elif [[ $1 == "anikki" ]]; then
    wget "https://dwarffortresswiki.org/images/d/d2/Anikki_square_16x16.png" -O "assets/game/Anikki.png" -q
elif [[ $1 == "yayo" ]]; then
    wget "https://dwarffortresswiki.org/images/2/20/Yayo_c64_1280x400_83b157.png" -O "assets/game/Yayo.png" -q
else
    wget "https://dwarffortresswiki.org/images/2/20/Yayo_c64_1280x400_83b157.png" -O "assets/game/Yayo.png" -q
fi
wget "https://dwarffortresswiki.org/images/6/63/VGA8x16.png" -O "assets/ui/Vga.png" -q