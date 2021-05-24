use satysfi_parser::CstText;
use satysfi_parser::Span;

pub fn make_comment_lst(csttext: &CstText, char_indices: std::str::CharIndices) -> Vec<Span> {
  let mut comment_lst = vec![];
  let mut is_in_comment_mode = false;
  let mut start = 0;
  for (i, _char) in char_indices {
    let is_comment_char = csttext.is_comment(i);
    if is_comment_char {
      if !is_in_comment_mode {
        // not comment mode -> comment mode
        start = i
      }
      is_in_comment_mode = true
    } else {
      if is_in_comment_mode {
        // comment mode -> not comment mode
        let span = Span { start, end: i - 1 };
        comment_lst.push(span);
      }
      is_in_comment_mode = false
    }
  }
  comment_lst
}

pub fn make_comments(comments: &Vec<String>) -> String {
  if comments.len() == 0 {
    String::new()
  } else {
    format!("\n% {}\n", comments.join("\n% "))
  }
}
