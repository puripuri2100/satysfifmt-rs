use anyhow::Result;
use satysfi_parser::Cst;
use satysfi_parser::CstText;
use satysfi_parser::Rule::*;
use satysfi_parser::Span;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Error, Hash)]
pub enum ToStringError {
  #[error("unexpedted rule: {0:?}")]
  UnExpectedRule(satysfi_parser::Rule),
  #[error("not found rule")]
  NotFoundRule,
}

/// コメントとcstを紐づける関数
/// cstのspanの間の範囲のリストを作り、その中に入っているcommentを抜き出す
fn separate_comments(
  input: &CstText,
  all_comments_lst: &Vec<Span>,
  start: usize,
  inner: &Vec<Cst>,
) -> Result<Vec<(Vec<String>, Cst)>> {
  let mut lst = Vec::new();
  let mut temporary_start = start;
  for cst in inner.iter() {
    let Span { start, end } = &cst.span;
    let s = temporary_start;
    let e = start;
    let comments_lst = all_comments_lst
      .iter()
      .filter_map(|span| {
        let Span { start, end } = span;
        if s < *start && end < e {
          Some(input.get_text_from_span(*span).to_string())
        } else {
          None
        }
      })
      .collect::<Vec<_>>();
    lst.push((comments_lst, cst.clone()));
    temporary_start = *end;
  }
  Ok(lst)
}

