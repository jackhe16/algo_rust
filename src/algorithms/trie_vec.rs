//! # 字典树 (前缀树)
//!
//! ## 算法描述
//! - Vec作为底层结构
//!
//! ## 算法思路
//! - children[p][j]: p代表前一字符节点编号, j为该字符字符到'a'的距离, children[p][j]代表当前字符节点编号
//!

pub struct Trie {
    children: Vec<Vec<usize>>,
    count: usize,
    word_counts: Vec<usize>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            children: vec![vec![0; 26]; 1],
            count: 0,
            word_counts: vec![0; 1],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut p = 0;
        for c in word.chars() {
            let j = c as usize - 'a' as usize;
            if self.children[p][j] == 0 {
                self.count += 1;
                self.children[p][j] = self.count;

                self.children.push(vec![0; 26]);
                self.word_counts.push(0);
            }
            p = self.children[p][j];
        }
        self.word_counts[p] += 1;
    }

    pub fn query(&self, word: &str) -> usize {
        let mut p = 0;
        for c in word.chars() {
            let j = c as usize - 'a' as usize;
            if self.children[p][j] == 0 {
                return 0;
            }
            p = self.children[p][j];
        }
        self.word_counts[p]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let mut t = Trie::new();
        t.insert("cat");
        t.insert("car");
        t.insert("cate");
        t.insert("car");
        assert_eq!(t.query("car"), 2);
        assert_eq!(t.query("cost"), 0);
    }
}
