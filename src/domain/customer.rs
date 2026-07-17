/// A named regular, promoted out of the anonymous「お客様」so that rumors
/// (第5節) can be attributed to a specific speaker. Deliberately minimal for
/// now (a name is all `rumors::CATALOG` needs to attribute and, in turn,
/// discredit a rumor) -- register/tone-per-customer is a future extension,
/// not modeled here yet.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CustomerId(pub u8);

pub struct Customer {
    pub name: &'static str,
}

pub const CUSTOMERS: &[Customer] = &[
    Customer { name: "サユリ" },
    Customer { name: "カズ" },
    Customer { name: "ミチル" },
    Customer { name: "トウジ" },
    Customer { name: "アリス" },
];

impl CustomerId {
    pub fn name(self) -> &'static str {
        CUSTOMERS[self.0 as usize].name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_customer_id_used_by_the_catalog_resolves_to_a_name() {
        for i in 0..CUSTOMERS.len() as u8 {
            assert!(!CustomerId(i).name().is_empty());
        }
    }
}
