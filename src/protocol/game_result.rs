use std::collections::HashMap;

use crate::util::{Element, SCError, SCResult};

use super::{ScoreDefinition, Player, Score};

#[derive(Debug, Clone)]
pub struct GameResult {
    definition: ScoreDefinition,
    scores: HashMap<Player, Score>,
    winner: Player,
}

impl GameResult {
    #[inline]
    pub fn new(definition: ScoreDefinition, scores: impl Into<HashMap<Player, Score>>, winner: Player) -> Self {
        Self { definition, scores: scores.into(), winner }
    }

    #[inline]
    pub fn definition(&self) -> &ScoreDefinition { &self.definition }

    #[inline]
    pub fn scores(&self) -> &HashMap<Player, Score> { &self.scores }

    #[inline]
    pub fn winner(&self) -> &Player { &self.winner }
}

impl TryFrom<&Element> for GameResult {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Self {
            definition: elem.child_by_name("definition")?.try_into()?,
            scores: elem
                .child_by_name("scores")?
                .childs_by_name("entry")
                .map(|e| {
                    let player = Player::try_from(e.child_by_name("player")?)?;
                    let score = Score::try_from(e.child_by_name("score")?)?;
                    Ok((player, score))
                })
                .collect::<SCResult<_>>()?,
            winner: elem.child_by_name("winner")?.try_into()?,
        })
    }
}
