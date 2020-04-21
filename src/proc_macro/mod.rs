extern crate proc_macro;
use ::proc_macro::{*, TokenTree as TT};
use ::core::iter;

macro_rules! matches {(
    $expr:expr,
    $(|)? $($pat:pat)|+
        $(if $guard:expr)?
    $(,)?
) => (match $expr {
    $(| $pat)+
        $(if $guard)?
    => true,
    | _
    => false,
})}

#[proc_macro] pub
fn item (input: TokenStream)
  -> TokenStream
{
    match try_map(input) {
        | Ok(it) => it,
        | Err((ref err_msg, span)) => compile_error(err_msg, span),
    }
}

fn compile_error(err_msg: &'_ str, span: Span)
  -> TokenStream
{
    macro_rules! spanned {($expr:expr) => ({
        let mut it = $expr;
        it.set_span(span);
        it
    })}
    Vec::<TokenTree>::into_iter(vec![
        TT::Ident(Ident::new("compile_error", span)),
        TT::Punct(spanned!(Punct::new('!', Spacing::Alone))),
        TT::Group(spanned!(Group::new(
            Delimiter::Brace,
            iter::once(TT::Literal(
                spanned!(Literal::string(err_msg))
            )).collect(),
        ))),
    ])
    .collect()
}

fn try_map (input: TokenStream)
  -> Result<TokenStream, (&'static str, Span)>
{
    input.into_iter().map(|mut tt| Ok({
        if let TT::Group(ref group) = tt {
            let span = tt.span();
            tt = TT::Group(Group::new(
                group.delimiter(),
                try_map(group.stream())?
            ));
            tt.set_span(span);
        }
        match tt {
            | TT::Group(ref group)
                if true
                && group.delimiter() == Delimiter::Bracket
                && matches!(
                    group.stream().into_iter().next(),
                    Some(TT::Punct(p))
                        if p.as_char() == '<'
                    ,
                )
            => {
                let mut tokens: Box<dyn Iterator<Item = TokenTree>> =
                    Box::new(group.stream().into_iter())
                ;
                let _ = tokens.next();
                let ref mut ident = String::new();
                loop { match tokens.next() {
                    | None
                    => {
                        return Err((
                            "Missing `>` before the closing `]`",
                            tt.span(),
                        ));
                    },
                    | Some(TT::Punct(p))
                        if p.as_char() == '>'
                    => if let Some(unexpected_tt) = tokens.next() {
                        return Err((
                            "Unexpected trailing token after the terminating `>`",
                            unexpected_tt.span(),
                        ));
                    } else {
                        break;
                    },
                    | Some(TT::Literal(lit))
                    => {
                        let ref s = lit.to_string();
                        if s.chars().all(|c| matches!(c,
                            'a' ..= 'z' |
                            'A' ..= 'Z' |
                            '0' ..= '9' |
                            '_'
                        ))
                        {
                            ident.push_str(s);
                        } else {
                            return Err((
                                "Cannot be converted into an identifier",
                                lit.span(),
                            ));
                        }
                    },
                    | Some(TT::Ident(it))
                    => {
                        ident.push_str(&it.to_string());
                    },
                    | Some(TT::Group(it))
                        if it.delimiter() == Delimiter::None
                    => {
                        tokens = Box::new(it.stream().into_iter().chain(tokens));
                        continue;
                    },
                    | Some(it @ TT::Group(_))
                    => {
                        return Err((
                            "Unexpected group",
                            it.span(),
                        ));
                    },
                    | Some(it @ TT::Punct(_))
                    => {
                        return Err((
                            "Unexpected punct",
                            it.span(),
                        ));
                    },
                }}
                return Ok(TT::Ident(Ident::new(ident, tt.span())));
            },
            | _ => {},
        }
        tt
    })).collect()
}

/** Not part of the public API **/ #[doc(hidden)]
#[proc_macro_derive(__expr_hack__)] pub
fn __expr_hack__ (input: TokenStream)
  -> TokenStream
{
    // enum
    // EnumName
    // {
    //     VariantName
    //     =
    //     (
    //         stringify
    //         !
    //         (
    //             <input>
    //         )
    // , 0).1,}

    let mut tokens = input.into_iter();
    // `enum EnumName`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `{ <tokens> }`
    let mut tokens = if let Some(TT::Group(it)) = tokens.next() { it } else {
        panic!()
    }.stream().into_iter();
    // `VariantName =`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `( <tokens> )`
    let mut tokens = if let Some(TT::Group(it)) = tokens.next() { it } else {
        panic!()
    }.stream().into_iter();
    // `stringify !`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `( <input> )`
    let input = if let Some(TT::Group(it)) = tokens.next() { it } else {
        panic!()
    }.stream();
    let ret = match try_map(input) {
        | Ok(it) => it,
        | Err((ref err_msg, span)) => return compile_error(err_msg, span),
    };
    let span = Span::call_site();
    Vec::<TokenTree>::into_iter(vec![
        TT::Ident(Ident::new("macro_rules", span)),
        TT::Punct(Punct::new('!', Spacing::Alone)),
        TT::Ident(Ident::new("__mini_paste__Hack__", span)),
        TT::Group(Group::new(
            Delimiter::Brace,
            Vec::<TokenTree>::into_iter(vec![
                TT::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
                TT::Punct(Punct::new('=', Spacing::Joint)),
                TT::Punct(Punct::new('>', Spacing::Alone)),
                TT::Group(Group::new(
                    Delimiter::Parenthesis,
                    ret,
                )),
            ]).collect(),
        )),
    ])
    .collect()
}
