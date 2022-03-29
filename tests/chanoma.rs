use chanoma::{Chanoma, Correspondence, Item, Synthesized, TableBuilder};

#[test]
fn can_not_normalized() {
    let corr = Correspondence::new(Synthesized::new(vec![Item::new("z", "Z")]));
    let mut table_builder = TableBuilder::new();
    table_builder.add_corr(&corr);
    let table = table_builder.build();
    let chanoma = Chanoma::from_table(table);
    assert_eq!(chanoma.normalize("abc"), "abc");
}
