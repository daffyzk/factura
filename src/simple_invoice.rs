use crate::invoice::Invoice;
use crate::types::{ParsedInvoice, RawInvoice};
use std::error::Error as stdError;

#[cfg(feature = "html")]
use std::io::Write;
#[cfg(feature = "html")]
use askama::Template;

#[cfg(feature = "pdf")]
use crate::invoice::ExportsPDF;
#[cfg(feature = "pdf")]
use genpdf::{ 
    error::Error as genpdfError,
    Margins, Mm, Alignment, Document, Element, SimplePageDecorator,
    fonts::{FontData, FontFamily},
    elements::{Break, FrameCellDecorator, LinearLayout, Paragraph, TableLayout, TableLayoutRow},
    style::{Style, StyledString},
};

/// Simple invoice implementation, the style is nothing to write home about, but it does it's job. 
/// Could return either HTML or PDF, depending on the enable feature flags in Cargo.toml 
pub struct SimpleInvoice {
    invoice: ParsedInvoice,

    #[cfg(feature = "pdf")]
    font_family: FontFamily<FontData>
}

impl Invoice for SimpleInvoice {}

impl SimpleInvoice {

    pub fn new(raw: RawInvoice) -> Self {
        SimpleInvoice { 
            invoice: Self::parse_raw_invoice(raw), 
            #[cfg(feature = "pdf")]
            font_family: Self::set_pdf_fonts(None).unwrap(),
        }
    }
}



#[cfg(feature = "html")]
impl crate::invoice::ExportsHTML for SimpleInvoice {
    fn to_html(self, file_name: String) -> Result<(), Box<dyn stdError>> {
        let invoice: TemplateableInvoice = TemplateableInvoice::from(self.invoice.clone());
        let rendered = invoice.render()?; 

        let mut file = std::fs::File::create(&format!("{}.html", file_name))?;
        file.write_all(rendered.as_bytes())?;
        Ok(())

    }
}

#[cfg(feature = "pdf")]
impl ExportsPDF for SimpleInvoice {

    fn to_pdf(self, file_name: String) -> Result<(), Box<dyn stdError>> {
 
        let invoice = self.invoice.clone();
        let pad_text = Margins::from((Mm::from(1), Mm::from(0)));
        let pad_box  = Margins::from(Mm::from(2));
        
        let mut doc = Document::new(self.font_family);
        doc.push(Break::new(1.5));
        doc.set_title("invoice");

        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);

        let mut layout = LinearLayout::vertical();

        let mut header_table: TableLayout = TableLayout::new(vec![3, 7]); // 2 cols are 30% and 70%
        let cell = FrameCellDecorator::new(false, false, false);

        header_table.set_cell_decorator(cell);

