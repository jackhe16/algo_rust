//! 单源最短路径(有向有权图)
//!
//! - 从起点出发, 相连边的顶点min_dist有变化时该点入队, 直至队列为空,
//! - 当存在负环时
//!   - 判断负环: 记录所有点的入队次数, 若存在某点超过n-1次, 代表成环, 存在负环。
//!   - 最短路的解是无限小, 队列死循环, 且后续的min_dist结果是错的, 后续及之后的正确结果需要在遍历边前复制min_dist作为副本用来判断
//!     - let min_dist_copy = self.min_dist.clone();
//!     - if min_dist_copy[vs] + w < self.min_dist[vt]
//! - 优化: 每一轮松弛, 相同节点不用重复放入队列，但需要重复松弛

use std::collections::VecDeque;

pub struct BellmanFord {
    graph: Vec<Vec<(usize, i32)>>,
    n: usize,
    /** 源点到该点的最小权值和 */
    min_dist: Vec<i32>,
    parent: Vec<usize>,
    s: usize,
    // 负环标记
    flag: bool,
}

impl BellmanFord {
    pub fn new(n: usize) -> Self {
        BellmanFord {
            graph: vec![vec![]; n + 1],
            n,
            min_dist: vec![i32::MAX; n + 1],
            parent: vec![0; n + 1],
            s: 0,
            flag: false,
        }
    }

    pub fn add_edge(&mut self, s: usize, t: usize, w: i32) {
        self.graph[s].push((t, w));
    }

    pub fn bellman_ford(&mut self, s: usize, t: usize) -> i32 {
        self.s = s;

        let mut que = VecDeque::new();
        let mut counts = vec![0; self.n + 1];

        self.min_dist[s] = 0;
        que.push_back(s);
        counts[s] += 1;

        'outer: while !que.is_empty() {
            let mut visited = vec![false; self.n + 1];
            let que_len = que.len();

            for _ in 0..que_len {
                let vs = que.pop_front().unwrap();
                for j in 0..self.graph[vs].len() {
                    let (vt, w) = self.graph[vs][j];
                    if self.min_dist[vs] + w < self.min_dist[vt] {
                        self.min_dist[vt] = self.min_dist[vs] + w;
                        self.parent[vt] = vs;

                        // 判断负环: 记录所有点的入队次数, 若存在某点超过n-1次, 存在负环。
                        counts[vt] += 1;
                        if counts[vt] == self.n {
                            self.flag = true;
                            que.clear();
                            break 'outer;
                        }

                        // 每一轮松弛, 相同节点不用重复放入队列，但需要重复松弛
                        if visited[vt] {
                            continue;
                        }
                        visited[vt] = true;
                        que.push_back(vt);
                    }
                }
            }
        }

        self.min_dist[t]
    }

    pub fn print_min_dist(&self) {
        if self.flag {
            println!("存在负环,最短路无限小");
            return;
        }

        for i in 2..self.n + 1 {
            let wsum = self.min_dist[i];
            println!(
                "(source .. parent -> target: wsum) {} .. {} -> {}: {}",
                self.s, self.parent[i], i, wsum
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        let mut bf = BellmanFord::new(5);
        bf.add_edge(1, 2, 4);
        bf.add_edge(1, 3, 1);
        bf.add_edge(1, 4, 3);
        bf.add_edge(2, 3, 2);
        bf.add_edge(2, 5, 2);
        bf.add_edge(3, 5, 2);

        let res = bf.bellman_ford(1, 5);
        println!("{}", res);
        assert_eq!(res, 3);

        bf.print_min_dist();
    }

    #[test]
    fn test_bellman_ford_negative() {
        let mut bf = BellmanFord::new(5);
        bf.add_edge(1, 2, 4);
        bf.add_edge(1, 3, 1);
        bf.add_edge(1, 4, 3);
        bf.add_edge(2, 3, 2);
        bf.add_edge(2, 5, -2);
        bf.add_edge(3, 5, 2);

        let res = bf.bellman_ford(1, 5);
        println!("{}", res);
        assert_eq!(res, 2);

        bf.print_min_dist();
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut bf = BellmanFord::new(4);
        bf.add_edge(1, 2, 1);
        bf.add_edge(1, 3, 1);
        bf.add_edge(1, 4, 1);
        bf.add_edge(2, 4, 1);
        bf.add_edge(3, 2, 3);
        bf.add_edge(3, 4, 1);
        bf.add_edge(4, 3, -6);

        let res = bf.bellman_ford(1, 4);
        println!("{}", res);

        bf.print_min_dist();
    }
}
