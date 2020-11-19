use r0syntax::span::Span;
use std::io::Write;
use unicode_width::UnicodeWidthStr;

pub fn pretty_print_error(
    writer: &mut dyn Write,
    input: &str,
    error: &str,
    span: Span,
) -> Result<(), std::io::Error> {
    writeln!(writer, "{}", error)?;

    if span == Span::eof() {
        let line = input.lines().last().unwrap_or("");
        writeln!(writer, "{}", line)?;
        writeln!(writer, "{:space_width$}^", space_width = line.width())?;

        Ok(())
    } else {
        let start = line_span::find_line_range(input, span.start());
        let end = line_span::find_line_range(input, span.end());

        if let Some(line) = line_span::find_prev_line_range(input, span.start()) {
            writeln!(writer, "{}", &input[line])?;
        }
        if start == end {
            writeln!(writer, "{}", &input[start.clone()])?;
            writeln!(
                writer,
                "{:space_width$}{:^^line_width$}",
                "",
                "",
                space_width = input[start.start..span.start()].width(),
                line_width = input[span.start()..span.end()].width()
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
                space_width = input[start.start..span.start()].width(),
                line_width = input[span.start()..start.end].width()
            )?;
            for i in 1..(input_range.len() - 1) {
                writeln!(writer, "{}", input_range[i])?;
                writeln!(writer, "{:^^len$}", "", len = input_range[i].width())?;
            }
            writeln!(writer, "{}", input_range[input_range.len() - 1])?;
            writeln!(
                writer,
                "{:^^line_width$}",
                "",
                line_width = input[end.start..(span.end())].width()
            )?;
        }
        if let Some(line) = line_span::find_next_line_range(input, span.end()) {
            writeln!(writer, "{}", &input[line])?;
        }
        Ok(())
    }
}
