//! # 灌水算法 (连通分量)
//!
//! ## 应用场景
//! - 判断无向图中两顶点是否处于同一个连通分量(集合)
//!
//! ## 算法思路
//! - que: 队列,广优搜索存储相邻顶点
//! - mark: 顶点所属连通分量
//!   - 如makr[3]=1表示顶点3属于连通分量1
//!   - 为0表示还未标记,visited作用
//! - count: 连通分量标号计数,从1开始

use std::collections::VecDeque;

pub struct FloodFill {
    graph: Vec<Vec<usize>>,
    n: usize,
    mark: Vec<usize>,
    count: usize,
}

impl FloodFill {
    pub fn new(n: usize) -> Self {
        FloodFill {
            graph: vec![vec![]; n + 1],
            n,
            mark: vec![0; n + 1],
            count: 0,
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize) {
        self.graph[l].push(r);
        self.graph[r].push(l);
    }

    pub fn flood_fill(&mut self) {
        let mut que = VecDeque::new();

        // 遍历所有节点,逐个寻找未标记过的连通起点
        for v in 1..self.n + 1 {
            if self.mark[v] != 0 {
                continue;
            }

            self.count += 1;
            self.mark[v] = self.count;
            que.push_back(v);
            while !que.is_empty() {
                let cur = que.pop_front().unwrap();
                for &r in self.graph[cur].iter() {
                    if self.mark[r] == 0 {
                        self.mark[r] = self.count;
                        que.push_back(r);
                    }
                }
            }
        }
    }

    // 两顶点是否属于同一连通分量
    pub fn is_same(&self, l: usize, r: usize) -> bool {
        self.mark[l] == self.mark[r]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() {
        let mut ff = FloodFill::new(5);
        ff.add_edge(1, 2);
        ff.add_edge(1, 3);
        ff.add_edge(2, 3);
        ff.add_edge(4, 5);

        ff.flood_fill();
        assert!(ff.is_same(1, 3));
        assert!(!ff.is_same(1, 5));
    }
}
