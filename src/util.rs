//! hydrogen model — drum machine/sequencer.

#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: String,
    pub type_: String,
    pub volume: f32,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub bpm: u32,
    pub instruments: Vec<Instrument>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub patterns: Vec<Pattern>,
}

impl Default for Model {
    fn default() -> Self { Self { patterns: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_pattern(&mut self, pattern: Pattern) { self.patterns.push(pattern); }

    pub fn list(&self) -> &[Pattern] { &self.patterns }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pattern() {
        let mut h = Model::new();
        h.add_pattern(Pattern { name: "beat1".into(), bpm: 120, instruments: vec![Instrument { name: "kick".into(), type_: "sample".into(), volume: 1.0 }] });
        assert_eq!(h.list().len(), 1);
    }
}
