use rudis_client::biz;
//测试字符串与字节相关的，aaa字节数为3，中国人的字节数为9
fn main(){
    let a = "中国人";
    println!("字节数{}",a.len());//len是计算byte
    println!("字节数{}",a.chars().count());//len是计算“字符”的长度
    assert_eq!(b"aaaa","aaaa".as_bytes());

    let b=String::from("bb");
    b;
    //b;//什么也不做，也会发生ownship转移
//    biz::set("a".to_string(),"123".to_string());
    let y1 = r"a\rd\rc\r";//强制不转义
    let y2 = "x\r\nz\r\n2\r\n";
    //let x: &[_] = &['\r'];
   println!("{}",y1.trim_matches(|c| c == '\r' || c == '\n'));
   println!("{}",y2.trim_matches(|c| c == '\r' || c == '\n'));


}