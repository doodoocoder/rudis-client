//测试 str 的copy特性,String默认是没有copy特性的
fn main() {
    //str();
  //  String();
    let a:String;

    let a ="bb";//str 的copy特性
    let d = a;
    println!("{}",a);

    /*let b = foo("aa");//String默认是没有copy特性的
    let c = b;
    println!("{}",b);*/

    let f = String::from("ff");
    foo_String(f);
   // foo_String(f);//use moved value 和实现无关，只要调用函数就会发生ownership转移

    let e = foo("aa");//为什么可以连续print而不会报错？print不会发生ownership 转移
    println!("{}",e);
    println!("{}",e);




}

/*fn null(st:&String) -> String{
   if Some(st) = st{
       st
   }
}*/

fn str() {
    let a = "aaa";
    let _b = a;//编译器要求用_
    println!("{}", a);//可以打印说明str 有copy特性
}

fn String() {
    let a = String::from("aaa");//放堆上的，默认没有实现copy trait
      let  _b = &*a; //这样可以通过
  //  let _b = a;//编译器推断这个应该为放内存上的,这样不能通过

    println!("{}", a);
}
///故意这样写，不然编译器也会编译，虽然不在main中
#[cfg(target_os="linux")]
fn foo_str(x: &str) -> &str {
    let  a = "Hello".to_string() + x;//从ide可看出这个a 最终被编译器推断出为 String
    &*a//这样不能通过 `a` is borrowed here returns a value referencing data owned by the current function

}
fn foo_String(x:String) ->String{
    x
}

fn foo(x: &str) -> String {
    let a = "Hello".to_string() + x;//从ide可看出这个a 最终被编译器推断出为 String
    a
}
fn foo_s() -> &'static str{
    let a ="aaa";
    &a
}