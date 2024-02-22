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

// ----------------------------------------------------------------

use std::thread;
use std::time::Duration;

use crate::threadpools::{self, Runnable};

pub fn on_test() {
    multi_thread_task();
}

fn multi_thread_task() {
    for i in 0..10 {
        threadpools::execute(move || {
            let worker = TaskRunnable { id: i };
            worker.run();
        });
    }

    thread::sleep(Duration::from_secs(15));
}

pub struct TaskRunnable {
    id: u64,
}

impl Runnable for TaskRunnable {
    fn run(&self) {
        long_task(self.id, (self.id + 1))
    }
}

fn long_task(id: u64, sec: u64) {
    println!("Task {} is running", id);
    thread::sleep(Duration::from_secs(sec));
    println!("Task {} is ending", id);
}
