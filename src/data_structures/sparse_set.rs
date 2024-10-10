//use super::paged_vec::PagedVec;

use std::{collections::HashMap, ptr::NonNull};

use super::type_erased_vec::TypeErasedVec;

pub struct SpraseDenseValueIndex
{
    sparse_page: usize,
    sparse_index: usize,
}

pub struct SparseSet<const PAGE_SIZE: usize>
{
    dense_indecies: TypeErasedVec,
    dense: TypeErasedVec,
    sparse: HashMap<usize, [usize; PAGE_SIZE]>, // TODO: get rid of hashmap
}

impl<const PAGE_SIZE: usize> SparseSet<PAGE_SIZE>
{
    pub fn new<T>() -> Self
    {
        let mut dense = TypeErasedVec::new::<T>();
        let mut dense_indecies = TypeErasedVec::new::<SpraseDenseValueIndex>();

        dense.reserve(PAGE_SIZE);
        dense_indecies.reserve(PAGE_SIZE);

        dense.emplace();
        dense_indecies.emplace();

        Self
        {
            dense_indecies,
            dense,
            sparse: HashMap::new(),
        }
    }

    fn map_index(index: usize) -> (usize, usize)
    {
        let page  = index / PAGE_SIZE;
        let index = index % PAGE_SIZE;
        (page, index)
    }

    pub fn emplace(&mut self, index: usize) -> bool
    {
        let (page, index) = Self::map_index(index);
        let page_sparse = self.sparse.entry(page).or_insert_with(|| [0; PAGE_SIZE]);

        if page_sparse[index] != 0
        {
            return false;
        }
        
        self.dense.emplace();
        self.dense_indecies.emplace();

        let dense_index = self.dense.len() - 1;
        let dense_value_index = self.dense_indecies.get_typed_mut::<SpraseDenseValueIndex>(dense_index);

        dense_value_index.sparse_page = page;
        dense_value_index.sparse_index = index;

        page_sparse[index] = dense_index;

        true
    }

    pub fn set<T>(&mut self, index: usize, value: T)
    {
        if (self.emplace(index))
        {
            let dense_index = self.dense.len() - 1;
            *self.dense.get_typed_mut::<T>(dense_index) = value;
        }
    }

    pub fn get<T>(&self, index: usize) -> Option<&T>
    {
        let (page, index) = Self::map_index(index);
        self.sparse.get(&page).and_then(|page_sparse| 
        {
            let dense_index = page_sparse[index];
            if dense_index != 0
            {
                Some(self.dense.get_typed::<T>(dense_index))
            }
            else
            {
                None
            }
        })
    }

    pub fn remove(&mut self, index: usize)
    {
        let (page, index) = Self::map_index(index);

        if let Some(page_sparse) = self.sparse.get_mut(&page)
        {
            let dense_index = page_sparse[index];
            let last_dense_index = self.dense.len() - 1;
            if dense_index != 0
            {
                let last_dense_value_index = self.dense_indecies.get_typed::<SpraseDenseValueIndex>(last_dense_index);
                let last_page = last_dense_value_index.sparse_page;
                let last_index = last_dense_value_index.sparse_index;

                page_sparse[index] = 0;

                if dense_index != last_dense_index
                {
                    let last_page_sparse = self.sparse.get_mut(&last_page).unwrap();
                    last_page_sparse[last_index] = dense_index;
                    self.dense.remove_swap_with_last(dense_index);
                    self.dense_indecies.remove_swap_with_last(dense_index);
                }
                else
                {
                    self.dense.remove_swap_with_last(dense_index);
                    self.dense_indecies.remove_swap_with_last(dense_index);
                }
            }
        }
    }

    pub fn iter<T: 'static>(&self) -> impl Iterator<Item = (usize, &T)>
    {
        let dense_indices = self.dense_indecies.iter_typed::<SpraseDenseValueIndex>().skip(1);
        let dense_values = self.dense.iter_typed::<T>().skip(1);

        dense_indices.zip(dense_values).map(|(index, value)| {
            let idx = index.sparse_page * PAGE_SIZE + index.sparse_index;
            (idx, value)
        })
    }

    pub fn iter_mut<T: 'static>(&mut self) -> impl Iterator<Item = (usize, &mut T)>
    {
        let dense_indices = self.dense_indecies.iter_typed::<SpraseDenseValueIndex>().skip(1);
        let dense_values = self.dense.iter_typed_mut::<T>().skip(1);

        dense_indices.zip(dense_values).map(|(index, value)| {
            let idx = index.sparse_page * PAGE_SIZE + index.sparse_index;
            (idx, value)
        })
    }
}