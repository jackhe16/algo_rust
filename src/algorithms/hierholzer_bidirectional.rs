//! # 海尔霍尔泽算法
//!
//! ## 这里只考虑无向欧拉图
//! ## 这里无向图的存储方式支持双向查找边

pub struct Hierholzer {
    graph: Vec<Vec<usize>>,
    edges: Vec<usize>,
    n: usize,
}

impl Hierholzer {
    pub fn new(n: usize) -> Self {
        Hierholzer {
            graph: vec![vec![]; n + 1],
            edges: vec![],
            n,
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize) {
        let e_len = self.edges.len();
        self.edges.push(r);
        self.edges.push(l);
        self.graph[l].push(e_len);
        self.graph[r].push(e_len + 1);
    }

    pub fn is_eulerian_graph(&self) -> bool {
        let mut degree = vec![0; self.n + 1];

        for l in 1..self.graph.len() {
            for &ei in self.graph[l].iter() {
                let r = self.edges[ei];
                degree[l] += 1;
                degree[r] += 1;
            }
        }

        degree.iter().skip(1).all(|v| v % 2 == 0)
    }

    fn dfs(&mut self, l: usize, ans: &mut Vec<usize>) {
        for j in 0..self.graph[l].len() {
            let ei = self.graph[l][j];
            let r = self.edges[ei];
            if r != 0 {
                self.edges[ei] = 0;
                self.edges[ei ^ 1] = 0;
                self.dfs(r, ans);
            }
        }
        ans.push(l);
    }

    pub fn hierholzer(&mut self, start: usize) -> Vec<usize> {
        let mut ans = vec![];

        // 不是欧拉图,不存在欧拉回路
        if !self.is_eulerian_graph() {
            return ans;
        }

        self.dfs(start, &mut ans);
        ans.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierholzer() {
        let mut hier = Hierholzer::new(5);
        hier.add_edge(1, 2);
        hier.add_edge(1, 3);
        hier.add_edge(2, 3);
        hier.add_edge(3, 4);
        hier.add_edge(3, 5);
        hier.add_edge(4, 5);

        let res = hier.hierholzer(4);
        println!("{:?}", res);
    }
}
