use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

pub fn flood_fill<N, I, FV>(initial: N, mut visit: FV)
where
    N: Eq + Hash,
    I: IntoIterator<Item = N>,
    FV: FnMut(&N) -> I,
{
    let mut visited = HashMap::new();
    dfs(initial, |n| {
        let iter = match visited.entry(n) {
            Entry::Occupied(_) => None,
            Entry::Vacant(slot) => {
                let slot = slot.insert_entry(());
                Some(visit(slot.key()).into_iter())
            }
        };
        iter.into_iter().flatten()
    });
}

pub fn dfs<N, I, FV>(initial: N, mut visit: FV)
where
    N: Eq + Hash,
    I: IntoIterator<Item = N>,
    FV: FnMut(N) -> I,
{
    pub fn dfs_impl<N, I, FV>(current: N, visit: &mut FV)
    where
        N: Eq + Hash,
        I: IntoIterator<Item = N>,
        FV: FnMut(N) -> I,
    {
        for node in visit(current) {
            dfs_impl(node, visit);
        }
    }

    dfs_impl(initial, &mut visit)
}
