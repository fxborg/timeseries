use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt::Debug;

use chrono::prelude::{DateTime, Utc};

/// 時系列データ型
pub struct TimeSeries<T> {
    is_adding: bool,
    buffer_size: usize,
    buffer: VecDeque<(i64, DateTime<Utc>, T)>,
}

///　バリュー イテレータ
pub struct ValueIter<'a, T> {
    ts: &'a TimeSeries<T>,
    pos: usize,
}

///　アイテム イテレータ
pub struct ItemIter<'a, T> {
    ts: &'a TimeSeries<T>,
    pos: usize,
}

///　バリュー イテレータの実装
impl<'a, T> Iterator for ValueIter<'a, T> {
    /// 要素の型を定義
    type Item = &'a T;
    /// next メソッドを実装
    fn next(&mut self) -> Option<Self::Item> {
        /// １つ進める
        self.pos += 1;
        /// サイズを超えないかチェック
        let sz: usize = self.ts.buffer.len();
        if sz >= self.pos {
            ///　参照を返す
            Some(&self.ts.buffer[sz - self.pos].2)
        } else {
            None
        }
    }
}

///　アイテム イテレータの実装
impl<'a, T> Iterator for ItemIter<'a, T> {
    /// 要素の型を定義
    type Item = &'a (i64, DateTime<Utc>, T);
    /// next メソッドを実装
    fn next(&mut self) -> Option<Self::Item> {
        /// １つ進める
        self.pos += 1;
        let sz: usize = self.ts.buffer.len();
        if sz >= self.pos {
            ///　参照を返す
            Some(&self.ts.buffer[sz - self.pos])
        } else {
            None
        }
    }
}

impl<T: Debug + Clone + Copy> TimeSeries<T> {
    /// コンストラクタ
    pub fn new(size: usize) -> Self {
        TimeSeries {
            is_adding: false,
            buffer_size: max(1_usize, min(99999_usize, size)),
            buffer: VecDeque::new(),
        }
    }

    /// イテレータ1
    pub fn item_iter(&self) -> ItemIter<T> {
        ItemIter { ts: &self, pos: 0 }
    }
    /// イテレータ2
    pub fn value_iter(&self) -> ValueIter<T> {
        ValueIter { ts: &self, pos: 0 }
    }

    /// 追加できたか？
    pub fn is_adding(&self) -> bool {
        self.is_adding
    }

    /// 追加
    pub fn push(&mut self, i0: i64, t0: DateTime<Utc>, val: T) -> Result<bool, String> {
        self.is_adding = false;

        ///　末尾のデータを取得
        let updated = match self.buffer.back_mut() {
            Some(x) => {
                if x.0 == i0 && x.1 == t0 {
                    /// 新データと既存データのIDと日付が一致する場合は値だけを更新
                    x.2 = val;
                    true
                } else if i0 > x.0 && t0 > x.1 {
                    /// 新データのIDと日付が既存データより大きい（追加可能）
                    false
                } else {
                    /// 新データのIDと日付が既存データより小さい場合はエラー
                    return Err("out of range".to_string());
                }
            }
            /// 空っぽの場合（追加可能）
            None => false,
        };

        if !updated {
            /// 更新しなかった場合は追加する
            self.buffer.push_back((i0, t0, val));
            self.is_adding = true;
        }

        if self.is_adding && self.buffer.len() > self.buffer_size {
            /// 前方からバッファ上限を超えた件数を削除する。
            let delsz: usize = self.buffer.len() - self.buffer_size;
            self.buffer.drain(0..delsz);
        }
        /// 追加か更新を返す。
        Ok(self.is_adding)
    }
}
