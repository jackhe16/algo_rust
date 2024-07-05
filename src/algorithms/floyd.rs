//! # 多源最短路径(有向有权图)(非负环)
//!
//! ## 算法实现
//! - 对于任意一中心点k, i != j && i != k && j != k, 若G[i][j] > G[i][k] + G[k][j], 则i到j的最短路径应该经过k
//! - G[i][k] + G[k][j]会溢出(i32::MAX), 需特殊处理
//! - 可以遍历中心点k, 循环i,j, 最终结果就是多源最短路径
//! - 可以使用Path矩阵存储中间节点
//! - 可负权

pub struct Floyd {
    graph: Vec<Vec<i32>>,
    n: usize,
    /// 顶点的索引从1开始, 值为0代表不经过中间点
    path: Vec<Vec<usize>>,
}

impl Floyd {
    pub fn new(n: usize) -> Self {
        Floyd {
            graph: vec![vec![i32::MAX; n + 1]; n + 1],
            n,
            path: vec![vec![0; n + 1]; n + 1],
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize, w: i32) {
        self.graph[l][r] = w;
    }

    pub fn floyd(&mut self) {
        for k in 1..self.n + 1 {
            for i in 1..self.n + 1 {
                for j in 1..self.n + 1 {
                    if i != j
                        && i != k
                        && j != k
                        && self.graph[i][k] != i32::MAX
                        && self.graph[k][j] != i32::MAX
                        && self.graph[i][j] > self.graph[i][k] + self.graph[k][j]
                    {
                        self.graph[i][j] = self.graph[i][k] + self.graph[k][j];
                        self.path[i][j] = k;
                    }
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

    #[test]
    fn test_floyd() {
        let mut fl = Floyd::new(5);
        fl.add_edge(1, 2, 4);
        fl.add_edge(1, 3, 1);
        fl.add_edge(1, 4, 3);
        fl.add_edge(2, 3, 2);
        fl.add_edge(2, 5, 2);
        fl.add_edge(3, 5, 2);

        fl.floyd();
        let res = fl.get_val(1, 5);
        println!("{}", res);

        fl.print_path(1, 5);
    }

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
