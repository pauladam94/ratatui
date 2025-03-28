use crate::symbols::line::{BorderSymbol, LineStyle};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MergeStyle {
    Exact,
    // Merge symbols in a visual pleasing way.
    // An example is that " " and " " will be merged to : ""
    BestFit,
}

pub const fn merge_border(
    prev: BorderSymbol,
    next: BorderSymbol,
    style: &MergeStyle,
) -> BorderSymbol {
    use LineStyle::Nothing;
    let mut res = BorderSymbol::new(Nothing, Nothing, Nothing, Nothing);

    res.right = merge_line_style(prev.right, next.right);
    res.up = merge_line_style(prev.up, next.up);
    res.left = merge_line_style(prev.left, next.left);
    res.down = merge_line_style(prev.down, next.down);

    match style {
        MergeStyle::BestFit => {
            // do something clever for complicated characters
            // remove double and thick mix up ->
            // all combination of thick character are available
        }
        _ => {}
    }

    res
}

pub const fn merge_line_style(prev: LineStyle, next: LineStyle) -> LineStyle {
    use LineStyle::{
        Double, DoubleDash, Nothing, Plain, QuadrupleDash, QuadrupleDashThick, Thick, TripleDash,
        TripleDashThick,
    };
    match (prev, next) {
        (Nothing, Nothing) => Nothing,
        (s, Nothing) | (Nothing, s) => s,
        (Thick, Plain | Thick) | (Plain, Thick) => Thick,
        (Double, Plain | Double) | (Plain, Double) => Double,
        (Plain, Plain) => Plain,
        (DoubleDash, DoubleDash) => DoubleDash,
        (TripleDash, TripleDash) => TripleDash,
        (TripleDashThick, TripleDashThick) => TripleDashThick,
        (QuadrupleDash, QuadrupleDash) => QuadrupleDash,
        (QuadrupleDashThick, QuadrupleDashThick) => QuadrupleDashThick,
        (_, next) => next,
    }
}
