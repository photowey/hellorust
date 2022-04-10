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

/*
 * @Author: photowey
 * @Date: 2022-04-09 20:29:14
 * @LastEditTime: 2022-04-10 11:06:03
 * @LastEditors: photowey
 * @Description: main.rs
 * @FilePath: /hello-rust/src/main.rs
 * Copyright (c) 2022 by photowey<photowey@gmail.com>, All Rights Reserved.
 */

#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/**
 * {@code Rectangle} 方法 area()
 */
impl Rectangle {
    /**
     * 方法
     * 第一个参数为: self
     */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /**
     * 关联函数
     * 第一个参数不为: self
     */
    fn new(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum Coin {
    Penny,
    Nickkel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_coin(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1 << 1,
            Coin::Nickkel => 1 << 2,
            Coin::Dime => 1 << 3,
            Coin::Quarter => 1 << 4,
        }
    }
}

fn main() {
    // loop_fx();
    // for_fx();
    // rect_fx();
    // mod_fx();
}

fn loop_fx() {
    let mut connter = 0;

    let result = loop {
        connter += 1;
        if connter == 11 {
            break connter * 2;
        }
    };

    println!("The result is:{}", result);
}

fn for_fx() {
    let array = [1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("the value is:{}", element);
    }

    for element in (1..4).rev() {
        println!("{}", element);
    }
    println!("LEFTOFF");
}

fn var_move_fx() {
    let s1 = String::from("Hello World");
    let s2 = s1;
    // borrow of moved value: `s1`
    // println!("the move value s1:{}", s1);
}

fn rect_fx() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::new(50);

    let area_fx = area_fx(&rect);
    let area = rect.area();

    println!("the rect area is:{}", area_fx);
    println!("the rect area is:{}", area);

    // Debug 调式字符串 {:?}
    println!("the rect info is:{:?}", rect);
    // Debug 调式字符串 {:#?} - 格式化输出
    println!("the rect info is:{:#?}", rect);
    println!("the rect info is:{:#?}", rect2);
}

fn area_fx(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn enum_fx() {
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello Message"));
    let c = Message::ChangeColor(0, 255, 255);

    m.call();
}

fn mod_fx() {
    // let words = hello::greeting::say_hello(String::from("sharkchili"));
    // println!("{}", words)
}
