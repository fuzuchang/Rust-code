use std::io;

fn read_input () -> io::Result<()> {

    let mut input = String::new();
    //! 控制台输入
    io::stdin().read_line(&mut input).expect("输入类型不对");
    //! 控制台输出
    println!("你输入的是：{}", input.trim() );

    Ok(())
}

fn main() {
    println!("请输入什么：" );
    
    read_input();
}
