/*
 * @Author: photowey
 * @Date: 2022-04-10 16:46:38
 * @LastEditTime: 2022-04-11 14:06:39
 * @LastEditors: weichangjun
 * @Description: grammar.rs
 * @FilePath: /hellorust/src/grammar/grammar.rs
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

/**
 1.Crate 类型
- binary
- library

2.Crate Root
- 是源代码文件
- Rust 编译器从这里开始, 组成 Crate 的根 Module

3.Package
- 包含 1 个 Cargo.toml, 它表述了如何构建这写 Crates
- 只能包含 0-1 个 library crate
- 可以包含任意数量的 binary crate
- 但必须至少包含一个 crate (library, binary)

- 4.所有权
- Rust 中的每一个值都有一个 所有者（owner）。
- 值在任一时刻有且只有一个所有者。
- 当所有者（变量）离开作用域，这个值将被丢弃。
 */
use crate::hello;

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
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_coin(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1 << 1,
            Coin::Nickel => 1 << 2,
            Coin::Dime => 1 << 3,
            Coin::Quarter => 1 << 4,
        }
    }
}

fn loop_fx() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 11 {
            break counter * 2;
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
    let words = hello::greeting::say_hello(String::from("sharkchili"));
    println!("{}", words)
}
