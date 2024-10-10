/*
 * [data, data, null, data, ..., data, null, null, ..., data]
 * [0, 2, 3, 4, 5] 
 */
/*
pub struct PagedVecU64<const PAGE_SIZE: usize>
{
    pages: Vec<[u64; PAGE_SIZE]>,
    indecies: Vec<u64>,
    len: usize,
}

impl<const PAGE_SIZE: usize> PagedVecU64<PAGE_SIZE>
{
    pub fn new() -> Self 
    {
        Self 
        {
            pages   : Vec::new(),
            indecies: Vec::new(),
            len: 0,
        }
    }
    
    fn map_index(index: usize) -> (usize, usize)
    {
        let page  = index / PAGE_SIZE;
        let index = index % PAGE_SIZE;
        (page, index)
    }

    fn absolute_index(&self, page: usize, index: usize) -> usize
    {
        page * PAGE_SIZE + index
    }

    pub fn set(&mut self, index: usize, data: T)
    {
        let (page, index) = Self::map_index(index);

        if page >= self.pages.len()
        {
            self.pages.resize(page + 1, [0; PAGE_SIZE]);
        }

        self.pages[page][index] = data;

        if index >= self.len
        {
            self.len = index + 1;
        }

        if (self.)
    }

    pub fn get(&self, index: usize) -> T
    {
        let (page, index) = self.map_index(index);
        self.pages[page * PAGE_SIZE + index]
    }

    pub fn remove(&mut self, index: usize)
    {
        let (page, index) = self.map_index(index);
    }

    pub fn swap(&mut self, index: usize, other: usize)
    {
        let (page, index) = self.map_index(index);
        let (page, other) = self.map_index(other);
    }
}
*/