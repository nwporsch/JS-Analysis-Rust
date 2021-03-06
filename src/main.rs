/*
MIT License

Copyright (c) 2020 nwporsch

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

 */

use std::env;
use std::fs;

fn main() {
    //Get arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = read_file(filename); //Passes the pointer to the filename

    println!("{}", data);
}

/**
 * read_file: When given a file name it will attempt to read from the file and return it's contents
 */
fn read_file(filename : &String) -> String{ //Returns the last line in the function

    println!("Reading from file: {}", filename);

    //Reading from file and displaying contents
    let data = fs::read_to_string(filename)
        .expect("Unable to read file.");
    data
}
