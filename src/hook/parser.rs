use std::collections::VecDeque;

use crate::hook::{HookSpan, VersionSpan};

use crate::hook::error::HookParseError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser as ParserDerive;
use semver::{BuildMetadata, Prerelease};

#[doc(hidden)]
#[derive(ParserDerive)]
#[grammar = "hook/version_dsl.pest"]
struct HookDslParser;

#[derive(Debug, Eq, PartialEq)]
pub enum VersionAccessToken {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Version,
    VersionTag,
    LatestVersion,
    LatestVersionTag,
    Package,
    Amount(u64),
    Add,
    Major,
    Minor,
    Patch,
    PreRelease(Prerelease),
    BuildMetadata(BuildMetadata),
    VersionAccess(VersionAccessToken),
}

pub fn parse(hook: &str) -> Result<HookSpan, HookParseError> {
    let pairs = HookDslParser::parse(Rule::version_dsl, hook)?
        .next()
        .unwrap();

    let mut span = HookSpan {
        version_spans: vec![],
        content: pairs.as_str().to_string(),
    };

    for pair in pairs.into_inner() {
        if pair.as_rule() == Rule::version {
            let version_span = parse_version(pair)?;
            span.version_spans.push(version_span);
        }
    }

    Ok(span)
}

fn parse_version(pair: Pair<Rule>) -> Result<VersionSpan, HookParseError> {
    let mut tokens = VecDeque::new();

    let start = pair.as_span().start();
    let end = pair.as_span().end();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::current_version => tokens.push_back(Token::Version),
            Rule::current_tag => tokens.push_back(Token::VersionTag),
            Rule::latest_version => tokens.push_back(Token::LatestVersion),
            Rule::latest_tag => tokens.push_back(Token::LatestVersionTag),
            Rule::package => tokens.push_back(Token::Package),
            Rule::ops => parse_operator(&mut tokens, pair.into_inner())?,
            Rule::pre_release => {
                let identifiers = pair.into_inner().next().unwrap();
                let semver_pre_release = Prerelease::new(identifiers.as_str())?;
                tokens.push_back(Token::PreRelease(semver_pre_release));
            }
            Rule::build_metadata => {
                let identifiers = pair.into_inner().next().unwrap();
                let semver_build_meta = BuildMetadata::new(identifiers.as_str())?;
                tokens.push_back(Token::BuildMetadata(semver_build_meta));
            }
            Rule::version_access_major => {
                tokens.push_back(Token::VersionAccess(VersionAccessToken::Major))
            }
            Rule::version_access_minor => {
                tokens.push_back(Token::VersionAccess(VersionAccessToken::Minor))
            }
            Rule::version_access_patch => {
                tokens.push_back(Token::VersionAccess(VersionAccessToken::Patch))
            }
            _ => (),
        }
    }

    Ok(VersionSpan {
        range: start..end,
        tokens,
    })
}

fn parse_operator(
    tokens: &mut VecDeque<Token>,
    pairs: Pairs<'_, Rule>,
) -> Result<(), HookParseError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::add => tokens.push_back(Token::Add),
            Rule::amt => tokens.push_back(Token::Amount(str::parse::<u64>(pair.as_str()).unwrap())),
            Rule::major => tokens.push_back(Token::Major),
            Rule::minor => tokens.push_back(Token::Minor),
            Rule::patch => tokens.push_back(Token::Patch),
            _ => (),
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::hook::parser::Token;
    use crate::hook::{parser, VersionSpan};

    use semver::Prerelease;
    use speculoos::prelude::*;

    #[test]
    fn parse_version_and_latest() {
        let result = parser::parse("the latest {{latest+1minor}}, the greatest {{version+patch}}");
        assert_that!(result)
            .is_ok()
            .map(|span| &span.version_spans)
            .contains(&VersionSpan {
                range: 11..28,
                tokens: VecDeque::from(vec![
                    Token::LatestVersion,
                    Token::Add,
                    Token::Amount(1),
                    Token::Minor,
                ]),
            });
    }

    #[test]
    fn parse_version_tag() -> anyhow::Result<()> {
        let span =
            parser::parse("the latest {{latest_tag+1minor}}, the greatest {{version_tag+patch}}")?;

        assert_that!(&span.version_spans).contains(&VersionSpan {
            range: 11..32,
            tokens: VecDeque::from(vec![
                Token::LatestVersionTag,
                Token::Add,
                Token::Amount(1),
                Token::Minor,
            ]),
        });

        assert_that!(&span.version_spans).contains(&VersionSpan {
            range: 47..68,
            tokens: VecDeque::from(vec![Token::VersionTag, Token::Add, Token::Patch]),
        });

        Ok(())
    }

    #[test]
    fn parse_version_with_pre_release() {
        let result = parser::parse("the greatest {{version+patch-pre.alpha0}}");
        assert_that!(result)
            .is_ok()
            .map(|span| &span.version_spans)
            .contains(&VersionSpan {
                range: 13..41,
                tokens: VecDeque::from(vec![
                    Token::Version,
                    Token::Add,
                    Token::Patch,
                    Token::PreRelease(Prerelease::new("pre.alpha0").unwrap()),
                ]),
            });
    }

    #[test]
    fn parse_package() {
        let result = parser::parse("version package: {{package}}");
        assert_that!(result)
            .is_ok()
            .map(|span| &span.version_spans)
            .contains(&VersionSpan {
                range: 17..28,
                tokens: VecDeque::from(vec![Token::Package]),
            });
    }

    #[test]
    fn invalid_dsl_is_err() {
        let result = parser::parse("the greatest {{+patch-pre.alpha0}}");

        assert_that!(result).is_err();
    }
}
