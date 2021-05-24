use satysfi_parser::Span;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AbstractTreeKind {
  UnitConstant,
  KeyWord(String),
  Value(String),
  List(Vec<AbstractTree>),
  Record(Vec<(String, AbstractTree)>),
  Tuple(Vec<AbstractTree>),
  IfThenElse(Box<AbstractTree>, Box<AbstractTree>, Box<AbstractTree>),
  Lambda(Vec<String>, Box<AbstractTree>),
  LetNonRec(String, Vec<String>, Box<AbstractTree>),
  LetRec(String, Vec<String>, Box<AbstractTree>),
  Itemize(Itemize),
  //Math(Math),
  LetMutable(String, Box<AbstractTree>),
  Overwrite(String, Box<AbstractTree>),
  Open(String),
  OpenIn(String, Box<AbstractTree>),
  ModuleSig(String, Vec<Sigunature>, Vec<AbstractTree>),
  Module(String, Vec<AbstractTree>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AbstractTree {
  comments: Vec<String>,
  kind: AbstractTreeKind,
  span: Span,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SigunatureKind {
  SigType(String, Vec<TypeArgument>),
  SigValue(String, ManualType, Constraints),
  SigDirect(String, ManualType, Constraints),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sigunature {
  comments: Vec<String>,
  kind: SigunatureKind,
  span: Span,
}
