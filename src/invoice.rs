use crate::types::{ParsedInvoice, RawInvoice, Item, Total};
use genpdf::fonts::{self, FontData, FontFamily};

pub trait Parseable { 

    /// Take Raw invoice data and turn his data into ParsedInvoice
    fn parse_raw_invoice(data: RawInvoice) -> ParsedInvoice {  
        let mut items = vec![];

        let mut total = Total {
            no_tax: 0,
            tax_amount: 0,
            amount: 0,
            amount_due: 0,
        };

        // add to items 
        for (_, item) in data.items.iter().enumerate() { 
            let tax_value = (item.amount as u32 * item.tax_percent as u32 / 100) as u32;
            let item_total = (item.amount + tax_value) * item.quantity as u32;

            let new_item = Item {
                description: item.description.clone(),
                quantity: item.quantity,
                unit_price: item.amount,
                tax_percent: item.tax_percent,
                total: item_total,
            };

            total.no_tax = total.no_tax + (new_item.unit_price * new_item.quantity as u32);
            total.tax_amount = total.tax_amount + (tax_value * new_item.quantity as u32);
            total.amount = total.amount + item_total;
            total.amount_due = total.amount_due + item_total;

            items.push(new_item);
        }
        
        let mut payment = data.payment.clone();
        payment.tx = format!("\"{}\"", payment.tx).to_string();

        ParsedInvoice {
            from: data.from.clone(),
            to: data.to,
            items,
            total,
            payment: data.payment.clone(),
            data: data.data.clone()
        }
    }
} 

pub trait NewInvoice {
    /// Return new instance of self, with parsed invoice
    fn new(raw: RawInvoice) -> Self where Self: Sized + Parseable {
        Self::from_parsed(Self::parse_raw_invoice(raw))
    }
    
    /// Return a new instance of self
    /// This needs to be implemented by the Invoice struct you made
    fn from_parsed(parsed: ParsedInvoice) -> Self where Self: Sized; 
}

pub trait ExportsHTML {  
    fn to_html(self, file_name: String) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait ExportsPDF { 
    fn to_pdf(self, file_name: String) -> Result<(), Box<dyn std::error::Error>>;
    
    /// This method sets the font for PDF generation.
    ///
    /// It's very important to use this, or something similar if you implement your own invoice design
    fn set_pdf_fonts(font_path: Option<String>) -> Result<FontFamily<FontData>, Box<dyn std::error::Error>>  {
        let prj_fonts: String;
        match font_path {
            Some(p) => prj_fonts = p,
            None => prj_fonts = "fonts/".to_string(),
        }
        let font_list: [(&str, &str); 3] = [
            ("LiberationSans", &prj_fonts),
            ("LiberationSerif", &prj_fonts),
            ("LiberationMono", &prj_fonts)
        ];
        
        let (font_name, font_dir) = font_list[2];

        match fonts::from_files(font_dir, font_name, None) {
            Ok(v)  => { println!("font fonted"); return Ok(v) },
            Err(e) => { 
                return Err(
                    Box::new( 
                        std::io::Error::new(
                            std::io::ErrorKind::Other, 
                            e.to_string()
                        )
                    ) as Box<dyn std::error::Error + 'static>); 
            },
        }
    }
}
