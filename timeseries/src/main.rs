extern crate timeseries;
extern crate chrono;

use timeseries::TimeSeries;
use chrono::prelude::*;

/// 任意のデータ型を定義する。
#[derive(Debug,Clone,Copy)]
struct HL{
    high:f64,
    low:f64,
}

pub fn main(){
    /// サイズ上限５件のTimeSeries<HL>インスタンスを生成
    let mut ts=<TimeSeries<HL>>::new(5);
    let mut _a;
    ///　適当な時系列データを追加
    _a = ts.push(1,Utc.ymd(2019, 3, 1).and_hms(18, 00, 00), HL{high:3.0,low:2.0});
    _a = ts.push(2,Utc.ymd(2019, 3, 2).and_hms(18, 00, 00), HL{high:3.1,low:2.3});
    _a = ts.push(3,Utc.ymd(2019, 3, 3).and_hms(18, 00, 00), HL{high:3.2,low:2.4});
    _a = ts.push(4,Utc.ymd(2019, 3, 4).and_hms(18, 00, 00), HL{high:3.3,low:2.5});
    _a = ts.push(5,Utc.ymd(2019, 3, 5).and_hms(18, 00, 00), HL{high:3.4,low:2.6});
    _a = ts.push(6,Utc.ymd(2019, 3, 6).and_hms(18, 00, 00), HL{high:3.5,low:2.7});
    _a = ts.push(7,Utc.ymd(2019, 3, 7).and_hms(18, 00, 00), HL{high:3.6,low:2.8});
    _a = ts.push(8,Utc.ymd(2019, 3, 8).and_hms(18, 00, 00), HL{high:3.7,low:2.9});
    _a = ts.push(9,Utc.ymd(2019, 3, 9).and_hms(18, 00, 00), HL{high:3.8,low:2.11});

    /// 要素を全件出力
    for v in ts.item_iter(){
        println!("{:?}",&v);
    }
    /// 値のみ全件出力
    for v in ts.value_iter(){
        println!("{:?}",&v);
    }

}
