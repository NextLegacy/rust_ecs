use std::collections::HashMap;

use super::ECSStorage;

pub trait ComponentQuery<'a> {
    type Iter: Iterator;
    fn query(storage: &'a ECSStorage) -> Self::Iter;
}

pub trait ComponentQueryMut<'a> {
    type Iter: Iterator;
    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter;
}

impl<'a, T1: 'static> ComponentQuery<'a> for (&'a T1,) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1)> + 'a>;

    fn query(storage: &'a ECSStorage) -> Self::Iter {
        storage.iter_components::<T1>()
    }
}

impl<'a, T1: 'static> ComponentQueryMut<'a> for (&'a mut T1,) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a mut T1)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        storage.iter_components_mut::<T1>()
    }
}

impl<'a, T1: 'static, T2: 'static> ComponentQuery<'a> for (&'a T1, &'a T2) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1, &'a T2)> + 'a>;

    fn query(storage: &'a ECSStorage) -> Self::Iter {
        let iter1 = storage.iter_components::<T1>().collect::<HashMap<_, _>>();
        let iter2 = storage.iter_components::<T2>();

        Box::new(iter2.filter_map(move |(i2, t2)| {
            iter1.get(&i2).map(|t1| (i2, *t1, t2))
        }))
    }
}
impl<'a, T1: 'static, T2: 'static> ComponentQueryMut<'a> for (&'a mut T1, &'a T2) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a mut T1, &'a T2)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *mut T1> = storage.iter_components_mut::<T1>()
            .map(|(i, t1)| (i, t1 as *mut T1))
            .collect();

        let iter2: Vec<(usize, *mut T2)> = storage.iter_components_mut::<T2>()
            .map(|(i, t2)| (i, t2 as *mut T2))
            .collect();

        Box::new(iter2.into_iter().filter_map(move |(i2, t2)| {
            iter1.get(&i2).map(|&t1| (i2, unsafe { &mut *t1 }, unsafe { &*t2 }))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static> ComponentQueryMut<'a> for (&'a T1, &'a mut T2) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1, &'a mut T2)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *const T1> = storage.iter_components::<T1>()
            .map(|(i, t1)| (i, t1 as *const T1))
            .collect();

        let iter2: Vec<(usize, *mut T2)> = storage.iter_components_mut::<T2>()
            .map(|(i, t2)| (i, t2 as *mut T2))
            .collect();

        Box::new(iter2.into_iter().filter_map(move |(i2, t2)| {
            iter1.get(&i2).map(|&t1| (i2, unsafe { &*t1 }, unsafe { &mut *t2 }))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static> ComponentQueryMut<'a> for (&'a mut T1, &'a mut T2) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a mut T1, &'a mut T2)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *mut T1> = storage.iter_components_mut::<T1>()
            .map(|(i, t1)| (i, t1 as *mut T1))
            .collect();

        let iter2: Vec<(usize, *mut T2)> = storage.iter_components_mut::<T2>()
            .map(|(i, t2)| (i, t2 as *mut T2))
            .collect();

        Box::new(iter2.into_iter().filter_map(move |(i2, t2)| {
            iter1.get(&i2).map(|&t1| (i2, unsafe { &mut *t1 }, unsafe { &mut *t2 }))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQuery<'a> for (&'a T1, &'a T2, &'a T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1, &'a T2, &'a T3)> + 'a>;

    fn query(storage: &'a ECSStorage) -> Self::Iter {
        let iter1 = storage.iter_components::<T1>().collect::<HashMap<_, _>>();
        let iter2 = storage.iter_components::<T2>().collect::<HashMap<_, _>>();
        let iter3 = storage.iter_components::<T3>();

        Box::new(iter3.filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|t1| iter2.get(&i3).map(|t2| (i3, *t1, *t2, t3)))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a T1, &'a T2, &'a T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1, &'a T2, &'a T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *const T1> = storage.iter_components::<T1>()
            .map(|(i, t1)| (i, t1 as *const T1))
            .collect();

        let iter2: HashMap<usize, *const T2> = storage.iter_components::<T2>()
            .map(|(i, t2)| (i, t2 as *const T2))
            .collect();

        let iter3: Vec<(usize, *const T3)> = storage.iter_components::<T3>()
            .map(|(i, t3)| (i, t3 as *const T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &*t1 }, unsafe { &*t2 }, unsafe { &*t3 })))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a mut T1, &'a T2, &'a T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a mut T1, &'a T2, &'a T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *mut T1> = storage.iter_components_mut::<T1>()
            .map(|(i, t1)| (i, t1 as *mut T1))
            .collect();

        let iter2: HashMap<usize, *const T2> = storage.iter_components::<T2>()
            .map(|(i, t2)| (i, t2 as *const T2))
            .collect();

        let iter3: Vec<(usize, *mut T3)> = storage.iter_components_mut::<T3>()
            .map(|(i, t3)| (i, t3 as *mut T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &mut *t1 }, unsafe { &*t2 }, unsafe { &*t3 })))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a T1, &'a mut T2, &'a T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1, &'a mut T2, &'a T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *const T1> = storage.iter_components::<T1>()
            .map(|(i, t1)| (i, t1 as *const T1))
            .collect();

        let iter2: HashMap<usize, *mut T2> = storage.iter_components_mut::<T2>()
            .map(|(i, t2)| (i, t2 as *mut T2))
            .collect();

        let iter3: Vec<(usize, *mut T3)> = storage.iter_components_mut::<T3>()
            .map(|(i, t3)| (i, t3 as *mut T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &*t1 }, unsafe { &mut *t2 }, unsafe { &*t3 })))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a mut T1, &'a mut T2, &'a T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a mut T1, &'a mut T2, &'a T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *mut T1> = storage.iter_components_mut::<T1>()
            .map(|(i, t1)| (i, t1 as *mut T1))
            .collect();

        let iter2: HashMap<usize, *mut T2> = storage.iter_components_mut::<T2>()
            .map(|(i, t2)| (i, t2 as *mut T2))
            .collect();

        let iter3: Vec<(usize, *const T3)> = storage.iter_components::<T3>()
            .map(|(i, t3)| (i, t3 as *const T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &mut *t1 }, unsafe { &mut *t2 }, unsafe { &*t3 })))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a T1, &'a T2, &'a mut T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1, &'a T2, &'a mut T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *const T1> = storage.iter_components::<T1>()
            .map(|(i, t1)| (i, t1 as *const T1))
            .collect();

        let iter2: HashMap<usize, *const T2> = storage.iter_components::<T2>()
            .map(|(i, t2)| (i, t2 as *const T2))
            .collect();

        let iter3: Vec<(usize, *mut T3)> = storage.iter_components_mut::<T3>()
            .map(|(i, t3)| (i, t3 as *mut T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &*t1 }, unsafe { &*t2 }, unsafe { &mut *t3 })))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a mut T1, &'a T2, &'a mut T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a mut T1, &'a T2, &'a mut T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *mut T1> = storage.iter_components_mut::<T1>()
            .map(|(i, t1)| (i, t1 as *mut T1))
            .collect();

        let iter2: HashMap<usize, *const T2> = storage.iter_components::<T2>()
            .map(|(i, t2)| (i, t2 as *const T2))
            .collect();

        let iter3: Vec<(usize, *mut T3)> = storage.iter_components_mut::<T3>()
            .map(|(i, t3)| (i, t3 as *mut T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &mut *t1 }, unsafe { &*t2 }, unsafe { &mut *t3 })))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a T1, &'a mut T2, &'a mut T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a T1, &'a mut T2, &'a mut T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *const T1> = storage.iter_components::<T1>()
            .map(|(i, t1)| (i, t1 as *const T1))
            .collect();

        let iter2: HashMap<usize, *mut T2> = storage.iter_components_mut::<T2>()
            .map(|(i, t2)| (i, t2 as *mut T2))
            .collect();

        let iter3: Vec<(usize, *mut T3)> = storage.iter_components_mut::<T3>()
            .map(|(i, t3)| (i, t3 as *mut T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &*t1 }, unsafe { &mut *t2 }, unsafe { &mut *t3 })))
        }))
    }
}

impl <'a, T1: 'static, T2: 'static, T3: 'static> ComponentQueryMut<'a> for (&'a mut T1, &'a mut T2, &'a mut T3) {
    type Iter = Box<dyn Iterator<Item = (usize, &'a mut T1, &'a mut T2, &'a mut T3)> + 'a>;

    fn query_mut(storage: &'a mut ECSStorage) -> Self::Iter {
        let iter1: HashMap<usize, *mut T1> = storage.iter_components_mut::<T1>()
            .map(|(i, t1)| (i, t1 as *mut T1))
            .collect();

        let iter2: HashMap<usize, *mut T2> = storage.iter_components_mut::<T2>()
            .map(|(i, t2)| (i, t2 as *mut T2))
            .collect();

        let iter3: Vec<(usize, *mut T3)> = storage.iter_components_mut::<T3>()
            .map(|(i, t3)| (i, t3 as *mut T3))
            .collect();

        Box::new(iter3.into_iter().filter_map(move |(i3, t3)| {
            iter1.get(&i3).and_then(|&t1| iter2.get(&i3).map(|&t2| (i3, unsafe { &mut *t1 }, unsafe { &mut *t2 }, unsafe { &mut *t3 })))
        }))
    }
}

