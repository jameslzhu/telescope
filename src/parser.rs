use combine::*;
use combine::char::*;

use atom::{Symbol, Atom, List, Node, Expr};
use error::*;

fn atom<I>(input: I) -> ParseResult<Atom, I>
    where I: Stream<Item=char>
{
    let num = optional(char('-'))
        .and(many1(digit()))
        .map(|(sign, digits): (Option<char>, Vec<char>)| {
            let magnitude = digits.into_iter()
                .collect::<String>()
                .parse::<i32>().unwrap();
            if sign == Some('-') { -magnitude } else { magnitude }
        })
        .map(Atom::from);

    let sym = satisfy(|c| "+-*/%".chars().any(|x| x == c)) 
        .map(|sym| match sym {
            '+' => Symbol::Add,
            '-' => Symbol::Sub,
            '*' => Symbol::Mul,
            '/' => Symbol::Div,
            '%' => Symbol::Mod,
            _ => unreachable!(),
        })
        .or(choice([string("head"), string("tail")])
            .map(|sym| match sym {
                "head" => Symbol::Head,
                "tail" => Symbol::Tail,
                _ => unreachable!(),
            }))
        .map(Atom::from);

    num.or(sym).parse_stream(input)
}

fn list<I>(input: I) -> ParseResult<List, I>
    where I: Stream<Item=char>
{
    let lex_char = |c| char(c).skip(spaces());
    let space_list = sep_by(parser(node), spaces());

    between(lex_char('['), lex_char(']'), space_list)
        .map(|list: Vec<Node>| List::from(list.as_slice()))
        .parse_stream(input)
}

fn expr<I>(input: I) -> ParseResult<Expr, I>
    where I: Stream<Item=char>
{
    let lex_char = |c| char(c).skip(spaces());
    let space_list = sep_by(parser(node), spaces());

    between(lex_char('('), lex_char(')'), space_list)
        .map(|list: Vec<Node>| Expr::new(list))
        .parse_stream(input)
}

fn node<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item=char>
{
    parser(atom).map(Node::from)
        .or(parser(list).map(Node::from))
        .or(parser(expr).map(Node::from))
        .skip(spaces())
        .parse_stream(input)
}

fn lang<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item=char>
{
    spaces().with(parser(node))
        .parse_stream(input)
}

pub fn parse<'a, L>(line: L) -> Result<Node>
    where L: Into<String>,
{
    parser(lang)
        .parse(State::new(line.into().as_str()))
        .map_err(|e| e.into())
        .and_then(|(node, _)| node.eval())
}

#[cfg(test)]
mod tests {
    use atom::*;
    quickcheck!{
        fn parse_num(x: i32) -> bool {
            use parser::parse;
            match parse(x.to_string()) {
                Ok(node) => node == Node::from(x),
                Err(_) => false,
            }
        }

        fn parse_list(xs: Vec<i32>) -> bool  {
            use parser::parse;
            use atom::*;

            // Build parse string
            let mut strings: Vec<String> = xs.iter().map(ToString::to_string).collect();
            strings.insert(0, String::from("["));
            strings.push(String::from("]"));
            let parse_string = strings.join(" ");

            // Build list of nodes
            let list = List::from(&*xs.iter().cloned().map(Node::from).collect::<Vec<_>>());

            // Parse and compare
            match parse(parse_string) {
                Ok(node) => node == Node::from(list),
                Err(_) => false,
            }
        }

        fn parse_expr(xs: Vec<i32>) -> bool  {
            use parser::parse;
            use atom::*;

            // Build parse string
            let mut strings: Vec<String> = xs.iter().map(ToString::to_string).collect();
            strings.insert(0, String::from("("));
            strings.push(String::from(")"));
            let parse_string = strings.join(" ");

            // Build list of nodes
            let expr = Expr::new(xs.iter().cloned().map(Node::from).collect::<Vec<_>>());

            // Parse and compare
            match parse(parse_string) {
                Ok(node) => node == Node::from(expr),
                Err(_) => false,
            }
        }
    }
}