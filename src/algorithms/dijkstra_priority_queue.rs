//! # 单源最短路算法(有向非负权图)
//!
//! 优先队列版本

use std::collections::BinaryHeap;

pub struct DijkstraPQ {
    graph: Vec<Vec<(usize, usize)>>,
    n: usize,
    /** 源点到该点的最小权值和 */
    min_dist: Vec<usize>,
    parent: Vec<usize>,
    s: usize,
}

impl DijkstraPQ {
    pub fn new(n: usize) -> Self {
        DijkstraPQ {
            graph: vec![vec![]; n + 1],
            n,
            min_dist: vec![usize::MAX; n + 1],
            parent: vec![0; n + 1],
            s: 0,
        }
    }

    pub fn add_edge(&mut self, s: usize, t: usize, w: usize) {
        self.graph[s].push((t, w));
    }

    /// 为何只能用于非负权值: 负权值存在时, 权值和累加过程中不能保证单调性
    pub fn dijkstra(&mut self, s: usize, t: usize) -> usize {
        self.s = s;

        let mut used = vec![false; self.n + 1];
        self.min_dist[s] = 0;

        let mut pq = BinaryHeap::new();
        pq.push((s, 0));

        while !pq.is_empty() {
            // 寻找min_dist中最小值
            let (prev, _) = pq.pop().unwrap();

            // 标记访问
            if used[prev] {
                continue;
            }
            used[prev] = true;

            // 更新min_dist
            for i in 0..self.graph[prev].len() {
                let (next, nextw) = self.graph[prev][i];
                if !used[next] && self.min_dist[prev] + nextw < self.min_dist[next] {
                    self.min_dist[next] = self.min_dist[prev] + nextw;
                    self.parent[next] = prev;
                    pq.push((next, self.min_dist[next]));
                }
            }
        }

        self.min_dist[t]
    }

    pub fn print_min_dist(&self) {
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
    fn test_dijkstra() {
        let mut dij = DijkstraPQ::new(5);
        dij.add_edge(1, 2, 4);
        dij.add_edge(1, 3, 1);
        dij.add_edge(1, 4, 3);
        dij.add_edge(2, 3, 2);
        dij.add_edge(2, 5, 2);
        dij.add_edge(3, 5, 2);

        let res = dij.dijkstra(1, 5);
        println!("{}", res);

        dij.print_min_dist();
    }
}
