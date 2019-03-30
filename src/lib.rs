/// Union-find data structure with "union by rank" and "path compression" optimizations.
pub struct UnionFind {
    components: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            components: n,
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i == j {
            return;
        }
        // Union by rank
        if self.size[i] < self.size[j] {
            self.parent[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.parent[j] = i;
            self.size[i] += self.size[j];
        }
        self.components -= 1;
    }

    pub fn find(&mut self, mut p: usize) -> usize {
        let mut r = p;
        while r != self.parent[r] {
            r = self.parent[r];
        }
        // Path compression
        while p != self.parent[p] {
            let pp = self.parent[p];
            self.parent[p] = r;
            p = pp;
        }
        r
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
    pub fn components(&self) -> usize {
        self.components
    }
    pub fn size(&self) -> usize {
        self.parent.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_disjoint(uf: &mut UnionFind, c1: &[usize], c2: &[usize]) {
        for x in c1 {
            for y in c2 {
                assert!(!uf.connected(*x, *y));
            }
        }
    }

    fn check_connected(uf: &mut UnionFind, comp: &[usize]) {
        for i in 0..comp.len() - 1 {
            for j in i + 1..comp.len() {
                assert!(uf.connected(comp[i], comp[j]));
            }
        }
    }

    fn check_components(uf: &mut UnionFind, components: &[&[usize]]) {
        for comp in components {
            check_connected(uf, comp);
        }
        for i in 0..components.len() - 1 {
            for j in i + 1..components.len() {
                check_disjoint(uf, components[i], components[j]);
            }
        }
    }

    #[test]
    fn simple_1() {
        let mut uf = UnionFind::new(7);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(1, 3);
        uf.union(1, 4);
        uf.union(5, 6);
        assert_eq!(uf.components(), 2);
        assert_eq!(uf.size(), 7);
        let c1 = [0, 1, 2, 3, 4];
        let c2 = [5, 6];
        check_components(&mut uf, &[&c1, &c2]);
    }

    #[test]
    fn simple_2() {
        let mut uf = UnionFind::new(10);
        for i in 0..9 {
            uf.union(i, i + 1);
        }
        assert_eq!(uf.components(), 1);
        assert_eq!(uf.size(), 10);
        let c1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        check_components(&mut uf, &[&c1]);
    }

    #[test]
    fn simple_3() {
        let mut uf = UnionFind::new(10);
        for i in 0..10 {
            if i % 2 == 0 {
                uf.union(i, 2);
            } else if i % 3 == 0 {
                uf.union(i, 3);
            }
        }
        assert_eq!(uf.components(), 5);
        assert_eq!(uf.size(), 10);
        let c1 = [0, 2, 4, 6, 8];
        let c2 = [3, 9];
        let c3 = [1];
        let c4 = [5];
        let c5 = [7];
        check_components(&mut uf, &[&c1, &c2, &c3, &c4, &c5]);
    }

    #[test]
    fn simple_4() {
        let mut uf = UnionFind::new(20);
        for i in 2..uf.size() {
            let mut is_prime = true;
            for j in 2..i {
                if i % j == 0 {
                    is_prime = false;
                    break;
                }
            }
            let c = match is_prime {
                true => 0,
                false => 1,
            };
            uf.union(i, c);
        }
        assert_eq!(uf.components(), 2);
        let c1 = [2, 3, 5, 7, 11, 13, 17, 19];
        let c2 = [1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18];
        check_components(&mut uf, &[&c1, &c2]);
    }
}
