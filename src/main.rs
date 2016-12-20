use std::io::Read;
use std::fs::File;

fn get_file (name: &str) -> File  {
    match File::open(name) {
        Ok(v) => v,
        Err(err) =>
            panic!("Did some shit and it didn't work: {}", err)
    }

}

fn main() {
    let mut f = get_file("test/security.png");

    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Ok(size) => size,
        Err(err) =>
            panic!("Tried to read entire file and gagged: {}", err)
    };

    //I 73 H 72 D 68 R 82
    // 4 bytes width
    // 4 bytes height
    //
    let length = buffer.len();
    let header = buffer.split_off(8);

    assert_eq!(header, [137, 80, 78, 71, 13, 10, 26, 10]);

    for x in &buffer {
        println!("{}", x);
    }
}
