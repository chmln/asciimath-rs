use super::super::DEBUG;

pub fn log(msg: String) -> () {
    if DEBUG == true {
        println!("{}", msg);
    }
}
