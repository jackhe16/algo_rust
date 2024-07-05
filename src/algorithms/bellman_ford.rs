//! 单源最短路径(有向有权图)
//!
//! 按边松弛 n - 1 次
//!
//! 当存在负环时, 最短路的解是无限小, 可通过第n次去判断是否有变化, 有变化即存在负环,
//! 但第n次的min_dist结果是错的, 第n次及之后的正确结果需要在遍历边前复制min_dist作为副本用来判断

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
        self.min_dist[s] = 0;

        for _ in 1..self.n + 1 {
            for i in 1..self.n + 1 {
                for j in 0..self.graph[i].len() {
                    let vs = i;
                    let (vt, w) = self.graph[i][j];
                    if self.min_dist[vs] != i32::MAX && self.min_dist[vs] + w < self.min_dist[vt] {
                        self.min_dist[vt] = self.min_dist[vs] + w;
                        self.parent[vt] = vs;

                        if i == self.n {
                            self.flag = true;
                        }
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
    fn test_bellman_ford_neg() {
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
}
