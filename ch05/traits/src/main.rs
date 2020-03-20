struct MasterCard {
    number: u8,
    verification: u8,
}
impl CreditCharge for MasterCard {
    fn charge_with_id(&self, id: u32) -> bool {
        unimplemented!()
    }
}

struct Visa {
    number: u32,
}
impl CreditCharge for Visa {
    fn charge_with_id(&self, id: u32) -> bool {
        unimplemented!()
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

fn main() {
    let card = BitCredit { btcnumber: 1024 };
    let code = 4096;
    if card.charge_with_id(code) {
        println!("Success!");
    } else {
        println!("Failure.");
    }
}
