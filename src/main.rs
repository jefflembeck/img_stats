use std::io::Read;
use std::fs::File;

#[test]
fn test_is_png () {
    let png = get_file("./test/security.png");
    let jpg = get_file("./test/red.jpg");
    assert!(is_png(png), "This is a PNG");
    assert!(!is_png(jpg), "This is not a PNG");
}

fn get_file (name: &str) -> File {
    match File::open(name) {
        Ok(v) => v,
        Err(err) =>
            panic!("Did some shit and it didn't work: {}", err),
    }
}

fn is_png (mut filename: File) -> bool {
    let mut buffer = vec![0; 8];
    match filename.read_exact(&mut buffer) {
        Ok(size) => size,
        Err(err) =>
            panic!("This has less than 8 bytes: {}", err),
    };

    buffer == [137, 80, 78, 71, 13, 10, 26, 10]
}

fn main() {
    let mut file = get_file("./test/security.png");
    let mut file2 = get_file("./test/security.png");

    if is_png(file) {
        let mut buffer = Vec::new();
        match file2.read_to_end(&mut buffer) {
            Ok(size) => size,
            Err(err) =>
                panic!("This was bad and ugh: {}", err),
        };

        let mut iter = buffer.chunks(4);
        for chunk in iter {
            if chunk == [73, 72, 68, 82] {
                let width = iter.next();
            }
        }
   //     file.

       // for every 8 bytes
       //     examine bytes
       //         when i hit
                    //I 73 H 72 D 68 R 82
        //            grab the deets from the next sets of bytes
    } else {
        panic!("NO DICE");
    }

    //I 73 H 72 D 68 R 82
    // 4 bytes width
    // 4 bytes height
    //
}

