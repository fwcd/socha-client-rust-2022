use crate::{game::Team, util::{Element, SCError, SCResult}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    name: String,
    team: Team,
}

impl Player {
    #[inline]
    pub fn new(name: &str, team: Team) -> Self {
        Self { name: name.to_owned(), team }
    }

    #[inline]
    pub fn name(&self) -> &str { self.name.as_str() }

    #[inline]
    pub fn team(&self) -> Team { self.team }
}

impl TryFrom<&Element> for Player {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Player {
            name: elem.attribute("name")?.to_owned(),
            team: elem.attribute("team")?.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, protocol::Player, game::Team};

    #[test]
    fn test_parsing() {
        assert_eq!(Player::try_from(&Element::from_str(r#"
            <player name="Alice" team="ONE" />
        "#).unwrap()).unwrap(), Player::new("Alice", Team::One));
    }
}
