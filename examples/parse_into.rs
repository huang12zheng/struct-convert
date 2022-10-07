use struct_convert::Convert;

#[derive(Debug, Default, PartialEq)]
struct BInner {
    name: String,
}

#[derive(Debug, Default, Convert, PartialEq)]
#[convert(into = "BInner")]
struct B {
    #[convert_field(ignore)]
    bid: i64,

    #[convert_field(parse)]
    name: String,
}
struct BName(String);
impl BName {
    pub fn parse(s: String) -> BName {
        if s.eq("B want to be Inner") {
            Self("BInnerProperty".into())
        } else {
            panic!("Invalid")
        }
    }
}
// #[convert_field(custom= "UserEmail::parse(self.email)")]
// email: String
// ===>>>>
// {
//
// email: UserEmail::parse(self.email),
// }
fn main() {
    let b = B {
        bid: 2,
        name: String::from("B want to be Inner"),
    };
    let inner: BInner = b.into();
    debug_assert_eq!(
        BInner {
            name: String::from("BInnerProperty")
        },
        inner
    );
}
