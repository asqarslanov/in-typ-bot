use indoc::formatdoc;
use itertools::Itertools;
use teloxide::utils::html;

pub mod inline_query;
pub mod message;

use crate::logic::ErrorDetails;

fn generate_error_text(source_code: &str, errors: &[ErrorDetails], formatting: bool) -> String {
    if formatting {
        let errors_formatted = errors
            .iter()
            .map(|err| {
                let (line, column) = err.coordinates;
                let coordinates_text =
                    html::bold(&format!("Error on Line {line} : Column {column}"));
                let error_text = html::code_inline(&err.message);
                formatdoc! {"
                    {coordinates_text}
                    {error_text}\
                "}
            })
            .join("\n\n");

        let source_code_formatted = html::code_block_with_lang(source_code, "typst");
        formatdoc! {"
            {source_code_formatted}

            {errors_formatted}\
        "}
    } else {
        let error = errors
            .first()
            .expect("typst should output at least one error");

        format!(
            "{}:{}: {}",
            error.coordinates.0, error.coordinates.1, error.message,
        )
    }
}
