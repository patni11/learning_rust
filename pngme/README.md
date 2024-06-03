The program has CLI support, to run it use the following commands

    Encode: cargo run --package pngme --bin pngme -- encode --file-path dice.png --chunk-type FrSt --message "I am the last chunk" --output-file dice2.png

The above command runs our package called pngme and runs the encode command.
It takes 4 arguments: image_path, chunk_type, message and output-file.
The command will encode the message inside the png file and save it as output file. From the outside, or looking at the image, you won't be able to tell if there's any message

    Decode: cargo run --package pngme --bin pngme -- decode --file-path dice2.png --chunk-type FrSt

You can run the above command to decode the png file and get the message

    Remove: cargo run --package pngme --bin pngme -- remove --file-path dice.png --chunk-type FrSt

Remove will allow you to remove a chunk or message from the png file

    Print: cargo run --package pngme --bin pngme -- pring --file-path dice.png

Print will convert the png to a string and output to console (not very useful unless your are debugging something)
