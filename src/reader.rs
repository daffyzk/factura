use std::fs;
use serde::{Deserialize, Serialize};
use toml;

use crate::types::{InvoiceData, ItemRaw, Payment, PersonalInfo, RawInvoice};

pub trait FromFile {
    fn from_json(file: String) -> Result<RawInvoice, Box<dyn std::error::Error>> {
        let json_str = match fs::read_to_string(file) {
            Ok(v) => v,
            Err(e) => { return Err(Box::new(e)) },
        };
        let file_raw: FileInvoice = serde_json::from_str(&json_str).unwrap();
        Ok(RawInvoice::from(file_raw))
    }
    fn from_toml(file: String) -> Result<RawInvoice, Box<dyn std::error::Error>> {
        let toml_str = match fs::read_to_string(file) {
            Ok(v) => v,
            Err(e) => { return Err(Box::new(e)) },
        };
        let file_raw: FileInvoice = toml::from_str(&toml_str)?;
        Ok(RawInvoice::from(file_raw))
    }
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
    pub from: FilePersonalInfo,
    pub to: FilePersonalInfo,
    pub items: Vec<FileItemRaw>,
    pub payment: FilePayment,
    pub data: FileInvoiceData, 
}

#[derive(Serialize, Deserialize)]
pub struct FilePersonalInfo {
    pub email: String,
    pub name: String,
    pub addr_one: String,
    pub addr_two: String,
    pub postal: String,
    pub state: String,
    pub country: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileItemRaw {
    pub description: String,
    pub quantity: u8,
    pub amount: u32,
    pub tax_percent: u8,
}

#[derive(Serialize, Deserialize)]
pub struct FileInvoiceData {
    pub invoice_number: u16,
    pub due_date: String,
    pub issue_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilePayment {
    pub wallet_address: String,
    pub currency: String,
    pub tx: String,
}

