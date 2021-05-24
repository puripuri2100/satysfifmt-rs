use satysfi_parser::{Cst, CstText, Rule::*};

pub fn to_string(cst: &Cst, input: &CstText) -> String {
  let inner = &cst.inner;
  match inner.get(0).map(|cst| cst.rule) {
    Some(pat_list) => {
      todo!()
    }
    Some(pat_as) => {
      todo!()
    }
    Some(pat_tuple) => {
      todo!()
    }
    Some(var) => input.get_text_from_span(inner[0].span).to_string(),
    Some(const_unit) => "()".to_string(),
    Some(const_bool) => input.get_text_from_span(inner[0].span).to_string(),
    Some(const_int) => input.get_text_from_span(inner[0].span).to_string(),
    Some(const_float) => input.get_text_from_span(inner[0].span).to_string(),
    Some(const_length) => input.get_text_from_span(inner[0].span).to_string(),
    Some(const_string) => todo!(), //input.get_text_from_span(inner[0].span).to_string(),
    None => "_".to_string(),
    _ => unreachable!(),
  }
}
