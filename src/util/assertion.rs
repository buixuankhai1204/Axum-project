// use chrono::{DateTime, Utc};
// use itertools::Itertools;
// use std::{collections::HashMap, hash::Hash};
//
// pub fn eq<T>(result: &[T], expected: &[T]) -> bool
// where
//     T: Eq + Hash,
// {
//     fn count<T>(items: &[T]) -> HashMap<&T, usize>
//     where
//         T: Eq + Hash,
//     {
//         let mut cnt = HashMap::new();
//         for i in items {
//             *cnt.entry(i).or_insert(0) += 1
//         }
//         cnt
//     }
//     count(result) == count(expected)
// }
//
// pub fn vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
//     a.len() == b.len() && !a.iter().zip(b.iter()).any(|(a, b)| *a != *b)
// }
//
// pub fn compare_datetime(left: &DateTime<Utc>, right: &DateTime<Utc>) -> bool {
//     left.format("%d/%m/%Y %H:%M").to_string() == right.format("%d/%m/%Y %H:%M").to_string()
// }
//
// pub fn exist<T>(haystack: &[T], needle: &T) -> bool
// where
//     T: PartialEq,
// {
//     haystack.iter().any(|i| i == needle)
// }
//
// pub fn exist_all<T>(haystack: &[T], handful: &[T]) -> bool
// where
//     T: PartialEq,
// {
//     handful.iter().all(|i| haystack.contains(i))
// }
//
// pub fn is_sorted<I>(items: I, direction: Direction) -> bool
// where
//     I: IntoIterator,
//     I::Item: Ord + Clone,
// {
//     items.into_iter().tuple_windows().all(direction.as_closure())
// }
