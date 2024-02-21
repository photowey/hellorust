/*
 * @Author: photowey
 * @Date: 2022-04-10 16:28:50
 * @LastEditTime: 2022-04-10 16:55:39
 * @LastEditors: photowey
 * @Description: main.rs
 * @FilePath: /hellorust/src/main.rs
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

// ----------------------------------------------------------------

use crate::threadpool::{Callable, Runnable};

mod cmd;
mod threadpool;

// ----------------------------------------------------------------

// ----------------------------------------------------------------

struct HelloRunnable;

impl Runnable for HelloRunnable {
    fn run(&self) {
        println!("Hello from Runnable");
    }
}

struct HelloCallable;

impl Callable<i32> for HelloCallable {
    fn call(&self) -> i32 {
        42
    }
}

// ----------------------------------------------------------------

fn main() {
    cmd::cmder::run();

    println!("----|------------------------------------------------------------");
    run_thread_pool();
    println!("----|------------------------------------------------------------");
    multi_thread_call();
    println!("----|------------------------------------------------------------");
    multi_thread_fx();
}

fn run_thread_pool() {
    threadpool::THREAD_POOL.execute(|| {
        let worker = HelloRunnable;
        worker.run();
    });

    let result_receiver = threadpool::THREAD_POOL.submit(|| {
        let worker = HelloCallable;
        worker.call()
    });

    let result = result_receiver.recv().unwrap();
    println!("Result from Callable: {}", result);
}

fn multi_thread_call() {
    threadpool::thread_pool()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .execute(|| {
            let worker = HelloRunnable;
            worker.run();
        });

    let result_receiver = threadpool::thread_pool()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .submit(|| {
            let worker = HelloCallable;
            worker.call()
        });

    let result = result_receiver.recv().unwrap();
    println!("multi: result from Callable: {}", result);
}

fn multi_thread_fx() {
    threadpool::execute(|| {
        let worker = HelloRunnable;
        worker.run();
    });

    let result_receiver = threadpool::submit(|| {
        let worker = HelloCallable;
        worker.call()
    });

    let result = result_receiver.recv().unwrap();
    println!("multi: result from Callable: {}", result);
}
