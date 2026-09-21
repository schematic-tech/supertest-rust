use schematic::supertest;
use text_tools::collapse_spaces;

#[supertest]
pub fn collapsing_spaces_again_changes_nothing(text: String) {
    let once = collapse_spaces(&text);
    let twice = collapse_spaces(&once);

    assert_eq!(twice, once);
}
