#![feature(proc_macro)]

extern crate delegable_derive;

use delegable_derive::delegable;

#[delegable]
pub trait VecFacade<T> {
    fn capacity(&self) -> usize;
    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
    fn into_boxed_slice(self) -> Box<[T]>;
    fn truncate(&mut self, len: usize);
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
    unsafe fn set_len(&mut self, len: usize);
    fn swap_remove(&mut self, index: usize) -> T;
    fn insert(&mut self, index: usize, element: T);
    fn remove(&mut self, index: usize) -> T;
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool;
    fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>;
    fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool;
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn append(&mut self, other: &mut Vec<T>);
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn split_off(&mut self, at: usize) -> Vec<T>;
}

impl<T> VecFacade<T> for Vec<T> {
    fn capacity(&self) -> usize {
        self.capacity()
    }
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional)
    }
    fn reserve_exact(&mut self, additional: usize) {
        self.reserve_exact(additional)
    }
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit()
    }
    fn into_boxed_slice(self) -> Box<[T]> {
        self.into_boxed_slice()
    }
    fn truncate(&mut self, len: usize) {
        self.truncate(len)
    }
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
    unsafe fn set_len(&mut self, len: usize) {
        self.set_len(len)
    }
    fn swap_remove(&mut self, index: usize) -> T {
        self.swap_remove(index)
    }
    fn insert(&mut self, index: usize, element: T) {
        self.insert(index, element)
    }
    fn remove(&mut self, index: usize) -> T {
        self.remove(index)
    }
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain(f)
    }
    fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>,
    {
        self.dedup_by_key(key)
    }
    fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.dedup_by(same_bucket)
    }
    fn push(&mut self, value: T) {
        self.push(value)
    }
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
    fn append(&mut self, other: &mut Vec<T>) {
        self.append(other)
    }
    fn clear(&mut self) {
        self.clear()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn split_off(&mut self, at: usize) -> Vec<T> {
        self.split_off(at)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct U64Vec(Vec<u64>);

    impl delegate_VecFacade for U64Vec {
        type gen_T = u64;
        type Inner = Vec<u64>;

        fn inner(&self) -> &Self::Inner {
            &self.0
        }
        fn inner_mut(&mut self) -> &mut Self::Inner {
            &mut self.0
        }
        fn into_inner(self) -> Self::Inner {
            self.0
        }
        fn from_inner(delegate: Self::Inner) -> Self {
            Self { 0: delegate }
        }
    }

    #[test]
    fn push_to_vec() {
        let mut proxy_vec = U64Vec(vec![0, 2]);
        proxy_vec.push(42);
        assert_eq!(proxy_vec.len(), 3);
    }

}
