// solution to https://codeforces.com/problemset/problem/16/E



use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::io;
use std::io::{stdin, BufRead, Read};
use std::ops;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct FishSet(u32);

impl FishSet {
    fn new(n: u32) -> Self {
        Self((1 << n) - 1)
    }

    fn empty() -> Self {
        Self(0)
    }

    fn pair(x: Fish, y: Fish) -> Self {
        FishSet::empty() + x + y
    }

    fn array(&self, arr: &mut [Fish; 18]) -> usize {
        self.into_iter()
            .enumerate()
            .map(|(i, fish)| arr[i] = fish)
            .count()
    }
}

impl fmt::Debug for FishSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        f.write_str(
            &self
                .into_iter()
                .map(|f| format!("{:?}", f))
                .collect::<Vec<String>>()
                .join(","),
        )?;
        f.write_str("}")
    }
}

impl ops::Sub<Fish> for FishSet {
    type Output = FishSet;

    fn sub(self, rhs: Fish) -> Self::Output {
        FishSet(self.0 & !(1 << rhs.0))
    }
}

impl ops::Add<Fish> for FishSet {
    type Output = FishSet;

    fn add(self, rhs: Fish) -> Self::Output {
        FishSet(self.0 | (1 << rhs.0))
    }
}

impl From<Fish> for FishSet {
    fn from(f: Fish) -> Self {
        FishSet(1 << f.0)
    }
}

impl IntoIterator for FishSet {
    type Item = Fish;
    type IntoIter = FishSetIter;

    fn into_iter(self) -> Self::IntoIter {
        FishSetIter::new(self)
    }
}

struct FishSetIter(FishSet, Option<Fish>);

