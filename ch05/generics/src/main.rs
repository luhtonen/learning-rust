struct MasterCard {
    number: u8,
    verification: u8,
}
impl CreditCharge for MasterCard {
    fn charge_with_id(&self, id: u32) -> bool {
        self.number + self.verification % (id % 256) as u8 == 0
    }
}

struct Visa {
    number: u32,
}
impl CreditCharge for Visa {
    fn charge_with_id(&self, id: u32) -> bool {
        id != 1026
    }
}

struct WesternUnion {
    name: String,
    verification: u8,
}
impl CreditCharge for WesternUnion {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 3 == (self.verification % 4) as u32
    }
}

struct BitCredit {
    btcnumber: u32,
}
impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btcnumber % 2
    }
}

trait CreditCharge {
    fn charge_with_id(&self, id: u32) -> bool;
}

fn transact<Q: CreditCharge>(card: Q) {
    // get code from user
    let id = 4096;
    if card.charge_with_id(id) {
        println!("Success!");
    } else {
        panic!("Invalid code!");
    }
}

fn main() {
    let card = BitCredit { btcnumber: 1024 };
    transact(card);

    let card2 = Visa { number: 1024 };
    transact(card2);
}
