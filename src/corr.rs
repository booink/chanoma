//! 置換テーブルを合成するための仕組みを定義するモジュールです。

mod item;
mod linear;
mod unification;
pub use item::Item;
pub use linear::Linear;
pub use unification::Unification;

use std::collections::{BTreeMap, HashSet};
use std::ops::Deref;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// [Item] の集合を表すための trait です。
pub trait Corr {
    fn items(&self) -> Vec<Item>;

    // traitコンテキスト内でconst fnが記述できないため。
    // 実装はしてあるが、本crate内のCorrを実装するそれぞれの型では再実装している。
    fn corr(self) -> Correspondence<Self>
    where
        Self: Sized,
    {
        Correspondence::new(self)
    }

    fn synthesize(&self) -> Correspondence<Synthesized> {
        Correspondence::new(Synthesized::new(self.items()))
    }
}

/// [Corr] の実装同士を合成する際に利用する一時的な構造体です。
#[derive(Clone, Debug)]
pub struct Synthesized {
    items: Vec<Item>,
}

impl Corr for Synthesized {
    fn items(&self) -> Vec<Item> {
        self.items.clone()
    }
}

impl Default for Synthesized {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl Synthesized {
    /// [Synthesized] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::corr::{Item, Synthesized};
    ///
    /// let items = vec![Item::new("a", "Ａ")];
    /// let synth = Synthesized::new(items);
    /// ```
    pub const fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    /// [Correspondence] を返します。
    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }

    /// [Item] の集合を key: 置換後, value: 置換前 の [BTreeMap] の形にして返します。
    pub fn item_map(&self) -> BTreeMap<&String, Vec<&String>> {
        let mut m = BTreeMap::new();
        for item in self.items.iter() {
            m.entry(&item.to).or_insert_with(Vec::new).push(&item.from);
        }
        m
    }
}

/// [Corr] の実装同士を合成して [Synthesized] 構造体を生成するために利用する構造体です。
#[derive(Debug, Clone)]
pub struct Correspondence<T> {
    inner: T,
}

impl<Synthesized: Default> Default for Correspondence<Synthesized> {
    fn default() -> Self {
        Self::new(Synthesized::default())
    }
}

impl<T> Correspondence<T> {
    /// [Correspondence] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::corr::{Correspondence, Synthesized};
    ///
    /// let synth = Synthesized::default();
    /// let corr = Correspondence::new(synth);
    /// ```
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Corr> Deref for Correspondence<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Corr> AsRef<Correspondence<T>> for Correspondence<T> {
    fn as_ref(&self) -> &Correspondence<T> {
        self
    }
}

impl<T> From<Correspondence<T>> for Synthesized
where
    T: Corr,
{
    fn from(f: Correspondence<T>) -> Self {
        Synthesized::new(f.inner.items())
    }
}

impl<T> From<&Correspondence<T>> for Synthesized
where
    T: Corr,
{
    fn from(f: &Correspondence<T>) -> Self {
        Synthesized::new(f.inner.items())
    }
}

impl<T> PartialEq for Correspondence<T>
where
    T: Corr,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.items() == other.inner.items()
    }
}

// 演算子のオーバーロード
impl<T, U> Add<&Correspondence<U>> for Correspondence<T>
where
    T: Corr,
    U: Corr,
{
    type Output = Correspondence<Synthesized>;

    fn add(self, rhs: &Correspondence<U>) -> Self::Output {
        let mut items: HashSet<Item> = HashSet::new();
        for item in self.inner.items() {
            items.insert(item);
        }
        for item in rhs.inner.items() {
            items.insert(item);
        }
        let mut items = items.into_iter().collect::<Vec<_>>();
        items.sort();
        Correspondence::new(Synthesized::new(items))
    }
}

