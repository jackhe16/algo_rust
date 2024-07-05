//! # Kahn算法
//!
//! ## 应用场景
//! - 拓扑排序
//!
//! ## 问题描述
//! - 有向无环图 (Directed Acyclic Graph: DAG)
//!   - 点: 任务
//!   - 边: 任务依赖关系 (A->B表示A需要在B之前完成)
//!
//! ## 算法描述
//! - 是一种广优算法
//!
//! ## 算法思路
//! - 入度为0的顶点表示没有前置依赖,移除该顶点所有出边,对出边顶点入度减1
//! - 入度表存储所有顶点的入度
//! - 队列存储为入度为0的顶点
//! - 成环判断, 入度表存在不为0的顶点
//!
//! ## 复杂度分析
//! - O(N+M)

use std::collections::VecDeque;

pub struct Kahn {
    graph: Vec<Vec<usize>>,
    n: usize,
}

impl Kahn {
    pub fn new(n: usize) -> Self {
        Kahn {
            graph: vec![vec![]; n + 1],
            n,
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize) {
        self.graph[l].push(r);
    }

    pub fn kahn(&self) -> Vec<usize> {
        let mut in_degree = vec![0; self.n + 1];
        let mut que = VecDeque::new();

        // 入度表
        for l in 1..self.n + 1 {
            for &r in self.graph[l].iter() {
                in_degree[r] += 1;
            }
        }

        // 0入度入队
        for i in 1..in_degree.len() {
            if in_degree[i] == 0 {
                que.push_back(i);
            }
        }

        let mut res = vec![];
        while !que.is_empty() {
            let cur = que.pop_front().unwrap();
            res.push(cur);

            for &r in self.graph[cur].iter() {
                in_degree[r] -= 1;
                if in_degree[r] == 0 {
                    que.push_back(r);
                }
            }
        }

        if in_degree.iter().skip(1).any(|v| *v != 0) {
            println!("成环");
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Kahn;

    #[test]
    pub fn test_kahn() {
        let mut k = Kahn::new(4);
        k.add_edge(1, 2);
        k.add_edge(2, 4);
        k.add_edge(4, 3);
        k.add_edge(3, 2);

        let res = k.kahn();
        println!("{:?}", res);
    }

    #[test]
    pub fn test_kahn_ring() {
        let mut k = Kahn::new(4);
        k.add_edge(1, 2);
        k.add_edge(1, 3);
        k.add_edge(2, 4);
        k.add_edge(3, 4);
        k.add_edge(2, 3);

        let res = k.kahn();
        println!("{:?}", res);
    }
}
