//! # 最小生成树
//!
//! 按点生成, 选择一个顶点作为起点, min_dist[起点]=i32::MIN, 寻找min_dist中最小值, 标记该点已访问, 按该点相连的边更新min_dist
//!
//! min_dist: 该点到最小生成树的最小权值

pub struct Prim {
    /** 无向有权图 */
    graph: Vec<Vec<(usize, i32)>>,
    /** 顶点个数 */
    n: usize,
    parent: Vec<usize>,
    min_dist: Vec<i32>,
}

impl Prim {
    pub fn new(n: usize) -> Self {
        Prim {
            graph: vec![vec![]; n + 1],
            n,
            parent: vec![0; n + 1],
            min_dist: vec![i32::MAX; n + 1],
        }
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, w: i32) {
        self.graph[v1].push((v2, w));
        self.graph[v2].push((v1, w));
    }

    pub fn prim(&mut self) -> i32 {
        let mut used = vec![false; self.n + 1];
        self.min_dist[1] = i32::MIN;

        for _ in 1..self.n {
            // 寻找min_dist中最小值
            let mut min = i32::MAX;
            let mut idx = 0 as usize;
            for i in 1..self.n + 1 {
                if !used[i] && self.min_dist[i] < min {
                    min = self.min_dist[i];
                    idx = i;
                }
            }

            // 标记该点已访问
            used[idx] = true;

            // 更新min_dist
            for j in 0..self.graph[idx].len() {
                let (v2, w) = self.graph[idx][j];
                if !used[v2] && w < self.min_dist[v2] {
                    self.min_dist[v2] = w;
                    self.parent[v2] = idx;
                }
            }
        }

        let mut res = 0;
        for i in 2..self.n + 1 {
            res += self.min_dist[i];
        }

        res
    }

    pub fn print_min_dist(&self) {
        for i in 2..self.n + 1 {
            let w = self.min_dist[i];
            println!("(parent -> child: w) {} -> {}: {}", self.parent[i], i, w);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prim() {
        let mut p = Prim::new(5);
        p.add_edge(1, 2, 4);
        p.add_edge(1, 3, 1);
        p.add_edge(1, 4, 3);
        p.add_edge(2, 3, 2);
        p.add_edge(2, 5, 2);
        p.add_edge(3, 5, 2);

        let res = p.prim();
        println!("{}", res);

        p.print_min_dist();
    }
}
