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