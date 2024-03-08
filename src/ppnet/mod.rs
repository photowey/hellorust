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

extern crate ifcfg;

use std::error::Error;
use std::fmt;

use ifcfg::IfCfg;

#[derive(Debug)]
pub struct InterfaceError(String);

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InterfaceError {}

fn ifcfg_local_interfaces() -> Result<Vec<u8>, Box<dyn Error>> {
    let interfaces = match IfCfg::get() {
        Ok(interfaces) => interfaces,
        Err(err) => return Err(Box::new(InterfaceError(format!("IfCfgError: {:?}", err)))),
    };

    let mac_bytes = interfaces
        .iter()
        .find(|interface| !interface.name.contains("Loopback"))
        .map(|interface| {
            interface
                .mac
                .split('-')
                .map(|hex| u8::from_str_radix(hex, 16))
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|err| Box::new(err) as Box<dyn Error>)
        })
        .ok_or_else(|| Box::new(InterfaceError("Loopback not found".to_string())))??;

    Ok(mac_bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ifcfg_local_interfaces() {
        let octets = ifcfg_local_interfaces().unwrap();
        println!("the interface mac bytes is: {:?}", octets);

        // @formatter:off
        let index = octets.len() - 2;
        let x = ((0x000000FF & (octets[index] as u64))
            | (0x0000FF00 & ((octets[index + 1] as u64) << 8)));
        // @formatter:on
        println!("A | B is: {:?} - {:064b}", x, x);
        let id = x >> 6;

        println!("the interface id is: {:?} -  - {:064b}", id, id);
        println!("----------------------------------------------------------------");
    }
}