impl FishSetIter {
    fn new(set: FishSet) -> Self {
        Self(set, Some(Fish(0)))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Fish(u32);

impl Iterator for FishSetIter {
    type Item = Fish;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_fish = self.1?.0;
        loop {
            if next_fish >= 18 {
                self.1 = None;
                return None;
            }
            let fish_present = self.0 .0 & (1 << next_fish);
            if fish_present != 0 {
                self.1 = Some(Fish(next_fish + 1));
                return Some(Fish(next_fish));
            }
            next_fish += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct WinSetFish<SetBits, FishBits>(u32, SetBits, FishBits);

impl<SetBits, FishBits> WinSetFish<SetBits, FishBits>
    where
        SetBits: U32 + Default,
        FishBits: U32 + Default,
{
    fn pair(winner: Fish, looser: Fish) -> Self {
        WinSetFish::new(winner, FishSet::pair(winner, looser))
    }

    fn new(fish: Fish, set: FishSet) -> Self {
        let ones = (1 << FishBits::u32()) - 1;
        WinSetFish(
            set.0 << FishBits::u32() | fish.0 & ones,
            SetBits::default(),
            FishBits::default(),
        )
    }

    fn fish(&self) -> Fish {
        let ones = (1 << FishBits::u32()) - 1;
        Fish(self.0 & ones)
    }

    fn set(&self) -> FishSet {
        FishSet(self.0 >> FishBits::u32())
    }
}

trait U32 {
    fn u32() -> u32;
}

#[derive(Debug, Clone, Copy, Default)]
struct Const18;
impl U32 for Const18 {
    #[inline(always)]
    fn u32() -> u32 {
        18
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Const5;
impl U32 for Const5 {
    #[inline(always)]
    fn u32() -> u32 {
        5
    }
}

type Win = WinFishSet<Const18, Const5>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct WinFishSet<SetBits, FishBits>(u32, SetBits, FishBits);

impl<SetBits, FishBits> WinFishSet<SetBits, FishBits>
    where
        SetBits: U32 + Default,
        FishBits: U32 + Default,
{
    fn pair(winner: Fish, looser: Fish) -> Self {
        WinFishSet::new(winner, FishSet::pair(winner, looser))
    }

    fn new(fish: Fish, set: FishSet) -> Self {
        let ones = (1 << SetBits::u32()) - 1;
        WinFishSet(
            fish.0 << SetBits::u32() | set.0 & ones,
            SetBits::default(),
            FishBits::default(),
        )
    }

    fn fish(&self) -> Fish {
        Fish(self.0 >> SetBits::u32())
    }

    fn set(&self) -> FishSet {
        let ones = (1 << SetBits::u32()) - 1;
        FishSet(self.0 & ones)
    }
}

type Float = f64;

type WinProbability = WinProbabilityGeneric<Const18, Const5>;

#[derive(Debug)]
struct WinProbabilityGeneric<SetBits, FishBits>(Vec<Float>, SetBits, FishBits);

impl<SetBits, FishBits> WinProbabilityGeneric<SetBits, FishBits>
    where
        SetBits: U32 + Default,
        FishBits: U32 + Default,
{
    fn new() -> Self {
        WinProbabilityGeneric(
            vec![-1.; 1 << (SetBits::u32() + FishBits::u32())],
            SetBits::default(),
            FishBits::default(),
        )
    }

    fn insert(&mut self, win: Win, probability: Float) {
        self.0[win.0 as usize] = probability;
    }

    fn get(&self, target: Win) -> Option<Float> {
        let probability = self.0[target.0 as usize];
        if probability < 0. {
            None
        } else {
            Some(probability)
        }
    }

    fn wins(&mut self, target: Win) -> Float {
        if let Some(probability) = self.get(target) {
            return probability;
        }

        let m = target.set().into_iter().count();
        let branch_count = to_float(m * (m - 1) / 2);

        let result = (target.set() - target.fish())
            .into_iter()
            .map(|first_looser: Fish| -> Float {
                let survivors = target.set() - first_looser;
                survivors
                    .into_iter()
                    .map(|first_winner| self.wins(Win::pair(first_winner, first_looser)))
                    .sum::<Float>()
                    * self.wins(Win::new(target.fish(), survivors))
            })
            .sum::<Float>()
            / branch_count;
        self.insert(target.clone(), result);
        result
    }
}

fn to_float(x: usize) -> Float {
    let x: u16 = x.try_into().unwrap();
    x.try_into().unwrap()
}

fn permutations(k: u32, n: u32) -> Vec<FishSet> {
    _permutations(FishSet::empty(), k, n, 0)
}

fn _permutations(set: FishSet, k: u32, n: u32, start: u32) -> Vec<FishSet> {
    let mut arr = Vec::new();
    for fish in (start..n).map(|i| Fish(i)) {
        let new_set = set + fish;
        if new_set == set {
            continue;
        }
        if k >= 2 {
            arr.extend(_permutations(new_set, k - 1, n, fish.0))
        } else {
            arr.push(new_set)
        }
    }
    arr
}

fn prepare(memoized: &mut WinProbability, n: u32) {
    let mut fish_buffer = [Fish(0); 18];
    for k in 3..n - 1 {
        for set in permutations(k, n) {
            let len = set.array(&mut fish_buffer);

            let probability_sum = (0..len - 1)
                .map(|i| memoized.wins(Win::new(fish_buffer[i], set)))
                .sum::<Float>();

            // we can skip the calculation of the last probability
            // because it must be equal 1.-sum_of_other_probabilities
            memoized.insert(Win::new(fish_buffer[len - 1], set), 1. - probability_sum)
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read n failed");
    let n = buffer.trim().parse().expect("failed to parse n");
    // let n = 4;
    let memoized = WinProbability::new();
    let mut memoized = read_probabilities(&mut io::stdin().lock(), &mut buffer, n, memoized);
    prepare(&mut memoized, n);

    // println!("{:?}", memoized);
    for i in 0..n {
        let member = Win::new(Fish(i), FishSet::new(n));
        let probability = memoized.wins(member);
        print!("{} ", fmt_float(probability));
    }
    // println!();
    // println!("{:?}", memoized);
}

fn read_probabilities<S>(
    stream: &mut S,
    buffer: &mut String,
    n: u32,
    mut probabilities: WinProbability,
) -> WinProbability
    where
        S: BufRead,
{
    for i in 0..n {
        buffer.clear();
        stream
            .read_line(buffer)
            .expect(&format!("failed to read line {}", i));
        let row = buffer.split(" ").map(|v| {
            v.trim()
                .parse::<Float>()
                .expect(&format!("failed to parse value: {}", v))
        });
        for (j, probability) in row.enumerate() {
            let j = j.try_into().unwrap();
            let win = Win::new(Fish(i), FishSet::empty() + Fish(i) + Fish(j));
            probabilities.insert(win, probability);
        }
    }
    probabilities
}

fn fmt_float(x: Float) -> String {
    return format!("{:.6}", x);
}

mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::num::NonZeroI32;

    #[test]
    fn permutations_2_3() {
        assert_eq!(
            permutations(2, 3),
            vec![
                FishSet::empty() + Fish(0) + Fish(1),
                FishSet::empty() + Fish(0) + Fish(2),
                FishSet::empty() + Fish(1) + Fish(2),
            ]
        )
    }

    #[test]
    fn permutations_3_3() {
        assert_eq!(
            permutations(3, 3),
            vec![FishSet::empty() + Fish(0) + Fish(1) + Fish(2),]
        )
    }

    #[test]
    fn permutations_3_4() {
        assert_eq!(
            permutations(3, 4),
            vec![
                FishSet::empty() + Fish(0) + Fish(1) + Fish(2),
                FishSet::empty() + Fish(0) + Fish(1) + Fish(3),
                FishSet::empty() + Fish(0) + Fish(2) + Fish(3),
                FishSet::empty() + Fish(1) + Fish(2) + Fish(3),
            ]
        )
    }

    #[test]
    fn iter_fish_set_all() {
        let mut it = FishSet::new(5).into_iter();
        assert_eq!(it.next().unwrap(), Fish(0));
        assert_eq!(it.next().unwrap(), Fish(1));
        assert_eq!(it.next().unwrap(), Fish(2));
        assert_eq!(it.next().unwrap(), Fish(3));
        assert_eq!(it.next().unwrap(), Fish(4));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_fish_set() {
        let mut set = FishSet::new(5);
        set = set - Fish(0) - Fish(3);
        let mut it = set.into_iter();
        assert_eq!(it.next().unwrap(), Fish(1));
        assert_eq!(it.next().unwrap(), Fish(2));
        assert_eq!(it.next().unwrap(), Fish(4));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn equal_probability_3() {
        let mut proba = WinProbability::new();
        proba.insert(Win::pair(Fish(0), Fish(1)), 0.5);
        proba.insert(Win::pair(Fish(1), Fish(0)), 0.5);
        proba.insert(Win::pair(Fish(0), Fish(2)), 0.5);
        proba.insert(Win::pair(Fish(2), Fish(0)), 0.5);
        proba.insert(Win::pair(Fish(1), Fish(2)), 0.5);
        proba.insert(Win::pair(Fish(2), Fish(1)), 0.5);
        for i in 0..3 {
            let actual = proba.wins(Win::new(Fish(i), FishSet::new(3)));
            assert_eq!(fmt_float(actual), "0.333333");
        }
    }

    fn get_result<F>(n: u32, f: F) -> Vec<String>
        where
            F: Fn(&mut WinProbability) -> (),
    {
        let mut proba = WinProbability::new();
        f(&mut proba);
        prepare(&mut proba, n);

        (0..n)
            .map(|i| proba.wins(Win::new(Fish(i), FishSet::new(n))))
            .map(fmt_float)
            .collect()
    }

    #[test]
    fn equal_probability_2() {
        let proba = get_result(2, |proba| {
            proba.insert(Win::pair(Fish(0), Fish(1)), 0.5);
            proba.insert(Win::pair(Fish(1), Fish(0)), 0.5);
        });
        assert_eq!(proba, vec!["0.500000", "0.500000"]);
    }

    #[test]
    fn win_probability() {
        let actual = get_result(3, |proba| {
            proba.insert(Win::pair(Fish(0), Fish(1)), 0.5);
            proba.insert(Win::pair(Fish(1), Fish(0)), 0.5);
            proba.insert(Win::pair(Fish(0), Fish(2)), 0.4);
            proba.insert(Win::pair(Fish(2), Fish(0)), 0.6);
            proba.insert(Win::pair(Fish(1), Fish(2)), 0.3);
            proba.insert(Win::pair(Fish(2), Fish(1)), 0.7);
        });
        let expected = vec!["0.276667", "0.226667", "0.496667"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn win_probability_zeros() {
        let mut actual = get_result(3, |proba| {
            proba.insert(Win::pair(Fish(0), Fish(1)), 1.0);
            proba.insert(Win::pair(Fish(1), Fish(0)), 0.0);
            proba.insert(Win::pair(Fish(0), Fish(2)), 1.0);
            proba.insert(Win::pair(Fish(2), Fish(0)), 0.0);
            proba.insert(Win::pair(Fish(1), Fish(2)), 0.5);
            proba.insert(Win::pair(Fish(2), Fish(1)), 0.5);
        });
        let expected = vec!["1.000000", "0.000000", "0.000000"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn large() {
        let n = 18;
        let actual = get_result(n, |proba| {
            for i in 0..n {
                for j in 0..n {
                    let val = if i == j { 0.0 } else { 0.5 };
                    proba.insert(Win::pair(Fish(i), Fish(j)), val);
                }
            }
        });
        let expected: Vec<String> = (0..n).map(|_| fmt_float(1. / (n as Float))).collect();
        assert_eq!(actual, expected);
    }
}