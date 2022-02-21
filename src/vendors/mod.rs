pub enum VendorLevel{
    Shop,
    Market,
    Bank,
    NationalBank
}

pub struct Vendor{
    pub sells: u128,
    pub sector: VendorLevel,
}

impl Vendor{
    pub fn new(sector: VendorLevel) -> Vendor{
        let sells = match sector {
            VendorLevel::Shop => 1,
            VendorLevel::Market => 100,
            VendorLevel::Bank => 100_000,
            VendorLevel::NationalBank => 100_000_000,
        };

        Vendor { sells, sector }
    }
}