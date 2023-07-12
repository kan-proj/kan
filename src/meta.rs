#[derive(Debug, Clone)]
pub struct Metagraph<'a, O: ?Sized> {
    objects: Vec<&'a O>,
    arrows: Vec<(&'a O, &'a O)>,
}

impl<'a, O: PartialEq> Metagraph<'a, O> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            arrows: Vec::new(),
        }
    }

    fn dom(arrow: &(&'a O, &'a O)) -> &'a O {
        arrow.0
    }
    fn cod(arrow: &(&'a O, &'a O)) -> &'a O {
        arrow.1
    }
}

#[derive(Debug, Clone)]
pub struct Metacategory<'a, O: ?Sized> {
    metagraph: Metagraph<'a, O>,
}

impl<'a, O: PartialEq> Metacategory<'a, O> {
    pub fn new() -> Self {
        Self {
            metagraph: Metagraph::new(),
        }
    }
    pub fn hom(&self, b: O, c: O) -> Vec<&(&'a O, &'a O)> {
        let mut hom = Vec::new();
        self.metagraph.arrows.iter().for_each(|arrow| {
            if Metagraph::<'a, O>::dom(arrow) == &b && Metagraph::<'a, O>::cod(arrow) == &c {
                hom.push(arrow);
            }
        });
        hom
    }
}

#[cfg(test)]
mod tests {
    use super::Metacategory;

    #[test]
    fn hom() {
        let mut metacat: Metacategory<'_, usize> = Metacategory::new();

        metacat.metagraph.objects = vec![&1, &2, &3, &4];
        metacat.metagraph.arrows = vec![(&1, &3), (&3, &4)];

        // let mut metacat: Metacategory<'_, str> = Metacategory::new();
    }
}
