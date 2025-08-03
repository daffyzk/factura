

#[derive(Clone)]
/// Invoice values that have gone through refinement.
pub struct ParsedInvoice {
    pub from: PersonalInfo,
    pub to: PersonalInfo,
    pub items: Vec<Item>,
    pub total: Total,
    pub payment: Payment,
    pub data: InvoiceData,
}

#[derive(Clone)]
/// Could be either sender or reciever of the invoice.
pub struct PersonalInfo {
    pub email: String,
    pub name: String,
    pub addr_one: String,
    pub addr_two: String,
    pub postal: String,
    pub state: String,
    pub country: String,
}

#[derive(Clone)]
/// Information relevant to the invoice itself.
pub struct InvoiceData {
    pub invoice_number: u16,
    pub due_date: String,
    pub issue_date: String,
}

#[derive(Clone)]
/// Payment data for the goods/services outlined in the invoice.
pub struct Payment {
    pub wallet_address: String,
    pub currency: String,
    pub tx: String,
}

#[derive(Clone)]
/// Good or Service, with total price calculated.
pub struct Item {
    pub description: String,
    pub quantity: u8,
    pub unit_price: u32,
    pub tax_percent: u8,
    pub total: u32,
}

#[derive(Clone)]
/// Calculated values of all items in the raw invoice.
pub struct Total {
    pub no_tax: u32,
    pub tax_amount: u32,
    pub amount: u32,
    pub amount_due: u32,
}

/// Good or Service, before price calculation.
pub struct ItemRaw {
    pub description: String,
    pub quantity: u8,
    pub amount: u32,
    pub tax_percent: u8,
}

/// Invoice data with no price calculations.
pub struct RawInvoice {
    pub from: PersonalInfo,
    pub to: PersonalInfo,
    pub items: Vec<ItemRaw>,
    pub payment: Payment,
    pub data: InvoiceData, 
}
