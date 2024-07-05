pub struct Disjoint {
    father: Vec<usize>,
}

impl Disjoint {
    pub fn new(n: usize) -> Self {
        let father = (0..n).collect::<Vec<_>>();
        Disjoint { father }
    }

    pub fn find(&mut self, u: usize) -> usize {
        if u == self.father[u] {
            u
        } else {
            self.father[u] = self.find(self.father[u]);
            self.father[u]
        }
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        let fu = self.find(u);
        let fv = self.find(v);
        fu == fv
    }

    pub fn join(&mut self, u: usize, v: usize) {
        let fu = self.find(u);
        let fv = self.find(v);
        if fu != fv {
            self.father[fv] = fu;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint() {
        let mut disj = Disjoint::new(10);
        assert!(!disj.is_same(1, 5));
        disj.join(1, 3);
        disj.join(3, 5);
        assert!(disj.is_same(1, 5));
    }
}
