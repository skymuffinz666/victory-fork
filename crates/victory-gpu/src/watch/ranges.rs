//! Exact clean-block intervals. No write stamps or guard semantics change.
use std::collections::BTreeMap;

pub(super) struct Runs(BTreeMap<usize, usize>);

impl Runs {
    pub(super) const fn new() -> Self { Self(BTreeMap::new()) }

    pub(super) fn covers(&self, first: usize, last: usize) -> bool {
        first >= last || self.0.range(..=first).next_back().is_some_and(|(_, &end)| end >= last)
    }

    pub(super) fn insert(&mut self, mut first: usize, mut last: usize) {
        if first >= last { return; }
        if let Some((&a, &b)) = self.0.range(..=first).next_back() {
            if b >= first {
                first = a;
                last = last.max(b);
                self.0.remove(&a);
            }
        }
        while let Some((&a, &b)) = self.0.range(first..=last).next() {
            last = last.max(b);
            self.0.remove(&a);
        }
        self.0.insert(first, last);
    }

    pub(super) fn remove(&mut self, block: usize) {
        let Some((&a, &b)) = self.0.range(..=block).next_back() else { return; };
        if block >= b { return; }
        self.0.remove(&a);
        if a < block { self.0.insert(a, block); }
        if block + 1 < b { self.0.insert(block + 1, b); }
    }

    pub(super) fn present(&self, first: usize, last: usize) -> Vec<usize> {
        if first >= last { return Vec::new(); }
        let start = self.0.range(..=first).next_back().map_or(first, |(&a, _)| a);
        self.0.range(start..last)
            .flat_map(|(&a, &b)| a.max(first)..b.min(last))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn intervals_match_individual_blocks_under_writes_and_remaps() {
        let mut runs = Runs::new();
        let mut blocks = BTreeSet::new();
        let mut seed = 41u64;
        for step in 0..4000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = (seed >> 32) as usize % 512;
            let b = (a + (seed as usize % 61)).min(512);
            if step % 3 == 0 {
                runs.insert(a, b);
                blocks.extend(a..b);
            } else {
                runs.remove(a);
                blocks.remove(&a);
            }
            assert_eq!(runs.present(0, 512), blocks.iter().copied().collect::<Vec<_>>());
            assert_eq!(runs.covers(a, b), (a..b).all(|p| blocks.contains(&p)));
            assert_eq!(runs.present(a, b), blocks.range(a..b).copied().collect::<Vec<_>>());
        }
        runs.insert(0, 512);
        assert!(runs.covers(0, 512));
        runs.remove(0);
        runs.remove(511);
        assert!(!runs.covers(0, 512));
        assert!(runs.covers(1, 511));
    }
}
