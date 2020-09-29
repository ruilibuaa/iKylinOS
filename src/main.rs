//2020-09-29
/*
 * Author：LIRUI
 * E-mail：lir@pcl.ac.cn
 * 功能实现：在main()中周期性对标志位进行周期性翻转，置0置1
 *
*/

pub fn delay(mut count:i32) {
    while count > 0 {
        count = count-1;
    }
}

fn main() {
    loop {
        let mut flag = 0;
        delay(100);
        flag = 1;
        delay(100);
    }
}