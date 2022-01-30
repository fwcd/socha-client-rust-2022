use crate::util::{SCError, SCResult, Element};

use super::ScoreAggregation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreDefinitionFragment {
    aggregation: ScoreAggregation,
    relevant_for_ranking: bool,
}

impl ScoreDefinitionFragment {
    #[inline]
    pub fn aggregation(&self) -> ScoreAggregation { self.aggregation }

    #[inline]
    pub fn relevant_for_ranking(&self) -> bool { self.relevant_for_ranking }
}

impl TryFrom<&Element> for ScoreDefinitionFragment {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(ScoreDefinitionFragment {
            aggregation: elem.child_by_name("aggregation")?.content().parse()?,
            relevant_for_ranking: elem.child_by_name("relevantForRanking")?.content().parse()?,
        })
    }
}
