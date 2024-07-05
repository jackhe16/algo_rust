//! # 二分图最大匹配(库恩(kuhn)算法, 霍普克洛夫特-卡普(hopcroft-karp)算法)
//!
//!

use std::collections::VecDeque;

pub struct BipartiteMatching {
    /** 二分图(无权) */
    graph: Vec<Vec<usize>>,
    /** u集合顶点数 */
    u_len: usize,
    /** v集合顶点数 */
    v_len: usize,
    /** 记录u的匹配对象, u_match[u] = v, 默认值0代表未匹配 */
    u_match: Vec<usize>,
    /** 记录v的匹配对象, v_match[v] = u, 默认值0代表未匹配 */
    v_match: Vec<usize>,
    /** 递归时记录v已匹配 */
    v_used: Vec<bool>,
}

impl BipartiteMatching {
    pub fn new(u_len: usize, v_len: usize) -> Self {
        BipartiteMatching {
            graph: vec![vec![]; u_len + 1],
            u_len,
            v_len,
            u_match: vec![0; u_len + 1],
            v_match: vec![0; v_len + 1],
            v_used: vec![false; v_len + 1],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.graph[u].push(v);
    }

    pub fn print_matching(&self) {
        for i in 1..self.v_len + 1 {
            if self.v_match[i] != 0 {
                println!("Vertex matched (u, v): ({}, {})", self.v_match[i], i);
            }
        }
    }

    fn try_kuhn(&mut self, u: usize) -> bool {
        for i in 0..self.graph[u].len() {
            let v = self.graph[u][i];

            if self.v_used[v] {
                continue;
            }
            self.v_used[v] = true;

            if self.v_match[v] == 0 || self.try_kuhn(self.v_match[v]) {
                self.v_match[v] = u;
                return true;
            }
        }
        false
    }

    /// 库恩(kuhn)算法
    ///
    /// 也是增广路径的应用
    ///
    /// - 二分图, S={u1, u2}, T={v1, v2}, 路径(u1, v1), (u1, v2), (u2, v1)
    /// - 当(u1, v1)已匹配, u2匹配v1时, v1已被u1匹配, 此时递归u1如果有其他可行匹配(u1, v2),
    /// - 则存在一条增广路径(u2, v1), (v1, u1), (u1, v2), 打破匹配(u1, v1), 可构成匹配(u2, v1), (u1, v2)
    pub fn kuhn(&mut self) {
        self.v_match = vec![0; self.v_len + 1];

        for u in 1..self.u_len + 1 {
            self.v_used = vec![false; self.v_len + 1];
            self.try_kuhn(u);
        }
    }

    fn bfs(&self, dist: &mut Vec<i32>) -> bool {
        let mut q = VecDeque::new();

        for u in 1..self.u_len + 1 {
            if self.u_match[u] == 0 {
                dist[u] = 0;
                q.push_back(u);
            } else {
                dist[u] = i32::MAX;
            }
        }
        dist[0] = i32::MAX;

        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            if dist[u] < dist[0] {
                for i in 0..self.graph[u].len() {
                    let v = self.graph[u][i];
                    let u_prev = self.v_match[v];

                    if dist[u_prev] == i32::MAX {
                        dist[u_prev] = dist[u] + 1;
                        q.push_back(u_prev);
                    }
                }
            }
        }

        dist[0] != i32::MAX
    }

    fn dfs(&mut self, u: usize, dist: &mut Vec<i32>) -> bool {
        if u == 0 {
            return true;
        }

        for i in 0..self.graph[u].len() {
            let v = self.graph[u][i];

            if dist[self.v_match[v]] == dist[u] + 1 && self.dfs(self.v_match[v], dist) {
                self.v_match[v] = u;
                self.u_match[u] = v;
                return true;
            }
        }

        dist[u] = i32::MAX;
        false
    }

    /// 霍普克洛夫特-卡普(hopcroft-karp)算法
    ///
    /// 最大匹配 <=> 无法再找到一条增广路径
    ///
    /// - 匹配路径: 处于一个匹配中的路径
    /// - 非匹配路径: 不处于一个匹配中的路径
    /// - 自由顶点: 不处于一个匹配中的顶点
    /// - 交替路径: 匹配路径与非匹配路径交替
    /// - 增广路径: 从自由顶点u出发, 经过交替路径, 到达自由顶点v
    pub fn hopcroft_karp(&mut self) -> i32 {
        self.u_match = vec![0; self.u_len + 1];
        self.v_match = vec![0; self.v_len + 1];
        // 记录增广路径, MAX表示已匹配, 非MAX表示增广路径节点
        let mut dist = vec![i32::MAX; self.u_len + 1];
        let mut res = 0;

        while self.bfs(&mut dist) {
            for u in 1..self.u_len + 1 {
                if self.u_match[u] == 0 && self.dfs(u, &mut dist) {
                    res += 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::BipartiteMatching;

    #[test]
    fn test_kuhn() {
        let mut bm = BipartiteMatching::new(7, 7);
        bm.add_edge(1, 2);
        bm.add_edge(1, 4);
        bm.add_edge(1, 7);
        bm.add_edge(2, 1);
        bm.add_edge(2, 6);
        bm.add_edge(3, 2);
        bm.add_edge(3, 3);
        bm.add_edge(4, 2);
        bm.add_edge(4, 5);
        bm.add_edge(4, 7);
        bm.add_edge(5, 3);
        bm.add_edge(6, 3);
        bm.add_edge(7, 3);
        bm.add_edge(7, 5);

        bm.kuhn();

        bm.print_matching();

        assert_eq!(bm.v_match, vec![0, 2, 3, 5, 1, 7, 0, 4]);
    }

    #[test]
    fn test_hopcroft_karp() {
        let mut bm = BipartiteMatching::new(4, 4);
        bm.add_edge(1, 2);
        bm.add_edge(1, 3);
        bm.add_edge(2, 1);
        bm.add_edge(3, 2);
        bm.add_edge(4, 2);
        bm.add_edge(4, 4);

        bm.hopcroft_karp();

        bm.print_matching();

        assert_eq!(bm.v_match, vec![0, 2, 3, 1, 4]);
        assert_eq!(bm.u_match, vec![0, 3, 1, 2, 4]);
    }
}