impl<T> AddAssign<&Correspondence<T>> for Correspondence<Synthesized>
where
    T: Corr,
{
    fn add_assign(&mut self, rhs: &Correspondence<T>) {
        let mut items: HashSet<Item> = HashSet::new();
        for item in self.inner.items() {
            items.insert(item);
        }
        for item in rhs.inner.items() {
            items.insert(item);
        }
        let mut items = items.into_iter().collect::<Vec<_>>();
        items.sort();
        self.inner = Synthesized::new(items);
    }
}

impl<T, U> Sub<&Correspondence<U>> for Correspondence<T>
where
    T: Corr,
    U: Corr,
{
    type Output = Correspondence<Synthesized>;

    fn sub(self, rhs: &Correspondence<U>) -> Self::Output {
        let mut items = Vec::new();
        let comp_items = rhs.inner.items();
        for item in self.inner.items() {
            if !comp_items.contains(&item) {
                items.push(item);
            }
        }
        items.sort();
        Correspondence::new(Synthesized::new(items))
    }
}

impl<T> SubAssign<&Correspondence<T>> for Correspondence<Synthesized>
where
    T: Corr,
{
    fn sub_assign(&mut self, rhs: &Correspondence<T>) {
        let mut items = Vec::new();
        let comp_items = rhs.inner.items();
        for item in self.inner.items() {
            if !comp_items.contains(&item) {
                items.push(item);
            }
        }
        items.sort();
        self.inner = Synthesized::new(items);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct CorrA;
    impl Corr for CorrA {
        fn items(&self) -> Vec<Item> {
            vec![Item::new("a", "A")]
        }
    }

    struct CorrB;
    impl Corr for CorrB {
        fn items(&self) -> Vec<Item> {
            vec![Item::new("b", "B")]
        }
    }

    struct CorrAb;
    impl Corr for CorrAb {
        fn items(&self) -> Vec<Item> {
            vec![Item::new("a", "A"), Item::new("b", "B")]
        }
    }

    mod add {
        use super::*;

        #[test]
        fn no_duplicates() {
            let a = CorrA {};
            let b = CorrB {};
            let ab = a.corr() + &b.corr();
            assert_eq!(ab.items(), vec![Item::new("a", "A"), Item::new("b", "B")]);
        }

        #[test]
        fn duplicates() {
            let ab = CorrAb {};
            let b = CorrB {};
            let abb = ab.corr() + &b.corr();
            assert_eq!(abb.items(), vec![Item::new("a", "A"), Item::new("b", "B")]);
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn no_duplicates() {
            let a = CorrA {};
            let b = CorrB {};
            let ab = a.corr() - &b.corr();
            assert_eq!(ab.items(), vec![Item::new("a", "A")]);
        }

        #[test]
        fn duplicates() {
            let ab = CorrAb {};
            let b = CorrB {};
            let abb = ab.corr() - &b.corr();
            assert_eq!(abb.items(), vec![Item::new("a", "A")]);
        }
    }

    mod add_assign {
        use super::*;

        #[test]
        fn no_duplicates() {
            let mut a = CorrA {}.synthesize();
            let b = CorrB {};
            a += &b.corr();
            assert_eq!(a.items(), vec![Item::new("a", "A"), Item::new("b", "B")]);
        }

        #[test]
        fn duplicates() {
            let mut ab = CorrAb {}.synthesize();
            let b = CorrB {};
            ab += &b.corr();
            assert_eq!(ab.items(), vec![Item::new("a", "A"), Item::new("b", "B")]);
        }
    }

    mod sub_assign {
        use super::*;

        #[test]
        fn no_duplicates() {
            let mut a = CorrA {}.synthesize();
            let b = CorrB {};
            a -= &b.corr();
            assert_eq!(a.items(), vec![Item::new("a", "A")]);
        }

        #[test]
        fn duplicates() {
            let mut ab = CorrAb {}.synthesize();
            let b = CorrB {};
            ab -= &b.corr();
            assert_eq!(ab.items(), vec![Item::new("a", "A")]);
        }
    }
}
