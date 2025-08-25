use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    let graphemes = input.graphemes(true).rev();

    for g in graphemes {
        reversed.push_str(g);
    }

    reversed
}
