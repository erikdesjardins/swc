use crate::pass::Pass;
use ast::*;
use std::sync::Arc;
use swc_common::{FileName, Fold, SourceMap, DUMMY_SP};

#[cfg(test)]
mod tests;

/// `@babel/plugin-transform-react-jsx-source`
pub fn jsx_src(dev: bool, cm: Arc<SourceMap>) -> impl Pass {
    JsxSrc { cm, dev }
}

struct JsxSrc {
    cm: Arc<SourceMap>,
    dev: bool,
}

impl Fold<JSXOpeningElement> for JsxSrc {
    fn fold(&mut self, mut e: JSXOpeningElement) -> JSXOpeningElement {
        if !self.dev || e.span == DUMMY_SP {
            return e;
        }

        let file_lines = match self.cm.span_to_lines(e.span) {
            Ok(v) => v,
            _ => return e,
        };

        e.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: DUMMY_SP,
            name: JSXAttrName::Ident(quote_ident!("__source")),
            value: Some(
                box ObjectLit {
                    span: DUMMY_SP,
                    props: vec![
                        PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(quote_ident!("fileName")),
                            value: box Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                value: match file_lines.file.name {
                                    FileName::Real(ref p) => p.display().to_string().into(),
                                    _ => unimplemented!("file name for other than real files"),
                                },
                                has_escape: false,
                            })),
                        })),
                        PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(quote_ident!("lineNumber")),
                            value: box Expr::Lit(Lit::Num(Number {
                                span: DUMMY_SP,
                                value: (file_lines.lines[0].line_index + 1) as _,
                            })),
                        })),
                    ],
                }
                .into(),
            ),
        }));

        e
    }
}