/// top-levelでのコメント分割
/// 確定で0から始まり、ファイルの一番後ろにあるコメントも返すようにする
fn separate_comments_top_level(
  input: &CstText,
  all_comments_lst: &Vec<Span>,
  inner: &Vec<Cst>,
) -> Result<(Vec<(Vec<String>, Cst)>, Vec<String>)> {
  let mut lst = Vec::new();
  let mut temporary_start = 0;
  for cst in inner.iter() {
    let Span { start, end } = &cst.span;
    let s = temporary_start;
    let e = start;
    let comments_lst = all_comments_lst
      .iter()
      .filter_map(|span| {
        let Span { start, end } = span;
        if s < *start && end < e {
          Some(input.get_text_from_span(*span).to_string())
        } else {
          None
        }
      })
      .collect::<Vec<_>>();
    lst.push((comments_lst, cst.clone()));
    temporary_start = *end;
  }
  // ↑までは共通
  // ここからはファイルの一番後ろのコメントも取得する処理の部分
  let comments_lst = all_comments_lst
    .iter()
    .filter_map(|span| {
      let start = span.start;
      if temporary_start < start {
        Some(input.get_text_from_span(*span).to_string())
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
  Ok((lst, comments_lst))
}

#[test]
fn check_separate_comments() {
  let text = "% hoge
@require: span

let x = x + 1

in

% fuga
% ふが
x
";
  let csttext = CstText::parse(text, satysfi_parser::grammar::program).unwrap();
  let char_indices = text.char_indices();
  let comments_lst = crate::make_comment_lst(&csttext, char_indices);
  let inner = &csttext.cst.inner;
  assert_eq!(
    separate_comments(&csttext, &comments_lst, 0, inner).unwrap(),
    vec![
      (
        vec![" hoge".to_string()],
        Cst::new(
          headers,
          (7, 23),
          vec![Cst::new(
            header_require,
            (7, 22),
            vec![Cst::new(pkgname, (17, 21), vec![])]
          )]
        )
      ),
      (
        vec![],
        Cst::new(
          preamble,
          (23, 36),
          vec![Cst::new(
            let_stmt,
            (23, 36),
            vec![
              Cst::new(pattern, (27, 28), vec![Cst::new(var, (27, 28), vec![])]),
              Cst::new(
                expr,
                (31, 36),
                vec![Cst::new(
                  dyadic_expr,
                  (31, 36),
                  vec![
                    Cst::new(unary, (31, 32), vec![Cst::new(var, (31, 32), vec![]),]),
                    Cst::new(bin_operator, (33, 34), vec![]),
                    Cst::new(unary, (35, 36), vec![Cst::new(const_int, (35, 36), vec![])])
                  ]
                )]
              )
            ]
          )]
        )
      ),
      (
        vec![" fuga".to_string(), " ふが".to_string()],
        Cst::new(
          expr,
          (58, 59),
          vec![Cst::new(
            unary,
            (58, 59),
            vec![Cst::new(var, (58, 59), vec![])]
          )]
        )
      )
    ]
  )
}

fn make_comments(comments: &Vec<String>) -> String {
  if comments.len() == 0 {
    String::new()
  } else {
    format!("\n% {}\n", comments.join("\n% "))
  }
}

pub fn cst_to_string(
  cst: &Cst,
  index: u64,
  input: &CstText,
  all_comments_lst: &Vec<Span>,
) -> Result<String> {
  saty_or_satyh_to_string(cst, index, input, all_comments_lst)
}

fn saty_or_satyh_to_string(
  cst: &Cst,
  index: u64,
  input: &CstText,
  all_comments_lst: &Vec<Span>,
) -> Result<String> {
  let rule = &cst.rule;
  let inner = &cst.inner;
  let (inner_with_comments, last_comments) =
    separate_comments_top_level(input, all_comments_lst, inner)?;
  let str = match rule {
    program_saty => inner_with_comments
      .iter()
      .map(|(comments, cst)| {
        programs_to_string(cst, comments, index + 1, input, &all_comments_lst, true)
      })
      .collect::<Result<String>>(),
    program_satyh => inner_with_comments
      .iter()
      .map(|(comments, cst)| {
        programs_to_string(cst, comments, index + 1, input, &all_comments_lst, false)
      })
      .collect::<Result<String>>(),
    _ => Err(ToStringError::UnExpectedRule(*rule))?,
  }?;
  Ok(format!(
    "{}{last_comments}",
    str,
    last_comments = make_comments(&last_comments)
  ))
}

fn programs_to_string(
  cst: &Cst,
  comments: &Vec<String>,
  index: u64,
  input: &CstText,
  all_comments_lst: &Vec<Span>,
  is_saty: bool,
) -> Result<String> {
  let comments_str = make_comments(comments);
  let rule = &cst.rule;
  let span = &cst.span;
  let inner = &cst.inner;
  let main_str = match rule {
    stage => stage_to_string(span, input),
    headers => headers_to_string(inner, input, all_comments_lst)?,
    preamble => {
      if is_saty {
        format!("\npreamble\nin\n")
      } else {
        format!("\npreamble\n")
      }
    }
    expr => {
      format!("\nexpr\n")
    }
    _ => Err(ToStringError::UnExpectedRule(*rule))?,
  };
  Ok(format!(
    "{comments}{main}",
    comments = comments_str,
    main = main_str
  ))
}

fn stage_to_string(span: &Span, input: &CstText) -> String {
  let stage_str = input.get_text_from_span(*span);
  format!("@stage: {}\n", stage_str)
}

// TODO:引数の順序を揃える
fn headers_to_string(
  header_cst_list: &Vec<Cst>,
  input: &CstText,
  all_comments_lst: &Vec<Span>,
) -> Result<String> {
  let mut str = String::new();
  let start = header_cst_list
    .get(0)
    .map(|cst| cst.span.end)
    .unwrap_or_else(|| 0);
  let inner_with_comments = separate_comments(input, all_comments_lst, start, header_cst_list)?;
  for (comments, cst) in inner_with_comments.iter() {
    let rule = &cst.rule;
    let inner = &cst.inner;
    match rule {
      header_require => {
        let pkgname_cst = &inner[0];
        let pkgname_rule = pkgname_cst.rule;
        let pkgname_span = pkgname_cst.span;
        match pkgname_rule {
          pkgname => {
            let pkgname_str = input.get_text_from_span(pkgname_span);
            let require_string = format!("{}@require: {}\n", make_comments(comments), pkgname_str);
            str.push_str(&require_string)
          }
          _ => {
            unreachable!()
          }
        }
      }
      header_import => {
        let pkgname_cst = &inner[0];
        let pkgname_rule = pkgname_cst.rule;
        let pkgname_span = pkgname_cst.span;
        match pkgname_rule {
          pkgname => {
            let pkgname_str = input.get_text_from_span(pkgname_span);
            let require_string = format!("{}@import: {}\n", make_comments(comments), pkgname_str);
            str.push_str(&require_string)
          }
          _ => {
            unreachable!()
          }
        }
      }
      _ => {
        unreachable!()
      }
    }
  }
  Ok(str)
}
