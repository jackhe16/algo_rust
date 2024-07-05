//! # 多源最短路径(有向有权图)(非负环)
//!
//! 判断负环

pub struct Floyd {
    graph: Vec<Vec<i32>>,
    n: usize,
    /// 顶点的索引从1开始, 值为0代表不经过中间点
    path: Vec<Vec<usize>>,
    is_negative_cycle: bool,
}

impl Floyd {
    pub fn new(n: usize) -> Self {
        let mut g = vec![vec![0; n + 1]; n + 1];
        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if i != j {
                    g[i][j] = i32::MAX;
                }
            }
        }

        Floyd {
            graph: g,
            n,
            path: vec![vec![0; n + 1]; n + 1],
            is_negative_cycle: false,
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize, w: i32) {
        self.graph[l][r] = w;
    }

    pub fn floyd(&mut self) {
        for k in 1..self.n + 1 {
            for i in 1..self.n + 1 {
                for j in 1..self.n + 1 {
                    if i != k
                        && j != k
                        && self.graph[i][k] != i32::MAX
                        && self.graph[k][j] != i32::MAX
                        && self.graph[i][j] > self.graph[i][k] + self.graph[k][j]
                    {
                        self.graph[i][j] = self.graph[i][k] + self.graph[k][j];
                        self.path[i][j] = k;
                    }
                }

                // 负环判断
                if self.graph[i][i] < 0 {
                    self.is_negative_cycle = true;
                }
            }
        }
    }

    /// 得到s->t的最短路径值
    pub fn get_val(&self, s: usize, t: usize) -> i32 {
        self.graph[s][t]
    }

    /// 输出s->t的最短路径过程
    pub fn print_path(&self, s: usize, t: usize) {
        if self.is_negative_cycle {
            println!("存在负环");
            return;
        }

        let k = self.path[s][t];
        if k == 0 {
            println!("{} -> {}", s, t);
            return;
        }
        self.print_path(s, k);
        self.print_path(k, t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 负环
    #[test]
    fn test_floyd_cycle() {
        let mut fl = Floyd::new(4);
        fl.add_edge(1, 2, 1);
        fl.add_edge(3, 2, 1);
        fl.add_edge(2, 4, 1);
        fl.add_edge(4, 3, -3);

        fl.floyd();
        let res = fl.get_val(1, 4);
        println!("{}", res);

        fl.print_path(1, 4);
    }
}
