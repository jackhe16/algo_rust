//! # 求s->t的最大流
//!
//! ## 算法实现
//! - (1) 得到Level Graph, 即从s点出发, 走一步能到的点为第一层, 走两步能到的点为第二层(不包括已走过的点),
//!       若在Level Graph上找不到s->t的路径则结束
//! - (2) 在Level Graph上使用Naive算法得到流量图
//! - (3) 在原图上减去流量图, 并增加反向流量
//! - (4) 重复(1) - (3)
//!
//! ## 复杂度 (m:边数,n:顶点数)
//! - 时间复杂度为 O(m * n^2)
//! - 边数可以远大于顶点数, 性能比Edmonds–Karp好

use std::collections::VecDeque;

pub struct FlowEdge {
    t: usize,
    cap: usize,
    flow: usize,
}

impl FlowEdge {
    pub fn new(t: usize, cap: usize) -> Self {
        FlowEdge { t, cap, flow: 0 }
    }
}

pub struct Dinic {
    // 互查双向边的无向图
    graph: Vec<Vec<usize>>,
    edges: Vec<FlowEdge>,
    ne: usize,
    s: usize,
    t: usize,
    level: Vec<usize>,
    last: Vec<usize>,
    solved: bool,
}

impl Dinic {
    pub fn new(nv: usize, s: usize, t: usize) -> Self {
        Dinic {
            graph: vec![vec![]; nv + 1],
            edges: vec![],
            ne: 0,
            s,
            t,
            level: vec![0; nv + 1],
            last: vec![0; nv + 1],
            solved: false,
        }
    }

    pub fn add_edge(&mut self, s: usize, t: usize, cap: usize) {
        self.edges.push(FlowEdge::new(t, cap));
        self.edges.push(FlowEdge::new(s, 0));
        self.graph[s].push(self.ne);
        self.graph[t].push(self.ne + 1);
        self.ne += 2;
    }

    // level图是否存在s->t路径
    fn bfs(&mut self) -> bool {
        let mut que = VecDeque::new();
        que.push_back(self.s);

        while !que.is_empty() {
            let cur = que.pop_front().unwrap();

            for &ei in self.graph[cur].iter() {
                let edge = &self.edges[ei];

                if self.level[edge.t] != 0 || edge.cap <= edge.flow {
                    continue;
                }

                self.level[edge.t] = self.level[cur] + 1;
                que.push_back(edge.t);
            }
        }

        self.level[self.t] != 0
    }

    // TODO: 多路增广
    fn dfs(&mut self, s: usize, pushed: usize) -> usize {
        if s == self.t {
            return pushed;
        }

        for j in self.last[s]..self.graph[s].len() {
            let ei = self.graph[s][j];
            let edge = &self.edges[ei];

            if self.level[s] + 1 != self.level[edge.t] || edge.cap <= edge.flow {
                continue;
            }

            let down_flow = self.dfs(edge.t, pushed.min(edge.cap - edge.flow));
            if down_flow == 0 {
                continue;
            }
            self.last[s] = j;
            self.edges[ei].flow += down_flow;
            // self.edges[ei ^ 1].flow -= down_flow;
            // flow类型为usize, 减法会导致溢出; 根据residual = cap - flow;
            self.edges[ei ^ 1].cap += down_flow;
            return down_flow;
        }

        self.last[s] = self.graph[s].len();
        0
    }

    pub fn dinic(&mut self) -> usize {
        self.solved = true;
        let mut total = 0;

        loop {
            self.level.fill(0);
            self.level[self.s] = 1;

            if !self.bfs() {
                break;
            }

            self.last.fill(0);
            loop {
                let flow = self.dfs(self.s, usize::MAX);
                if flow == 0 {
                    break;
                }
                total += flow;
            }
        }

        total
    }

    pub fn get_flow_edges(&mut self) -> Vec<(usize, usize, usize, usize)> {
        if !self.solved {
            self.dinic();
        }

        let mut result = vec![];
        for l in 1..self.graph.len() {
            for &ei in self.graph[l].iter() {
                if ei % 2 == 0 {
                    let edge = &self.edges[ei];
                    result.push((l, edge.t, edge.flow, edge.cap));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dinic() {
        let mut din = Dinic::new(4, 1, 4);
        din.add_edge(1, 2, 1);
        din.add_edge(1, 3, 1);
        din.add_edge(2, 3, 1);
        din.add_edge(2, 4, 1);
        din.add_edge(3, 4, 1);

        let res = din.dinic();
        println!("{}", res);
        assert_eq!(res, 2);

        let flow_edges = din.get_flow_edges();
        println!("{:?}", flow_edges);
    }
}
