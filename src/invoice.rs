use crate::types::{ParsedInvoice, RawInvoice, Item, Total};
use genpdf::fonts::{FontData, FontFamily};
use rust_embed::RustEmbed;

pub trait Invoice { 

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


#[cfg(feature = "html")]
pub trait ExportsHTML {  
    fn to_html(self, file_name: String) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(feature = "pdf")]
pub trait ExportsPDF { 
    fn to_pdf(self, file_name: String) -> Result<(), Box<dyn std::error::Error>>;
    
    /// This method sets the font for PDF generation.
    ///
    /// It's very important to use this, or something similar if you implement your own invoice design
    fn set_pdf_fonts() -> Result<FontFamily<FontData>, Box<dyn std::error::Error>>  {
        
        let font = Liberation::Mono;
        let font_variations = font.variations();        

        let vars: Vec<FontData> = font_variations.into_iter().map(|var| {
            let bytes = Fonts::get(&var).unwrap().data.into_owned();
            genpdf::fonts::FontData::new(bytes, None).unwrap()
        }).collect();
        
        Ok(genpdf::fonts::FontFamily {
            regular: vars.get(0).unwrap().clone(),
            bold: vars.get(1).unwrap().clone(),
            italic: vars.get(2).unwrap().clone(),
            bold_italic: vars.get(3).unwrap().clone(),
        })
    }
}

#[derive(RustEmbed)]
#[folder = "fonts"]
struct Fonts;

enum Liberation {
    Mono,
    Sans,
    Serif,
}

impl Liberation {
    pub fn value(&self) -> &'static str {
        match self {
            Liberation::Mono => "LiberationMono",
            Liberation::Sans => "LiberationSans",
            Liberation::Serif => "LiberationSerif",
        }
    }

    pub fn variations(&self) -> [String;4] { 
        let font_name = self.value();
        [
        format!("{}-Regular.ttf", font_name),
        format!("{}-Bold.ttf", font_name),
        format!("{}-Italic.ttf", font_name),
        format!("{}-BoldItalic.ttf", font_name),
        ]
    }
}

