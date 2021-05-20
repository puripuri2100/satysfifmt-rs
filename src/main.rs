use anyhow::{Error, Result};
use satysfi_parser::grammar;
use satysfi_parser::CstText;
use satysfi_parser::LineCol;

use satysfifmt::{cst2str, comments};

fn main() -> Result<()> {
  let satysfi_text = r#"
    @require: stdjabook
    % abc
      @require: code
      %ほげ
   @require:       itemize
   @require: tabular
 @require: math
 @import: test2

 let x = x + 1

 in

 document (|
   title = {test1};
   author = {__};
   show-title = true;
   show-toc = false;
 |) '<
   +section{テスト1}<
     +p{テスト☞}
     +pn{hoge}
   >
 >

 % ファイルの一番後ろにあるコメント
    "#;
  let csttext = match CstText::parse(satysfi_text, grammar::program) {
    Ok(csttext) => csttext,
    Err(e) => return make_error_message_for_csttext(e),
  };
  let char_indices = satysfi_text.char_indices();
  let comment_lst = comments::make_comment_lst(&csttext, char_indices);
  let cst = &csttext.cst;
  let asttext = &csttext.pritty_cst_recursive(cst);
  println!("{}", asttext);
  let fixtext = cst2str::cst_to_string(cst, 0, &csttext, &comment_lst)?;
  println!("{}", fixtext);
  Ok(())
}

fn make_error_message_for_csttext(e: (LineCol, Vec<&str>)) -> Result<(), anyhow::Error> {
  let LineCol { line, column } = e.0;
  let expected = e.1;
  Err(Error::msg(format!(
    "parse err:\n       line: {line}, column: {column}\n       expected of [{expected}]",
    line = line,
    column = column,
    expected = expected.join(" | ")
  )))
}
