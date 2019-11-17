#![crate_name = "rudis_client"]//将本crate的名称设置为rudis_client，不能随便叫

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn sy() {
    println!("goooood");
}

///连接信息
pub struct Info {
    ipport: String,
    /*
        user: String,
        pwd: String,*/
}

pub mod conn {
    use std::net::TcpStream;
    use crate::Info;

    pub fn connect(info: Info) -> TcpStream {
        TcpStream::connect(info.ipport).unwrap()
    }
}

pub mod biz {
    pub fn tran(str: &str) -> &[u8] {//将字符串转换为二进制
        str.as_bytes()
    }

    /// *<参数数量> CR LF
     ///$<参数 1 的字节数量> CR LF
    /// <参数 1 的数据> CR LF
     ///...
     ///$<参数 N 的字节数量> CR LF
     ///<参数 N 的数据> CR LF
    pub fn set(mut key: String, mut val: String) -> String {//set设置
        //不需要判断为空，不初始化，编译器会报错
        //对特殊字符进行处理,将敏感字符\r\n去掉
        let x: &[_] = &['\r', '\n'];
        key = key.trim_matches(x).to_string();
        val = val.trim_matches(x).to_string();
        let mut rlt: String = String::from("*");
        let sp = "\r\n";
        let prefix = "$";
        //处理字符串：将输入字符串变成符合redis协议格式的字符串
        let paramCount = "3";//三个参数
        rlt.push_str(paramCount);
        rlt.push_str(sp);

        let methodBytes = "set".len().to_string();//计算的字节数
        rlt.push_str(prefix);
        rlt.push_str(&*methodBytes);
        rlt.push_str(sp);
        rlt.push_str("set");
        rlt.push_str(sp);


        let keyBytes = key.len().to_string();//key的字节数，不是位数。
        rlt.push_str(prefix);
        rlt.push_str(&*keyBytes);
        rlt.push_str(sp);
        rlt.push_str(&*key);
        rlt.push_str(sp);
        let valBytes = val.len().to_string();
        rlt.push_str(prefix);
        rlt.push_str(&*valBytes);
        rlt.push_str(sp);
        rlt.push_str(&*val);
        rlt.push_str(sp);

        println!("{}", rlt);
        return rlt;
        //转换字符串
    }
}


