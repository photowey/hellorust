/*
 * @Author: photowey
 * @Date: 2022-04-10 11:03:24
 * @LastEditTime: 2022-04-10 16:19:19
 * @LastEditors: photowey
 * @Description: lib.rs
 * @FilePath: /hello-rust/src/lib.rs
 * Copyright (c) 2022 by photowey<photowey@gmail.com>, All Rights Reserved.
 */

/*
 *                        _oo0oo_
 *                       o8888888o
 *                       88" . "88
 *                       (| -_- |)
 *                       0\  =  /0
 *                     ___/`---'\___
 *                   .' \\|     |// '.
 *                  / \\|||  :  |||// \
 *                 / _||||| -:- |||||- \
 *                |   | \\\  - /// |   |
 *                | \_|  ''\---/''  |_/ |
 *                \  .-\__  '-'  ___/-. /
 *              ___'. .'  /--.--\  `. .'___
 *           ."" '<  `.___\_<|>_/___.' >' "".
 *          | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *          \  \ `_.   \_ __\ /__ _/   .-` /  /
 *      =====`-.____`.___ \_____/___.-`___.-'=====
 *                        `=---='
 *
 *
 *      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 *            佛祖保佑     永不宕机     永无BUG
 *
 *        佛曰:
 *                写字楼里写字间，写字间里程序员；
 *                程序人员写程序，又拿程序换酒钱。
 *                酒醒只在网上坐，酒醉还来网下眠；
 *                酒醉酒醒日复日，网上网下年复年。
 *                但愿老死电脑间，不愿鞠躬老板前；
 *                奔驰宝马贵者趣，公交自行程序员。
 *                别人笑我忒疯癫，我笑自己命太贱；
 *                不见满街漂亮妹，哪个归得程序员？
 */

#![allow(unused)]

pub use crate::front_of_house::hosting;

pub mod closure;
pub mod generic;
pub mod grammar;
pub mod hello;
pub mod lifecycle;
pub mod slicee;
pub mod structt;
pub mod traitt;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_lists() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        pub fn take_payment() {}
    }
}

pub fn eat_a_restaurant() {
    crate::front_of_house::hosting::add_to_wait_lists();
    front_of_house::hosting::add_to_wait_lists();

    hosting::add_to_wait_lists();
}

pub fn say_hi() -> String {
    let words = hello::greeting::say_hello(String::from("sharkchili"));
    println!("{}", words);
    return words;
}

pub fn add_two(x: i32) -> i32 {
    inner_add_two(x, 2)
}

fn inner_add_two(x: i32, y: i32) -> i32 {
    x + y
}

/// -- 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hi() {
        let words = say_hi();

        assert_eq!(words, "say hello to: sharkchili");
    }

    // $ cargo test -- --show-output

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
        assert_eq!(4, inner_add_two(2, 2));
    }
}
