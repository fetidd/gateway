/// Tests that an A (domain type) can be transformed into an E (api response type)
/// and then serialized into a JSON string representation in the expected manner.
pub fn check_serialize_to_response<'a, A, E>(obj: &'a A, exp: &E, exp_json: &str)
where
    E: PartialEq + From<&'a A> + std::fmt::Debug + serde::Serialize,
{
    let act: E = obj.into();
    assert_eq!(act, *exp);
    let act_json = serde_json::to_string_pretty(&act).expect("serialize to json");
    assert_eq!(act_json, exp_json);
}
