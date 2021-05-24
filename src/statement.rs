use crate::comments;
use crate::pattern;
use anyhow::Result;
use satysfi_parser::{Cst, CstText, Rule::*, Span};

pub fn let_stmt_to_string(
  cst_lst: &[Cst],
  input: &CstText,
  all_comments_lst: &Vec<Span>,
  index: u64,
) -> Result<String> {
  let pat_cst = &cst_lst[0];
  let arg_cst_lst = &cst_lst[1..cst_lst.len() - 1];
  let arg_str = arg_cst_lst
    .iter()
    .map(|cst| arg_to_string(cst, input))
    .collect::<Vec<String>>()
    .join(" ");
  let expr_cst = &cst_lst[cst_lst.len() - 1];
  let pat_str = pattern::to_string(pat_cst, input);
  Ok(format!("\nlet {} {} = \"expr\"", pat_str, arg_str))
}

fn arg_to_string(cst: &Cst, input: &CstText) -> String {
  println!("{:?}", cst.rule);
  match cst.rule {
    // arg以外にもtype_exprの可能性もあるけど、一旦無視
    arg => match cst.inner[0].rule {
      pattern => pattern::to_string(&cst.inner[0], input),
      var_ptn => todo!(),
      _ => unreachable!(),
    },
    type_expr => todo!(),
    _ => unreachable!(),
  }
}
