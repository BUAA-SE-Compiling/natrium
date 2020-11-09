use std::fmt::Write;

use r0syntax::span::Span;

pub fn pretty_print_error(
    writer: &mut dyn Write,
    input: &str,
    error: &str,
    span: Span,
) -> Result<(), std::fmt::Error> {
    writeln!(writer, "{}", error)?;

    let start = line_span::find_line_range(input, span.idx);
    let end = line_span::find_line_range(input, span.idx + span.len);

    if let Some(line) = line_span::find_prev_line_range(input, span.idx) {
        writeln!(writer, "{}", &input[line])?;
    }
    if start == end {
        writeln!(writer, "{}", &input[start.clone()])?;
        writeln!(
            writer,
            "{:space_width$}{:^^line_width$}",
            "",
            "",
            space_width = span.idx - start.start,
            line_width = span.len
        )?;
    } else {
        let print_range = start.start..end.end;
        let input_range = input[print_range].lines().collect::<Vec<_>>();

        writeln!(writer, "{}", input_range[0])?;
        writeln!(
            writer,
            "{:space_width$}{:^^line_width$}",
            "",
            "",
            space_width = span.idx - start.start,
            line_width = start.end - span.idx
        )?;
        for i in 1..(input_range.len() - 1) {
            writeln!(writer, "{}", input_range[i])?;
            writeln!(
                writer,
                "{:^^len$}",
                "",
                len = input_range[i].chars().count()
            )?;
        }
        writeln!(writer, "{}", input_range[input_range.len() - 1])?;
        writeln!(
            writer,
            "{:^^line_width$}",
            "",
            line_width = span.idx + span.len - end.start
        )?;
    }
    if let Some(line) = line_span::find_next_line_range(input, span.idx + span.len) {
        writeln!(writer, "{}", &input[line])?;
    }
    Ok(())
}
