use std::{collections::HashMap};

const INVALID_INDEX: usize = usize::MAX;

pub struct SparseDenseValueIndex {
    sparse_page: usize,
    sparse_index: usize,
}

impl SparseDenseValueIndex {
    pub fn new(sparse_page: usize, sparse_index: usize) -> Self {
        Self {
            sparse_page,
            sparse_index,
        }
    }
}

pub struct SparseSet<K, T, const PAGE_SIZE: usize> {
    dense_indices: Vec<SparseDenseValueIndex>,
    dense: Vec<T>,
    sparse_pages: HashMap<K, usize>,
    sparse: Vec<[usize; PAGE_SIZE]>,
}

impl<K, T, const PAGE_SIZE: usize> SparseSet<K, T, PAGE_SIZE>
where
    K: Copy + PartialEq + Eq + std::hash::Hash + From<usize> + Into<usize> + Clone,
{
    pub fn new() -> Self {
        let mut dense = Vec::new();
        let mut dense_indices = Vec::new();
        let mut sparse: Vec<[usize; PAGE_SIZE]> = Vec::new();
    
        dense.reserve(PAGE_SIZE);
        dense_indices.reserve(PAGE_SIZE);
        sparse.reserve(PAGE_SIZE);

        Self {
            dense_indices,
            dense,
            sparse_pages: HashMap::new(),
            sparse
        }
    }

    fn map_index(index: K) -> (usize, usize) {
        let page = index.into() / PAGE_SIZE;
        let index = index.into() % PAGE_SIZE;
        (page, index)
    }

    pub fn set(&mut self, index: K, value: T) -> bool {
        let (page, index) = Self::map_index(index);

        let sparse_page_index = self.sparse_pages.entry(K::from(page)).or_insert_with(|| {
            let page = [INVALID_INDEX; PAGE_SIZE];
            self.sparse.push(page);
            self.sparse.len() - 1
        });

        let page_sparse = &mut self.sparse[*sparse_page_index];

        if page_sparse[index] != INVALID_INDEX {
            return false;
        }

        self.dense.push(value);
        self.dense_indices.push(SparseDenseValueIndex::new(page, index));

        let dense_index = self.dense.len() - 1;

        page_sparse[index] = dense_index;

        true
    }

    pub fn get(&self, index: K) -> Option<&T> {
        let (page, index) = Self::map_index(index);
        
        let sparse_page_index = self.sparse_pages.get(&K::from(page))?;
        let page_sparse = &self.sparse[*sparse_page_index];
        let dense_index = page_sparse[index];

        if dense_index != INVALID_INDEX {
            self.dense.get(dense_index as usize)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: K) -> Option<&mut T> {
        let (page, index) = Self::map_index(index);
        self.sparse_pages.get(&K::from(page)).and_then(|&sparse_page_index| {
            let page_sparse = &self.sparse[sparse_page_index];
            let dense_index = page_sparse[index];
            if dense_index != INVALID_INDEX {
                self.dense.get_mut(dense_index as usize)
            } else {
                None
            }
        })
    }

    pub fn has(&self, index: K) -> bool {
        let (page, index) = Self::map_index(index);
        self.sparse_pages.get(&K::from(page)).map_or(false, |&sparse_page_index| {
            self.sparse[sparse_page_index][index] != INVALID_INDEX
        })
    }

    pub fn get_or_insert_with(&mut self, index: K, default: impl FnOnce() -> T) -> &mut T {
        if self.has(index) {
            self.get_mut(index).unwrap()
        } else {
            self.set(index, default());
            self.get_mut(index).unwrap()
        }
    }

    pub fn get_or_insert(&mut self, index: K, default: T) -> &mut T {
        self.get_or_insert_with(index, || default)
    }

    pub fn remove(&mut self, index: K) {
        let (page, index) = Self::map_index(index);

        if let Some(&sparse_page_index) = self.sparse_pages.get(&K::from(page)) {
            let page_sparse = &mut self.sparse[sparse_page_index];
            let dense_index = page_sparse[index];
            let last_dense_index = self.dense.len() - 1;
            if dense_index != INVALID_INDEX {
                let last_dense_value_index = self.dense_indices.get(last_dense_index).unwrap();
                let last_page = last_dense_value_index.sparse_page;
                let last_index = last_dense_value_index.sparse_index;

                page_sparse[index] = INVALID_INDEX;

                if dense_index as usize != last_dense_index {
                    let last_page_sparse = &mut self.sparse[self.sparse_pages[&K::from(last_page)]];
                    last_page_sparse[last_index] = dense_index;
                    self.dense.swap_remove(dense_index);
                    self.dense_indices.swap_remove(dense_index);
                } else {
                    self.dense.swap_remove(dense_index);
                    self.dense_indices.swap_remove(dense_index);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.dense.clear();
        self.dense_indices.clear();
        self.sparse.clear();
        self.sparse_pages.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, &T)> {
        self.dense_indices.iter().zip(self.dense.iter()).map(|(index, value)| {
            let idx = index.sparse_page * PAGE_SIZE + index.sparse_index;
            (K::from(idx), value)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut T)> {
        self.dense_indices.iter().zip(self.dense.iter_mut()).map(|(index, value)| {
            let idx = index.sparse_page * PAGE_SIZE + index.sparse_index;
            (K::from(idx), value)
        })
    }

    pub fn len(&self) -> usize {
        self.dense.len() - 1 // Subtract 1 to account for the null type
    }
}
