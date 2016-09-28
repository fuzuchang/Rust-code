mod aaa {
    const X: i32 = 10;

    pub  fn print_aaa() {
        println!("{}", 42);
    }


    mod BBB {
       pub  fn print_bbb() {
           println!("{}", 37);
       }
   }


}
