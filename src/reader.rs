#[cfg(feature = "from_file")]
pub mod reader {

use std::fs;
use serde::{Deserialize, Serialize};
use toml;
use serde_json;

use crate::types::{InvoiceData, ItemRaw, Payment, PersonalInfo, RawInvoice};

/// Allows generating an invoice from a json/toml file.
pub struct InvoiceReader {}
impl ReadInvoice for InvoiceReader {
    fn from_json(file: String) -> Result<Vec<RawInvoice>, Box<dyn std::error::Error>> {
        let parser = |s: &str| serde_json::from_str::<Vec<FileInvoice>>(s).map_err(|e| Box::new(e));
        let invoices: Vec<RawInvoice> = list_raw_invoices(file, parser).unwrap();
        Ok(invoices)
    }
    fn from_toml(file: String) -> Result<Vec<RawInvoice>, Box<dyn std::error::Error>> {
        let parser = |s: &str| toml::from_str::<Vec<FileInvoice>>(s).map_err(|e| Box::new(e));
        let invoices: Vec<RawInvoice> = list_raw_invoices(file, parser).unwrap();
        Ok(invoices)
    }
}

/// Implementing this trait, allows generating an invoice from a json/toml file.
pub trait ReadInvoice {
    /// Read a json file with a slice of Invoices and convert it to raw type 
    fn from_json(file: String) -> Result<Vec<RawInvoice>, Box<dyn std::error::Error>>;
    /// Read a toml file with a slice of Invoices and convert it to raw type 
    fn from_toml(file: String) -> Result<Vec<RawInvoice>, Box<dyn std::error::Error>>;
}

/// writing this was a big waste of time, but it saved 2 lines of code
fn list_raw_invoices <F,E> (file: String, parser_func: F) -> 
    Result<Vec<RawInvoice>, Box<std::io::Error>> 
    where 
        F: FnOnce(&str) -> Result<Vec<FileInvoice>, Box<E>>,
        E: std::error::Error,
{ 
    let file_string: String = match fs::read_to_string(file) {
        Ok(v) => v,
        Err(e) => { return Err(Box::new(e)) },
    };
    let file_raw: Vec<FileInvoice> = parser_func(file_string.as_str()).unwrap();
    Ok(file_raw.into_iter().map(RawInvoice::from).collect())
}


impl From<FileInvoice> for RawInvoice{
    fn from(i: FileInvoice) -> RawInvoice {
        RawInvoice { 
            from: PersonalInfo { 
                email: i.from.email, 
                name: i.from.name, 
                addr_one: i.from.addr_one, 
                addr_two: i.from.addr_two, 
                postal: i.from.postal, 
                state: i.from.state,
                country: i.from.country,
            }, 
            to: PersonalInfo { 
                email: i.to.email, 
                name: i.to.name, 
                addr_one: i.to.addr_one, 
                addr_two: i.to.addr_two, 
                postal: i.to.postal, 
                state: i.to.state, 
                country: i.to.country 
            }, 
            items: i.items.into_iter().map(Into::into).collect(), 
            payment: Payment { 
                wallet_address: i.payment.wallet_address, 
                currency: i.payment.currency, 
                tx: i.payment.tx 
            },
            data: InvoiceData { 
                invoice_number: i.data.invoice_number, 
                due_date: i.data.due_date, 
                issue_date: i.data.issue_date 
            }
        }
    }
}

impl From<FileItemRaw> for ItemRaw {
    fn from(f: FileItemRaw) -> Self {
        ItemRaw {
            description: f.description,
            quantity: f.quantity,
            amount: f.amount,
            tax_percent: f.tax_percent,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FileInvoice {
    from: FilePersonalInfo,
    to: FilePersonalInfo,
    items: Vec<FileItemRaw>,
    payment: FilePayment,
    data: FileInvoiceData, 
}

#[derive(Serialize, Deserialize)]
struct FilePersonalInfo {
    email: String,
    name: String,
    addr_one: String,
    addr_two: String,
    postal: String,
    state: String,
    country: String,
}

#[derive(Serialize, Deserialize)]
struct FileItemRaw {
    description: String,
    quantity: u8,
    amount: u32,
    tax_percent: u8,
}

#[derive(Serialize, Deserialize)]
struct FileInvoiceData {
    invoice_number: u16,
    due_date: String,
    issue_date: String,
}

#[derive(Serialize, Deserialize)]
struct FilePayment {
    wallet_address: String,
    currency: String,
    tx: String,
}

}