        // HEADER TEXT (table with no walls)
        let mut row: TableLayoutRow = header_table.row();
        row.push_element(Paragraph::new( bold_styled_string("From") )
            .padded(pad_text)
        );
        row.push_element(Paragraph::new( bold_styled_string("Invoice") )
            .aligned(Alignment::Right).padded(pad_text)
        );
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new(invoice.from.name.clone()));
        row.push_element(Paragraph::new(format!("Invoice #{}", invoice.data.invoice_number))
            .aligned(Alignment::Right)
            );
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new(invoice.from.email.clone()));
        row.push_element(Paragraph::new(format!("Issued on: {}", invoice.data.issue_date))
            .aligned(Alignment::Right)
        );
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new(invoice.from.addr_one.clone()));
        row.push_element(Paragraph::new(format!("Payment due by: {}", invoice.data.due_date))
            .aligned(Alignment::Right)
        );
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new(invoice.from.addr_two.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new(invoice.from.postal.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new(invoice.from.state.clone()));
        row.push_element(Paragraph::new(bold_styled_string("Wallet Address"))
            .aligned(Alignment::Right).padded(pad_text)
        );
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new(invoice.from.country.clone()));
        row.push_element(Paragraph::new(invoice.payment.wallet_address.clone())
            .aligned(Alignment::Right)
        );
        match_row(row.push());

        row = header_table.row();
        row.push_element(Paragraph::new("".to_string()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());
        
        layout.push(header_table);
         
        // BILLING TEXT (table with no walls)
        let mut billed_table = TableLayout::new(vec![1, 1]);
        billed_table.set_cell_decorator(FrameCellDecorator::new(false, false, false));

        row = billed_table.row();
        row.push_element(Paragraph::new( bold_styled_string("Billed to") ).padded(pad_text));
        row.push_element(Paragraph::new(bold_styled_string("Expected payment method"))
            .aligned(Alignment::Right).padded(pad_text)
        );
        match_row(row.push());

        row = billed_table.row();
        row.push_element(Paragraph::new(invoice.to.name.clone()));
        row.push_element(Paragraph::new(invoice.payment.currency.clone()).aligned(Alignment::Right));
        match_row(row.push());

        row = billed_table.row();
        row.push_element(Paragraph::new(invoice.to.email.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        row = billed_table.row();
        row.push_element(Paragraph::new(invoice.to.addr_one.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        row = billed_table.row();
        row.push_element(Paragraph::new(invoice.to.addr_two.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        row = billed_table.row();
        row.push_element(Paragraph::new(invoice.to.postal.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        row = billed_table.row();
        row.push_element(Paragraph::new(invoice.to.state.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        row = billed_table.row();
        row.push_element(Paragraph::new(invoice.to.country.clone()));
        row.push_element(Paragraph::new("".to_string()));
        match_row(row.push());

        layout.push(billed_table);

        layout.push(Break::new(2));

        layout.push(Paragraph::new(bold_styled_string("Transaction")).padded(pad_text));
        layout.push(Paragraph::new(invoice.payment.tx.clone()));
        
        layout.push(Break::new(2));
        
        // BOTTOM TABLE
        let mut table = TableLayout::new(vec![4, 1, 2, 1, 2]);
        table.set_cell_decorator(FrameCellDecorator::new(true, true, true));

        row = table.row();

        row.push_element(Paragraph::new( bold_styled_string("Description") )
            .padded(pad_box)
        );
        row.push_element(Paragraph::new( bold_styled_string("Qty") )
            .aligned(Alignment::Right).padded(pad_box)
        );
        row.push_element(Paragraph::new( bold_styled_string("Unit Price") )
            .aligned(Alignment::Right).padded(pad_box)
        );
        row.push_element(Paragraph::new( bold_styled_string("Tax") )
            .aligned(Alignment::Right).padded(pad_box)
        );
        row.push_element(Paragraph::new( bold_styled_string("Amount") )
            .aligned(Alignment::Right).padded(pad_box)
        );
        match_row(row.push());

        for item in invoice.items {
            row = table.row();
            row.push_element(Paragraph::new(item.description).padded(pad_box));
            row.push_element(Paragraph::new(item.quantity.to_string())
                .aligned(Alignment::Right).padded(pad_box)
            );
            row.push_element(Paragraph::new(format!("{} {}", item.unit_price, invoice.payment.currency))
                .aligned(Alignment::Right).padded(pad_box)
            );
            row.push_element(Paragraph::new(format!("{}%", item.tax_percent))
                .aligned(Alignment::Right).padded(pad_box)
            );
            row.push_element(Paragraph::new(format!("{} {}", item.total, invoice.payment.currency))
                .aligned(Alignment::Right).padded(pad_box)
            );
            match_row(row.push());
        }

        row = table.row();
        row.push_element(Paragraph::new( bold_styled_string("Total without tax") )
            .padded(pad_box)
        );
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new( 
            bold_styled_string(&format!("{} {}", invoice.total.no_tax, invoice.payment.currency)) 
        ).aligned(Alignment::Right).padded(pad_box));
        match_row(row.push());

        row = table.row();
        row.push_element(Paragraph::new( bold_styled_string("Total Tax Amount") ).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new( 
            bold_styled_string(&format!("{} {}", invoice.total.tax_amount, invoice.payment.currency)) 
        ).aligned(Alignment::Right).padded(pad_box));
        match_row(row.push());

        row = table.row();
        row.push_element(Paragraph::new( bold_styled_string("Total Amount") ).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new( 
            bold_styled_string(&format!("{} {}", invoice.total.amount, invoice.payment.currency)) 
        ).aligned(Alignment::Right).padded(pad_box));
        match_row(row.push());

        row = table.row();
        row.push_element(Paragraph::new( bold_styled_string("Amount Due") ).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new("".to_string()).padded(pad_box));
        row.push_element(Paragraph::new( 
            bold_styled_string(&format!("{} {}", invoice.total.amount_due, invoice.payment.currency)) 
        ).aligned(Alignment::Right).padded(pad_box));
        
        match_row(row.push());

        layout.push(table);

        doc.push(layout);
        
        let file_path = format!("{}.pdf", file_name);

        match doc.render_to_file(file_path) {
            Ok(_)  => {
                println!("pdf rendered!");
                Ok(())
            },
            Err(e) => { Err(genpdf_error_convert(e)) },
        }

    }
}

#[cfg(feature = "pdf")]
fn bold_styled_string(text: &str) -> StyledString {
    StyledString {
        s:text.to_string(), 
        style: Style::new().bold().with_line_spacing(0.1)
    }
}

#[cfg(feature = "pdf")]
fn match_row(rowsult: Result<(), genpdfError> ) {
    match rowsult {
        Ok(_r)  => { 
        // println!("LINE PUSHED: {:?}", r)
        },
        Err(e) => { eprintln!("Error pushing row: {:?}", e)},
    }
}

#[cfg(feature = "pdf")]
/// Takes a genpdfError and converts it to Box<dyn stdError>
/// meant as a helper for the pdf implementation
pub fn genpdf_error_convert(e: genpdfError ) -> Box<dyn stdError> {
    Box::new(
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    ) as Box<dyn stdError + 'static>
}


#[cfg(feature = "html")]
#[derive(Template)]
#[template(path = "simple_invoice.html")]
/// Askama Template type, which is meant to be converted from a ParsedInvoice
struct TemplateableInvoice {
    pub from: crate::types::PersonalInfo,
    pub to: crate::types::PersonalInfo,
    pub items: Vec<crate::types::Item>,
    pub total: crate::types::Total,
    pub payment: crate::types::Payment,
    pub data: crate::types::InvoiceData,
}

#[cfg(feature = "html")]
impl From<ParsedInvoice> for TemplateableInvoice {
    fn from(parsed: ParsedInvoice) -> Self {
        TemplateableInvoice {
            from: parsed.from,
            to: parsed.to,
            items: parsed.items,
            total: parsed.total,
            payment: parsed.payment,
            data: parsed.data,
        }
    }
}
