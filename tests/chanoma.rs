use chanoma::{Chanoma, Correspondence, Item, Synthesized, TableBuilder};

#[test]
fn can_not_normalized() {
    let corr = Correspondence::new(Synthesized::new(vec![Item::new("z", "Z")]));
    let table_builder = TableBuilder::new().with_corr(&corr);
    let table = table_builder.build();
    let chanoma = Chanoma::from_table(table.clone());
    assert_eq!(chanoma.normalize("abc"), "abc");
}
