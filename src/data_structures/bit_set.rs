pub struct BitSet
{
    bits: Vec<usize>,
}

impl BitSet
{
    pub fn new() -> Self
    {
        Self
        {
            bits: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self
    {
        Self
        {
            bits: Vec::with_capacity(capacity),
        }
    }

    pub fn set(&mut self, index: usize)
    {
        let (i, b) = (index / (usize::BITS as usize), index % (usize::BITS as usize));
        if i >= self.bits.len()
        {
            self.bits.resize(i + 1, 0);
        }
        self.bits[i] |= 1 << b;
    }

    pub fn clear(&mut self, index: usize)
    {
        let (i, b) = (index / (usize::BITS as usize), index % (usize::BITS as usize));
        if i < self.bits.len()
        {
            self.bits[i] &= !(1 << b);
        }
    }

    pub fn get(&self, index: usize) -> bool
    {
        let (i, b) = (index / (usize::BITS as usize), index % (usize::BITS as usize));
        if i < self.bits.len()
        {
            self.bits[i] & (1 << b) != 0
        }
        else
        {
            false
        }
    }

    pub fn clear_all(&mut self)
    {
        self.bits.clear();
    }

    pub fn len(&self) -> usize
    {
        self.bits.len() * 64
    }

    pub fn data(&self) -> &[usize]
    {
        &self.bits
    }
}