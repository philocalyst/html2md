use super::StructuredPrinter;
use super::TagHandler;
use crate::common::get_tag_attr;

use markup5ever_rcdom::{Handle, NodeData};

#[derive(Default)]
pub struct CodeHandler {
    code_type: String,
    language: Option<String>,
}

impl CodeHandler {
    /// Used in both starting and finishing handling
    fn do_handle(&mut self, printer: &mut StructuredPrinter, start: bool) {
        // if we're seeing <code> inside a <pre>, skip it
        let immediate_parent = printer.parent_chain.last().unwrap().to_owned();
        if self.code_type == "code" && immediate_parent == "pre" {
            return;
        }

        match self.code_type.as_ref() {
            "pre" => {
                if start {
                    printer.insert_newline();
                    // emit ``` + optional language
                    printer.append_str("\n```");
                    if let Some(ref lang) = self.language {
                        printer.append_str(lang);
                    }
                    printer.insert_newline();
                }

                if !start {
                    printer.append_str("\n```");
                    printer.insert_newline();
                }
            }
            "code" | "samp" => {
                // inline code
                printer.append_str("`");
            }
            _ => {}
        }
    }
}

impl TagHandler for CodeHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.code_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new(),
        };

        // grab data-language if!!! it exists
        self.language = get_tag_attr(tag, "data-language");

        self.do_handle(printer, true);
    }
    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        self.do_handle(printer, false);
    }
}
