use factura::{types::{InvoiceData, ItemRaw, Payment, PersonalInfo, RawInvoice}, ExportsPDF, SimpleInvoice};

#[test]
fn test_simple_invoice() {
    let raw = RawInvoice {
        from: PersonalInfo { 
            email: String::from("joe@services.com"), 
            name: String::from("Joe's Services"), 
            addr_one: String::from("Leuteritzweg"), 
            addr_two: String::from("13"), 
            postal: String::from("22399"), 
            state: String::from("Hamburg"), 
            country: String::from("Germany") 
        },
        to: PersonalInfo { 
            email: String::from("billing@corp.com"), 
            name: String::from("Super Corp"), 
            addr_one: String::from("Tegelsbarg"), 
            addr_two: String::from("73"), 
            postal: String::from("22399"), 
            state: String::from("Hamburg"), 
            country: String::from("Germany") 
        },
        items: vec![
            ItemRaw{ 
                description: String::from("Big truck transport fee"), 
                quantity: 2,
                amount: 400, 
                tax_percent: 20,
            },
            ItemRaw{ 
                description: String::from("Big truck transport fee"), 
                quantity: 2,
                amount: 400, 
                tax_percent: 20,
            }
        ],
        payment: Payment{ 
            wallet_address: String::from("0x123123123123123123123123123"), 
            currency: String::from("EURC"), 
            tx: String::from("none"), 
        },
        data: InvoiceData { 
            invoice_number: 376, 
            due_date: String::from("16/jan/2025"), 
            issue_date: String::from("5/jan/2025") 
        }
    };
    let invoice = SimpleInvoice::new(raw);
    let result = invoice.to_pdf(String::from("services_5_jan_2025")).unwrap();
    assert_eq!(result, ());
}
