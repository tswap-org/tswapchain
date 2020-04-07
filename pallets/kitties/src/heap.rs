use rstd::vec::Vec;
use support::{Parameter, StorageValue};

pub trait Compare {
    type A;
    fn closer_than(x: &Self::A, y: &Self::A) -> bool;
}

pub struct Heap<T, C, S> (rstd::marker::PhantomData<(T, C, S)>);

impl<T, C, S> Heap<T, C, S>
    where T: Parameter,
          C: Compare<A=T>,
          S: StorageValue<Vec<T>, Query=Vec<T>>,
{
    pub fn push(item: T) {
        let mut store = S::get();
        Self::push_into_store(&mut store, item);
        S::put(store);
    }

    pub fn push_vec(items: Vec<T>) {
        let mut store = S::get();
        for item in items {
            Self::push_into_store(&mut store, item);
        }
        S::put(store);
    }

    pub fn pop() -> Option<T> {
        let mut store = S::get();
        let top = Self::pop_from_store(&mut store);
        S::put(store);
        top
    }

    pub fn pop_vec(stake: &T) -> Vec<T> {
        let mut store = S::get();
        let vec = Self::pop_by_stake(&mut store, stake);
        S::put(store);
        vec
    }

    fn push_into_store(store: &mut Vec<T>, item: T) {
        store.push(item);
        let last = store.len() - 1;
        Self::shift_up(store, last);
    }

    fn pop_by_stake(store: &mut Vec<T>, stack: &T) -> Vec<T> {
        let mut vec = Vec::new();
        let peek_top = store.get(0);
        match peek_top {
            None => vec,
            Some(peek_top) => {
                if C::closer_than(peek_top, stack) {
                    let top = Self::pop_from_store(store);
                    match top {
                        None => vec,
                        Some(top) => {
                            vec.push(top);
                            vec.append(&mut Self::pop_by_stake(store, stack));
                            vec
                        }
                    }
                } else {
                    vec
                }
            }
        }
    }

    fn pop_from_store(store: &mut Vec<T>) -> Option<T> {
        match store.len() {
            0 => None,
            1 => store.pop(),
            _ => {
                let last = store.len() - 1;
                store.swap(0, last);
                let top = store.pop();
                Self.shift_down(store, 0);
                top
            }
        }
    }
    
    fn parent_idx(child: usize) -> Option<usize> {
        mathc child {
            0 => None,
            1..=2 => Some(0),
            _ => {
                if child % 2 == 1 {
                    Some((child - 1)/2)
                } else {
                    Some((child - 2)/2)
                }
            }
        }
    }

    fn left_idx(store: &[T], parent: usize) -> Option<usize> {
        let left: usize = parent * 2 + 1;
        if left < store.len() {
            Some(left)
        } else {
            None
        }
    }

    fn right_idx(store: &[T], parent: usize) -> Option<usize> {
        let right: usize = parent * 2 + 2;
        if right < store.len() {
            Some(right)
        } else {
            None
        }
    }

    fn shift_up(store: &mut [T], idx: usize) {
        match Self::parent_idx(idx) {
            None => {}
            Some(par) => {
                if C::closer_than(&store[idx], &store[par]) {
                    store.swap(idx, par);
                    Self::shift_up(store, par);
                }
            }
        }
    }

    fn shift_down(store: &mut [T], idx: usize) {
        match Self::left_idx(store, idx) {
            None => {}
            Some(left) => {
                match Self::right_idx(store, idx) {
                    None => {
                        if C::closer_than(&store[left], &store[idx]) {
                            store.swap(idx, left);
                            Self::shift_down(store, left);
                        }
                    }
                    Some(right) => {
                        let closer = 
                            if C::closer_than(&sotre[left], &store[right]) {
                                left
                            } else {
                                right
                            };
                        if C::closer_than(&store[closer], &store[idx]) {
                            store.swap(idx, closer);
                            Self::shift_down(store, closer);
                        }
                    }
                }
            }
        }
    }
}