//! hydrogen model — drum machine/sequencer.

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub bpm: u32,
    pub steps: Vec<String>,
    pub playing: bool,
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

    pub fn add_pattern(&mut self, name: &str, bpm: u32) -> usize {
        let id = self.patterns.len();
        self.patterns.push(Pattern { name: name.into(), bpm, steps: vec![], playing: false });
        id
    }

    pub fn add_step(&mut self, pattern_id: usize, step: &str) -> bool {
        if let Some(p) = self.patterns.get_mut(pattern_id) {
            p.steps.push(step.into());
            true
        } else { false }
    }

    pub fn play(&mut self, pattern_id: usize) -> bool {
        if let Some(p) = self.patterns.get_mut(pattern_id) {
            p.playing = true;
            true
        } else { false }
    }

    pub fn stop(&mut self, pattern_id: usize) -> bool {
        if let Some(p) = self.patterns.get_mut(pattern_id) {
            p.playing = false;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Pattern] { &self.patterns }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pattern_play_stop() {
        let mut h = Model::new();
        let id = h.add_pattern("beat1", 120);
        assert!(h.add_step(id, "kick"));
        assert!(h.play(id));
        assert!(h.stop(id));
    }
}
