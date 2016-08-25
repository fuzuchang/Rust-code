use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

fn main() {
    //创建一个文件路径
    let p = Path::new("Cargo.toml");
    let display = p.display();

    if p.exists() {
            let  fname = p.file_name();
            println!("{} 文件存在",display );
            println!("{:?}",fname );
            //打开只读文件
            let mut file = match File::open(&p) {
                Err(why) => panic!("不能打开{}:{}",display,Error::description(&why)),
                Ok(file) => file,
            };

            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("不能读取{}:{}",display,Error::description(&why)),
                Ok(_) => println!("{} 包含 {}", display, s ),
            }


    }else{
            println!("{} 文件不存在", display );


    }





}
