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

use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

pub mod threadpool;

// ----------------------------------------------------------------

// ----------------------------------------------------------------

// ----------------------------------------------------------------

lazy_static! {
    pub static ref THREAD_POOL: threadpool::ThreadPool = threadpool::ThreadPool::new(4);
}

lazy_static! {
    pub static ref THREAD_POOL_INSTANCE: Arc<Mutex<Option<threadpool::ThreadPool>>> =
        Arc::new(Mutex::new(None));
}

// ----------------------------------------------------------------

pub trait Runnable {
    fn run(&self);
}

pub trait Callable<T> {
    fn call(&self) -> T;
}

// ----------------------------------------------------------------

pub fn determine_cpu_cores() -> usize {
    num_cpus::get()
}

// ----------------------------------------------------------------

pub fn thread_pool() -> Arc<Mutex<Option<threadpool::ThreadPool>>> {
    let mut instance = THREAD_POOL_INSTANCE.lock().unwrap();
    if instance.is_none() {
        let cores = determine_cpu_cores();
        *instance = Some(threadpool::ThreadPool::new(cores));
    }
    Arc::clone(&THREAD_POOL_INSTANCE)
}

// ----------------------------------------------------------------

pub fn execute<F>(fx: F)
where
    F: FnOnce() + Send + 'static,
{
    thread_pool().lock().unwrap().as_ref().unwrap().execute(fx);
}

pub fn submit<F, R>(fx: F) -> Receiver<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    thread_pool().lock().unwrap().as_ref().unwrap().submit(fx)
}
