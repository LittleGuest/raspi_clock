use anyhow::Result;
use sled::{self, Db};

const DISPLAY0_KEY: &str = "data_0";
const DISPLAY1_KEY: &str = "data_1";
const DISPLAY2_KEY: &str = "data_2";
const DISPLAY3_KEY: &str = "data_3";

lazy_static! {
    static ref DB: Db = sled::open(".max7219/max7219").unwrap();
}

/// 数据存储
pub struct Store;

impl Store {
    /// 获取
    pub fn get() -> Result<Vec<Vec<u8>>> {
        let mut res = vec![];
        if let Some(data) = DB.get(DISPLAY0_KEY)? {
            res.push(data.to_vec());
        }
        if let Some(data) = DB.get(DISPLAY1_KEY)? {
            res.push(data.to_vec());
        }
        if let Some(data) = DB.get(DISPLAY2_KEY)? {
            res.push(data.to_vec());
        }
        if let Some(data) = DB.get(DISPLAY3_KEY)? {
            res.push(data.to_vec());
        }
        Ok(res)
    }

    /// 插入
    pub fn insert(data: &[Vec<u8>]) -> Result<()> {
        DB.insert(DISPLAY0_KEY, data[0].clone())?;
        DB.insert(DISPLAY1_KEY, data[1].clone())?;
        DB.insert(DISPLAY2_KEY, data[2].clone())?;
        DB.insert(DISPLAY3_KEY, data[3].clone())?;
        Ok(())
    }
}
