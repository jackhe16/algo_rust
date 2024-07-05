//! # 单源最短路径(有向非负有权图)
//!
//!

pub struct Dijkstra {
    graph: Vec<Vec<(usize, usize)>>,
    n: usize,
    /** 源点到该点的最小权值和 */
    min_dist: Vec<usize>,
    parent: Vec<usize>,
    s: usize,
}

impl Dijkstra {
    pub fn new(n: usize) -> Self {
        Dijkstra {
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

        for _ in 1..self.n + 1 {
            // 寻找min_dist中最小值
            let mut min = usize::MAX;
            let mut idx = 0;
            for i in 1..self.n + 1 {
                if !used[i] && self.min_dist[i] < min {
                    min = self.min_dist[i];
                    idx = i;
                }
            }

            // 标记访问
            used[idx] = true;

            // 更新min_dist
            for j in 0..self.graph[idx].len() {
                let (t, w) = self.graph[idx][j];
                if !used[t] && self.min_dist[idx] + w < self.min_dist[t] {
                    self.min_dist[t] = self.min_dist[idx] + w;
                    self.parent[t] = idx;
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
        let mut dij = Dijkstra::new(5);
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
