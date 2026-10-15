//! Piles bornées et transactions d'objets, indépendantes de l'UI et des proxies.
use bevy::prelude::*;

pub const STACK_LIMIT: u32 = 20;
pub const CARRIED_SLOTS: usize = 8;
pub const CHEST_SLOTS: usize = 16;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Item {
    Alloy,
    Circuit,
}
impl Item {
    pub const ALL: [Self; 2] = [Self::Alloy, Self::Circuit];
    pub fn name(self) -> &'static str {
        match self {
            Self::Alloy => "Alliage",
            Self::Circuit => "Circuit",
        }
    }
    pub fn code(self) -> u32 {
        match self {
            Self::Alloy => 0,
            Self::Circuit => 1,
        }
    }
    pub fn from_code(code: u32) -> Option<Self> {
        Self::ALL.get(code as usize).copied()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stack {
    pub item: Item,
    pub quantity: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inventory {
    pub slots: Vec<Option<Stack>>,
}
impl Default for Inventory {
    fn default() -> Self {
        Self::new(CARRIED_SLOTS)
    }
}
impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
        }
    }
    pub fn valid(&self, capacity: usize) -> bool {
        self.slots.len() == capacity
            && self
                .slots
                .iter()
                .flatten()
                .all(|s| (1..=STACK_LIMIT).contains(&s.quantity))
    }
    pub fn used(&self) -> usize {
        self.slots.iter().flatten().count()
    }
    pub fn count(&self, item: Item) -> u32 {
        self.slots
            .iter()
            .flatten()
            .filter(|s| s.item == item)
            .map(|s| s.quantity)
            .sum()
    }
    pub fn room(&self, item: Item) -> u32 {
        self.slots
            .iter()
            .map(|s| match s {
                None => STACK_LIMIT,
                Some(s) if s.item == item => STACK_LIMIT - s.quantity,
                _ => 0,
            })
            .sum()
    }
    pub fn add(&mut self, item: Item, quantity: u32) -> Result<(), &'static str> {
        if quantity == 0 || self.room(item) < quantity {
            return Err("Inventaire plein : aucune récolte ni transfert");
        }
        let mut rest = quantity;
        for s in self.slots.iter_mut().flatten().filter(|s| s.item == item) {
            let n = rest.min(STACK_LIMIT - s.quantity);
            s.quantity += n;
            rest -= n;
        }
        for s in self.slots.iter_mut().filter(|s| s.is_none()) {
            if rest == 0 {
                break;
            }
            let n = rest.min(STACK_LIMIT);
            *s = Some(Stack { item, quantity: n });
            rest -= n;
        }
        Ok(())
    }
    pub fn pay(&mut self, recipe: &[(Item, u32)]) -> Result<(), &'static str> {
        // Agréger aussi les recettes répétant un type avant le premier débit.
        let costs = Item::ALL.map(|item| {
            (
                item,
                recipe
                    .iter()
                    .filter(|(i, _)| *i == item)
                    .map(|(_, n)| u64::from(*n))
                    .sum::<u64>(),
            )
        });
        if costs.iter().any(|(i, n)| u64::from(self.count(*i)) < *n) {
            return Err("Ressources portées insuffisantes");
        }
        for (item, cost) in costs {
            let mut rest = cost as u32;
            for slot in &mut self.slots {
                if let Some(s) = slot
                    && s.item == item
                {
                    let n = rest.min(s.quantity);
                    s.quantity -= n;
                    rest -= n;
                    if s.quantity == 0 {
                        *slot = None;
                    }
                }
            }
        }
        Ok(())
    }
    pub fn organize(&mut self) {
        let quantities = Item::ALL.map(|i| (i, self.count(i)));
        self.slots.fill(None);
        for (i, n) in quantities {
            if n > 0 {
                self.add(i, n).expect("Capacité conservée au rangement");
            }
        }
    }
    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), &'static str> {
        if a >= self.slots.len() || b >= self.slots.len() {
            return Err("Emplacement invalide");
        }
        self.slots.swap(a, b);
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Amount {
    Exact(u32),
    Stack,
    Maximum,
}
/// Une quantité exacte impossible est refusée. Maximum est la seule commande partielle.
pub fn transfer(
    source: &mut Inventory,
    target: &mut Inventory,
    slot: usize,
    amount: Amount,
) -> Result<u32, &'static str> {
    let stack = source
        .slots
        .get(slot)
        .and_then(Option::as_ref)
        .ok_or("Pile vide ou invalide")?;
    let item = stack.item;
    let n = match amount {
        Amount::Exact(n) => n,
        Amount::Stack => stack.quantity,
        Amount::Maximum => stack.quantity.min(target.room(item)),
    };
    if n == 0 || n > stack.quantity {
        return Err("Quantité indisponible : aucun transfert");
    }
    target.add(item, n)?;
    let s = source.slots[slot].as_mut().unwrap();
    s.quantity -= n;
    if s.quantity == 0 {
        source.slots[slot] = None;
    }
    Ok(n)
}
