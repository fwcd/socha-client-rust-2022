use crate::util::{Element, SCError, SCResult};

use super::ScoreDefinitionFragment;

#[derive(Debug, Clone)]
pub struct ScoreDefinition {
    fragments: Vec<ScoreDefinitionFragment>,
}

impl ScoreDefinition {
    #[inline]
    pub fn fragments(&self) -> &Vec<ScoreDefinitionFragment> { &self.fragments }
}

impl TryFrom<&Element> for ScoreDefinition {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(ScoreDefinition {
            fragments: elem.childs_by_name("fragment").map(ScoreDefinitionFragment::try_from).collect::<SCResult<_>>()?,
        })
    }
}
