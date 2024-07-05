//! # 求s->t的最大流(有向非负权图)
//!
//! - (1) 寻找一条s->t的路径,该路径上最小流量为flow,若找不到则结束
//! - (2) res += flow
//! - (3) 在图上沿着该条路径每条边减去flow, 并把反向边加上flow
//! - (4) 重复(1) - (3)
//!
//! ## 优化 (m边数, n顶点数, f最大流量权值)
//! - 该算法与流量权值相关, O(f * m), f可能很大
//! - 寻找一条s->t的路径使用最短路径时, 可以使时间复杂度与最大流量权值无关, O(m^2 * n), 这个算法叫Edmonds–Karp
//!
//! ## Naive算法(最大流基础算法,得到的是阻塞流,不是最大流,最大流一定是阻塞流)
//! - ford_fulkerson算法在Naive算法的基础上增加了反向流量

use std::collections::VecDeque;

pub struct FordFulkerson {
    graph: Vec<Vec<usize>>,
    n: usize,
}

impl FordFulkerson {
    pub fn new(n: usize) -> Self {
        FordFulkerson {
            graph: vec![vec![0; n + 1]; n + 1],
            n,
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize, w: usize) {
        self.graph[l][r] = w;
    }

    // 寻找s->t的一条路径
    fn bfs(&self, s: usize, t: usize, parent: &mut Vec<usize>) -> usize {
        // 此处parent同时担任了visited的作用,需要重置
        parent.fill(0);

        let mut que = VecDeque::new();
        que.push_back((s, usize::MAX));

        while !que.is_empty() {
            let (cur, flow) = que.pop_front().unwrap();

            for j in 1..self.n + 1 {
                // 此处parent同时担任了visited的作用,s点只能特殊判断
                if j != s && parent[j] == 0 && self.graph[cur][j] != 0 {
                    parent[j] = cur;
                    let next_flow = flow.min(self.graph[cur][j]);
                    if j == t {
                        return next_flow;
                    }
                    que.push_back((j, next_flow));
                }
            }
        }

        0
    }

    pub fn ford_fulkerson(&mut self, s: usize, t: usize) -> usize {
        let mut res = 0;
        let mut parent = vec![0; self.n + 1];

        loop {
            let flow = self.bfs(s, t, &mut parent);
            if flow == 0 {
                break;
            }
            res += flow;

            let mut cur = t;
            while cur != s {
                let prev = parent[cur];
                self.graph[prev][cur] -= flow;
                self.graph[cur][prev] += flow;
                cur = prev;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ford_fulkerson() {
        let mut ff = FordFulkerson::new(4);
        ff.add_edge(1, 2, 1);
        ff.add_edge(1, 3, 1);
        ff.add_edge(2, 3, 1);
        ff.add_edge(2, 4, 1);
        ff.add_edge(3, 4, 1);

        let res = ff.ford_fulkerson(1, 4);
        println!("{}", res);
        assert_eq!(res, 2);
    }
}
