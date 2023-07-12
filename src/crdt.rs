use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct GSet<T> {
    add_set: HashSet<T>,
}

impl<T> GSet<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            add_set: HashSet::new(),
        }
    }

    pub fn add(&mut self, element: T) -> bool {
        self.add_set.insert(element)
    }

    pub fn merge(mut self, other: Self) {
        self.add_set = self
            .add_set
            .union(&other.add_set)
            .map(|a| a.clone())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct TwoPSet<T> {
    add_set: GSet<T>,
    remove_set: GSet<T>,
}
impl<T> TwoPSet<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            add_set: GSet::new(),
            remove_set: GSet::new(),
        }
    }

    // pub fn query(&mut self) -> HashSet<T> {
    //     self.a
    //         .query()
    //         .a
    //         .difference(&self.remove_set.query().a)
    //         .map(|x| *x)
    //         .collect()
    // }

    pub fn add(&mut self, element: T) -> bool {
        self.add_set.add(element)
    }
    pub fn sub(&mut self, element: T) -> bool {
        self.remove_set.add(element)
    }

    pub fn merge(self, other: Self) {
        self.add_set.merge(other.add_set);
        self.remove_set.merge(other.remove_set);
    }
}

// TEST
//     let mut gset = GSet::new();
//     gset.add(3);
//     gset.add(3);
//     gset.add(5);
//     dbg!(&gset);

//     let mut two_p_set = TwoPSet::new();
//     two_p_set.add(3);
//     two_p_set.add(3);
//     two_p_set.add(5);
//     dbg!(&two_p_set);
//     dbg!(&two_p_set);
//     two_p_set.sub(3);
//     dbg!(&two_p_set);
//     dbg!(&two_p_set);
