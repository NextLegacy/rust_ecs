use std::hash::Hash;

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

    pub fn set(&mut self, index: usize, value: bool)
    {
        let (i, b) = (index / (usize::BITS as usize), index % (usize::BITS as usize));
        if i >= self.bits.len()
        {
            self.bits.resize(i + 1, 0);
        }
        self.bits[i] = (self.bits[i] & !(1 << b)) | ((value as usize) << b);
    }

    pub fn on(&mut self, index: usize)
    {
        self.set(index, true);
    }

    pub fn off(&mut self, index: usize)
    {
        self.set(index, false);
    }

    pub fn clear(&mut self)
    {
        self.bits.clear();
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

    pub fn union(&mut self, other: &Self) -> &Self
    {
        for (i, &b) in other.bits.iter().enumerate()
        {
            if i >= self.bits.len()
            {
                self.bits.resize(i + 1, 0);
            }
            self.bits[i] |= b;
        }
        self
    }

    pub fn or(&self, other: &Self) -> Self
    {
        let mut result = self.clone();
        result.union(other);
        result
    }

    pub fn intersection(&mut self, other: &Self) -> &Self
    {
        for (i, &b) in other.bits.iter().enumerate()
        {
            if i < self.bits.len()
            {
                self.bits[i] &= b;
            }
        }
        self
    }

    pub fn difference(&mut self, other: &Self) -> &Self
    {
        for (i, &b) in other.bits.iter().enumerate()
        {
            if i < self.bits.len()
            {
                self.bits[i] &= !b;
            }
        }
        self
    }

    pub fn not(&self) -> Self
    {
        let mut result = self.clone();
        for b in result.bits.iter_mut()
        {
            *b = !*b;
        }
        result
    }

    pub fn and(&self, other: &Self) -> Self
    {
        let mut result = self.clone();
        result.intersection(other);
        result
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (usize, bool)> + '_
    {
        self.bits.iter().enumerate().flat_map(|(i, &b)| (0..usize::BITS as usize).map(move |j| (i * (usize::BITS as usize) + j, b & (1 << j) != 0)))
    }

    pub fn len(&self) -> usize
    {
        self.bits.len() * 64
    }

    pub fn data(&self) -> &[usize]
    {
        &self.bits
    }

    pub fn clone(&self) -> Self
    {
        Self
        {
            bits: self.bits.clone(),
        }
    }
}

impl Hash for BitSet
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        self.bits.hash(state);
    }
}