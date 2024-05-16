use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("This is the rust version where each char are on 4 Bytes");
    let _ = rustchars();

    println!("This is the head version where each char are on 1 Byte");
    let _ = headchars();
    
    println!("This is the head version where each char are on 1 Byte using String instead of Vector");
    let _ = orheadchars();
}

fn rustchars() -> io::Result<()> {

    let mut f = File::open("one.txt")?;
    let mut contents = String::new();
    let _ = f.read_to_string(&mut contents);
    let v: Vec<char> = contents.chars().collect();
    let max_char = 1;
    let mut i = 0;
    let mut mychar = String::new();
    for c in v {
        mychar.push(c);
        i += 1;
        if i == max_char {
            break
        }
    }
    println!("This is the first \"{}byte(s)\" ", max_char);
    println!("{}\n", mychar);

    Ok(())
}


fn headchars() -> io::Result<()> {

    let mut f = File::open("one.txt")?;
    let _ = BufReader::new(&mut f);
    let max_char = 1;
    let mut i = 0;
    let mut mychar = vec![];
    for byte in f.bytes(){
        mychar.push(byte.unwrap());
        i += 1;
        if i == max_char {
            break
        }
    }
    println!("This is the first {}byte(s) ", max_char);
    println!("{}\n",String::from_utf8_lossy(&mychar));

    Ok(())
}

fn orheadchars() -> io::Result<()> {

    let mut f = File::open("one.txt")?;
    let _ = BufReader::new(&mut f);
    let max_char = 1;
    let mut i = 0;
    let mut mychar = String::new();
    for byte in f.bytes(){
        let byte_to_char = char::from(byte.unwrap());
        mychar.push(byte_to_char);
        i += 1;
        if i == max_char {
            break
        }
    }
    println!("This is the first {}byte(s) ", max_char);
    println!("{}\n",&mychar);




    Ok(())
}
