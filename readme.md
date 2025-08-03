# factura 
Generate quick pdf and html invoices.

## SimpleInvoice
Allows generating simple invoices in either html or pdf, requires a parameter of type RawInvoice, that defines the invoice data.
That raw invoice type is then parsed, and the total/tax values are calculated, resulting in a ParsedInvoice.

## FromFile
It can read either a json or toml file, and convert that raw file string into a RawInvoice type.

## extensibility
You can of course, create your own invoices, with knowledge of HTML+Askama or GenPdf.
The SimpleInvoice implementation can be used as an example, and there is a lot to extend from it.
You can add custom colors, set an svg as a watermark for your invoice, reorganize the items in different ways, remove invoice data that you don't need, etc.

## upcoming

planned
- Watermarks on simple invoice, or some other new invoice, perhaps WaterMarkedInvoice
- Other styles of invoices

unplanned
- I would like to make the invoice types more generic, allowing you to use the default provided types, or make your own raw-invoice types.
It's not really a priority, since my use-case is already fulfilled by the prior plans.
