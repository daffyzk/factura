
#[derive(Clone)]
pub struct ParsedInvoice {
    pub from: PersonalInfo,
    pub to: PersonalInfo,
    pub items: Vec<Item>,
    pub total: Total,
    pub payment: Payment,
    pub data: InvoiceData,
}

#[derive(Clone)]
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
pub struct InvoiceData {
    pub invoice_number: u16,
    pub due_date: String,
    pub issue_date: String,
}

#[derive(Clone)]
pub struct Payment {
    pub wallet_address: String,
    pub currency: String,
    pub tx: String,
}

#[derive(Clone)]
pub struct Item {
    pub description: String,
    pub quantity: u8,
    pub unit_price: u32,
    pub tax_percent: u8,
    pub total: u32,
}

#[derive(Clone)]
pub struct Total {
    pub no_tax: u32,
    pub tax_amount: u32,
    pub amount: u32,
    pub amount_due: u32,
}

pub struct ItemRaw {
    pub description: String,
    pub quantity: u8,
    pub amount: u32,
    pub tax_percent: u8,
}

pub struct RawInvoice {
    pub from: PersonalInfo,
    pub to: PersonalInfo,
    pub items: Vec<ItemRaw>,
    pub payment: Payment,
    pub data: InvoiceData, 
}
