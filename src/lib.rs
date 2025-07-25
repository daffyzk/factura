pub mod invoice;
pub mod types;
pub mod reader;
pub mod simple_invoice;

// use types::ParsedInvoice;
//
// use std::path::PathBuf;
//
// pub struct Factura {
//     output_dir: PathBuf,
// }
//
// impl Factura {
//
//     pub fn new(output_dir: PathBuf) -> Self {
//         Self {
//             output_dir
//         }
//     }
//
//     pub fn make_pdfs(&self, config: Config) -> Result<(), Box<dyn std::error::Error>>{
//         self.process_invoices(&config, |inv, out| {
//             // not ideal to clone fonts since they're pretty big but 
//             // it's better than recreating it on every loop run
//             match Self::pdf(inv, out, fonts.clone()) {
//                 Ok(v)  => {println!("OK: {:?}", v)},
//                 Err(e) => {println!("ERROR: {:?}", e)},
//             }
//         });
//         Ok(())
//     }
//
//     fn process_invoices<F>(&self, invoice: &ParsedInvoice, mut render_func: F)
//         where
//                 F: FnMut(String),
//         {
//             for (_, i) in invoice.iter().enumerate() {
//                 let output_file = format!("invoice_{}_{}",
//                     invoice.data.due_date,
//                     invoice.from.name.trim().to_lowercase().replace(' ', "_")
//                 );
//                 //TODO does this format work? idk??? maybe
//                 let output = format!("{}{}", &self.output_dir.to_string_lossy(), output_file);
//                 render_func(output.to_string());
//             }
//         }
//     }
//
