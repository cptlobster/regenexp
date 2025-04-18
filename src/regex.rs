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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Regexp {
    Matcher(Matcher),
    Quantifier(Box<Regexp>, Quantifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Matcher {
    Universal,
    Literal(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quantifier {
    // Zero or one of (?)
    Optional,
    // Zero or more (greedy) (*)
    Greedy,
    // Zero or more (lazy) (*?)
    Lazy,
    // Exactly n occurrences ({n})
    Exact(usize),
    // At least n occurrences ({n,})
    AtLeast(usize),
    Range(usize, usize)
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Regexp>> {
    recursive(|bf| choice((
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
}