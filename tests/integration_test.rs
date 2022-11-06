/*
 * @Author: photowey
 * @Date: 2022-04-10 16:17:09
 * @LastEditTime: 2022-04-10 16:44:21
 * @LastEditors: photowey
 * @Description: integration_test.rs
 * @FilePath: /hellorust/tests/integration_test.rs
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

use hellorust;
use hellorust::structt::structor;

mod common;

#[test]
fn test_add_two() {
    assert_eq!(4, hellorust::add_two(2));

    // $ cargo test --test integration_test
}

#[test]
fn test_common_fn() {
    assert_eq!(1, common::helpers::setup());

    // $ cargo test --test integration_test
    // $ cargo test --test integration_test -- --show-output
}

#[test]
fn test_first_world() {
    let word1 = String::from("Hello world");
    // crate::hellorust
    let hello2 = crate::hellorust::slicee::slicer::first_world(&word1);
    let hello3 = hellorust::slicee::slicer::first_world_slice(&word1[..]);
    assert_eq!(("Hello"), hello2);
    assert_eq!(("Hello"), hello3);

    let word2 = "Hello world";
    let hello4 = hellorust::slicee::slicer::first_world_slice(word2);
    assert_eq!(("Hello"), hello4);
}

/**
std::fmt::Display
std::fmt::Debug
#[derive(Debug)]
/// # ---- debug 调试模式
{:?}
{:#?}
 */
#[test]
fn test_structor() {
    let rect = structor::Rectangle {
        width: 30,
        height: 50,
    };

    // 打印结构体
    println!("{:#?}", rect);

    let area = structor::area(&rect);
    assert_eq!(1500, area);

    // 调用方法
    let area2 = rect.area();
    assert_eq!(1500, area2);

    // 调用函数
    let square = structor::Rectangle::square(20);
    let yes = rect.can_hold(&square);
    assert!(yes);
}

#[test]
fn test_generic() {
    let str_list = vec![String::from("Hello"), String::from("World")];
    let result = hellorust::generic::generic::largest(&str_list);

    println!("The largest word is {}", result);

    assert_eq!("World", result);
}
