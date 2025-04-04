// <one line to give the program's name and a brief idea of what it does.>
// Copyright (C) 2025 Dustin Thomas <stdio@cptlobster.dev>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//
use chumsky::prelude::*;
use subenum::subenum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[subenum(Character)]
pub enum Regexp {
    #[subenum(Character)]
    Any,
    Either(Regexp, Regexp),
    #[subenum(Character)]
    Not(Character),
    #[subenum(Character)]
    Whitespace,
    #[subenum(Character)]
    Digit,
    #[subenum(Character)]
    Word,
    Group(Vec<Regexp>),
    Count{ regexp: Regexp, min: Option<usize>, max: Option<usize> },
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Regexp>> {
    recursive(|bf| choice((
            // parse meta sequences
            just(".").to(Regexp::Any),
            just("\\s").to(Regexp::Whitespace),
            just("\\S").to(Regexp::Not(Character::Whitespace)),
            just("\\d").to(Regexp::Digit),
            just("\\D").to(Regexp::Not(Character::Digit)),
            just("\\w").to(Regexp::Word),
            just("\\W").to(Regexp::Not(Character::Word)),
            // parse groups
            bf.delimited_by(just('('), just(')')).map(Regexp::Group),
        )).repeated()
        .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn try_parse(input: &str) -> Vec<Regexp> {
        match parser().parse(input) {
            Ok(r) => r,
            Err(e) => panic!("Failed on parsing regex: {}", e)
        }
    }

    #[test]
    fn test_meta_sequences() {
        assert_eq!(try_parse("."), vec![Regexp::Any]);
        assert_eq!(try_parse("\\s"), vec![Regexp::Whitespace]);
        assert_eq!(try_parse("\\S"), vec![Regexp::Not(Character::Whitespace)]);
        assert_eq!(try_parse("\\d"), vec![Regexp::Digit]);
        assert_eq!(try_parse("\\D"), vec![Regexp::Not(Character::Digit)]);
        assert_eq!(try_parse("\\d"), vec![Regexp::Word]);
        assert_eq!(try_parse("\\D"), vec![Regexp::Not(Character::Word)]);

        assert_eq!(try_parse("\\w\\s\\w\\D"), vec![Regexp::Word, Regexp::Whitespace, Regexp::Word, Regexp::Not(Character::Digit)]);
    }

    #[test]
    fn test_groups() {
        assert_eq!(try_parse("(.)"), vec![Regexp::Group(vec![Regexp::Any])]);
    }
}