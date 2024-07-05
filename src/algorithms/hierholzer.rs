//! # 海尔霍尔泽算法
//!
//! ## 这里只考虑无向欧拉图
//!
//! ## 应用场景
//! - 得到欧拉回路
//!
//! ## 算法描述
//! - 欧拉路径(eulerian path): 从某一点出发经过一条不间断的路径,这条路径刚好访问图的所有边一次且仅一次。(图可以是无向图或有向图)
//! - 欧拉回路(eulerian circuit): 首尾相连的欧拉路径
//! - 欧拉图(eulerian graph): 具有欧拉回路的图
//!   - 无向图: 图的所有顶点的度数为偶数
//!   - 有向图: 图的所有顶点的入度和出度相等
//! - 半欧拉图(semi eulerian graph): 具有欧拉路径但不具有欧拉回路的图
//!   - 无向图: 有且仅有两个顶点的度数为奇数
//!   - 有向图: 有且仅有一个顶点入度-出度=1,另有且仅有一个顶点出度-入度=1
//!
//! ## 算法思路
//! - 判断给定的图是不是欧拉图或半欧拉图
//! - 确定路径起点: 欧拉图任意, 半欧拉图奇数点其一(无向图)或出度-入度=1(有向图)
//! - 初始化答案栈anser,存储路径途径节点
//! - 对图进行dfs遍历(允许点重复经过), 每访问一条边就标记(将自身设置为0或另外记录), 然后对这条边的另一端节点进行遍历, 当当前点相邻边均已标记时, 将此点加入anser栈, 递归返回
//! - 遍历结束, anser栈元素依次弹出, 得到的就是路径访问顺序
//!

pub struct Hierholzer {
    graph: Vec<Vec<usize>>,
    n: usize,
}

impl Hierholzer {
    pub fn new(n: usize) -> Self {
        Hierholzer {
            graph: vec![vec![]; n + 1],
            n,
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize) {
        self.graph[l].push(r);
        self.graph[r].push(l);
    }

    pub fn is_eulerian_graph(&self) -> bool {
        let mut degree = vec![0; self.n + 1];

        for l in 1..self.graph.len() {
            for &r in self.graph[l].iter() {
                degree[l] += 1;
                degree[r] += 1;
            }
        }

        degree.iter().skip(1).all(|v| v % 2 == 0)
    }

    fn dfs(&mut self, l: usize, ans: &mut Vec<usize>) {
        for j in 0..self.graph[l].len() {
            let r = self.graph[l][j];
            if r != 0 {
                self.graph[l][j] = 0;
                // 因为是无向图,反向边也要处理
                for i in 0..self.graph[r].len() {
                    if self.graph[r][i] == l {
                        self.graph[r][i] = 0;
                        break;
                    }
                }
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
