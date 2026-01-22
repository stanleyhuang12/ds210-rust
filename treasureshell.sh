#!/bin/zsh 

mkdir treasure_hunt
cd treasure_hunt
touch command_line_scavenger_hunt.txt
whoami > command_line_scavenger_hunt.txt
hostname >> command_line_scavenger_hunt.txt
pwd >> command_line_scavenger_hunt.txt
echo $HOME >> command_line_scavenger_hunt.txt

touch clue_1.txt
echo "The treasure is hidden in plain sight" > clue_1.txt

mkdir secret_chamber
cd secret_chamber
touch clue_2.txt
echo "Look for a hidden file" > clue_2.txt

touch .treasure_map.txt
echo "Congratulations. You found the treasure"

cd .. 
cd ..

zip -r treasure_hunt.zip treasure_hunt 