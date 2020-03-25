# program to generate arbitrary unicode tag sequences

This program exists for 💩︎ purporses.

## Usage

unicode-tagger requires rust; I ran on 1.42.0. You can install it by running `cargo install --path=.` in a checkout of this repo.

To generate a non-ridiculous sequence (if you can even call it that), run:

`unicode-tagger  🏴 'gbsct'` - this should print the scottish flag to stdout (if you use a utf-8 capable terminal).

To generate a more creative sequence, run:

`unicode-tagger 📼 "EVERY MORNING I WAKE UP AND OPEN PALM SLAM A VHS INTO THE SLOT. IT'S CHRONICLES OF RIDDICK AND RIGHT THEN AND THERE I START DOING THE MOVES ALONGSIDE WITH THE MAIN CHARACTER, RIDDICK. I DO EVERY MOVE AND I DO EVERY MOVE HARD. MAKIN WHOOSHING SOUNDS WHEN I SLAM DOWN SOME NECRO BASTARDS OR EVEN WHEN I MESS UP TECHNIQUE. NOT MANY CAN SAY THEY ESCAPED THE GALAXYS MOST DANGEROUS PRISON. I CAN. I SAY IT AND I SAY IT OUTLOUD EVERYDAY TO PEOPLE IN MY COLLEGE CLASS AND ALL THEY DO IS PROVE PEOPLE IN COLLEGE CLASS CAN STILL BE IMMATURE JEKRS. AND IVE LEARNED ALL THE LINES AND IVE LEARNED HOW TO MAKE MYSELF AND MY APARTMENT LESS LONELY BY SHOUTING EM ALL. 2 HOURS INCLUDING WIND DOWN EVERY MORNIng"`

## References

* [Language Tagging in Unicode Plain Text, `rfc2482`, January 1999](https://tools.ietf.org/html/rfc2482)
* [Deprecating Unicode Language Tag Characters: RFC 2482 is Historic, `rfc6082`, November 2010](https://tools.ietf.org/html/rfc6082)
* [Unicode 13.0 emoji sequences, March 2020](https://www.unicode.org/Public/emoji/13.0/emoji-sequences.txt)
