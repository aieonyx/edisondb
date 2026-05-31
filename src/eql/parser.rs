use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use super::ast::{Statement, Tier};

#[derive(Parser)]
#[grammar = "eql/grammar.pest"]
struct EqlParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error: {0}")]
    Syntax(#[from] pest::error::Error<Rule>),
    #[error("Unknown tier: {0}")]
    UnknownTier(String),
    #[error("Internal parser error: {0}")]
    Internal(String),
}

pub fn parse(input: &str) -> Result<Statement, ParseError> {
    let input = input.trim();
    let mut pairs = EqlParser::parse(Rule::program, input)?;
    let pair = pairs
        .next()
        .ok_or_else(|| ParseError::Internal("empty parse tree".into()))?
        .into_inner()
        .next()
        .ok_or_else(|| ParseError::Internal("missing statement".into()))?;

    match pair.as_rule() {
        Rule::write_stmt  => parse_write(pair),
        Rule::read_stmt   => parse_read(pair),
        Rule::list_stmt   => parse_list(pair),
        Rule::delete_stmt => parse_delete(pair),
        Rule::audit_stmt  => parse_audit(pair),
        r => Err(ParseError::Internal(format!("unexpected rule: {r:?}"))),
    }
}

fn parse_tier(s: &str) -> Result<Tier, ParseError> {
    match s.to_uppercase().as_str() {
        "CRITICAL" => Ok(Tier::Critical),
        "PERSONAL" => Ok(Tier::Personal),
        "NOISE"    => Ok(Tier::Noise),
        other      => Err(ParseError::UnknownTier(other.to_string())),
    }
}

fn parse_write(pair: pest::iterators::Pair<Rule>) -> Result<Statement, ParseError> {
    let mut inner = pair.into_inner();
    let id      = inner.next().ok_or_else(|| ParseError::Internal("missing id".into()))?.as_str().to_string();
    let tier    = parse_tier(inner.next().ok_or_else(|| ParseError::Internal("missing tier".into()))?.as_str())?;
    let payload = inner.next().ok_or_else(|| ParseError::Internal("missing payload".into()))?.as_str().to_string();
    Ok(Statement::Write { id, tier, payload })
}

fn parse_read(pair: pest::iterators::Pair<Rule>) -> Result<Statement, ParseError> {
    let id = pair.into_inner().next()
        .ok_or_else(|| ParseError::Internal("missing id".into()))?.as_str().to_string();
    Ok(Statement::Read { id })
}

fn parse_list(pair: pest::iterators::Pair<Rule>) -> Result<Statement, ParseError> {
    let tier = pair.into_inner().next().map(|p| parse_tier(p.as_str())).transpose()?;
    Ok(Statement::List { tier })
}

fn parse_delete(pair: pest::iterators::Pair<Rule>) -> Result<Statement, ParseError> {
    let id = pair.into_inner().next()
        .ok_or_else(|| ParseError::Internal("missing id".into()))?.as_str().to_string();
    Ok(Statement::Delete { id })
}

fn parse_audit(pair: pest::iterators::Pair<Rule>) -> Result<Statement, ParseError> {
    let id = pair.into_inner().next().map(|p| p.as_str().to_string());
    Ok(Statement::Audit { id })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_write_critical() {
        assert_eq!(
            parse("WRITE rec-1 TIER CRITICAL my secret").unwrap(),
            Statement::Write { id: "rec-1".into(), tier: Tier::Critical, payload: "my secret".into() }
        );
    }

    #[test]
    fn parse_write_personal() {
        assert_eq!(
            parse("WRITE note TIER PERSONAL birthday reminder").unwrap(),
            Statement::Write { id: "note".into(), tier: Tier::Personal, payload: "birthday reminder".into() }
        );
    }

    #[test]
    fn parse_write_noise() {
        assert_eq!(
            parse("WRITE log1 TIER NOISE server started").unwrap(),
            Statement::Write { id: "log1".into(), tier: Tier::Noise, payload: "server started".into() }
        );
    }

    #[test]
    fn parse_read() {
        assert_eq!(parse("READ rec-1").unwrap(), Statement::Read { id: "rec-1".into() });
    }

    #[test]
    fn parse_list_all() {
        assert_eq!(parse("LIST").unwrap(), Statement::List { tier: None });
    }

    #[test]
    fn parse_list_tier() {
        assert_eq!(
            parse("LIST TIER CRITICAL").unwrap(),
            Statement::List { tier: Some(Tier::Critical) }
        );
    }

    #[test]
    fn parse_delete_stmt() {
        assert_eq!(parse("DELETE rec-1").unwrap(), Statement::Delete { id: "rec-1".into() });
    }

    #[test]
    fn parse_audit_global() {
        assert_eq!(parse("AUDIT").unwrap(), Statement::Audit { id: None });
    }

    #[test]
    fn parse_audit_specific() {
        assert_eq!(
            parse("AUDIT rec-1").unwrap(),
            Statement::Audit { id: Some("rec-1".into()) }
        );
    }

    #[test]
    fn parse_case_insensitive() {
        assert!(parse("write k1 tier critical hello").is_ok());
        assert!(parse("read k1").is_ok());
        assert!(parse("list tier noise").is_ok());
    }

    #[test]
    fn parse_unknown_tier_fails() {
        assert!(parse("WRITE k1 TIER SECRET payload").is_err());
    }

    #[test]
    fn parse_empty_fails() {
        assert!(parse("").is_err());
    }
}