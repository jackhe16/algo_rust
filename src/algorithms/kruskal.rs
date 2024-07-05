//! # 最小生成树
//!
//! 边排序, 遍历边, 判断边的两个顶点是否已在同棵树中(并查集), 是则跳过, 不是则加入

use super::disjoint::Disjoint;

#[derive(Debug, Clone)]
struct Edge {
    l: usize,
    r: usize,
    v: i32,
}

pub struct Kruskal {
    edges: Vec<Edge>,
    n: usize,
    result: Vec<Edge>,
}

impl Kruskal {
    pub fn new(n: usize) -> Self {
        Kruskal {
            edges: vec![],
            n,
            result: vec![],
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize, v: i32) {
        self.edges.push(Edge { l, r, v });
    }

    pub fn kruskal(&mut self) -> i32 {
        self.edges.sort_by(|a, b| a.v.cmp(&b.v));
        let mut disj = Disjoint::new(self.n + 1);
        let mut res = 0;

        for e in &self.edges {
            if !disj.is_same(e.l, e.r) {
                self.result.push(e.clone());
                res += e.v;
                disj.join(e.l, e.r);
            }
        }

        res
    }

    pub fn print_result(&self) {
        for e in &self.result {
            println!("{} - {}: {}", e.l, e.r, e.v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal() {
        let mut krus = Kruskal::new(5);
        krus.add_edge(1, 2, 4);
        krus.add_edge(1, 3, 1);
        krus.add_edge(1, 4, 3);
        krus.add_edge(2, 3, 2);
        krus.add_edge(2, 5, 2);
        krus.add_edge(3, 5, 2);

        let res = krus.kruskal();
        println!("{}", res);

        krus.print_result();
    }
}
