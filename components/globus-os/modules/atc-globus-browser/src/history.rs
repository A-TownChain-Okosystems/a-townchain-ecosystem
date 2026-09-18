use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry { pub url: String }

#[derive(Debug, Default, Clone)]
pub struct History {
    entries: VecDeque<HistoryEntry>,
    cursor: Option<usize>,
}
impl History {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, url: impl Into<String>) {
        if let Some(cursor) = self.cursor { self.entries.truncate(cursor + 1); }
        self.entries.push_back(HistoryEntry { url: url.into() });
        self.cursor = Some(self.entries.len() - 1);
    }
    pub fn current(&self) -> Option<&HistoryEntry> { self.cursor.and_then(|i| self.entries.get(i)) }
    pub fn back(&mut self) -> Option<&HistoryEntry> {
        let cursor = self.cursor?;
        if cursor == 0 { return None; }
        self.cursor = Some(cursor - 1);
        self.current()
    }
    pub fn forward(&mut self) -> Option<&HistoryEntry> {
        let cursor = self.cursor?;
        let next = cursor + 1;
        if next >= self.entries.len() { return None; }
        self.cursor = Some(next);
        self.current()
    }
    pub fn len(&self) -> usize { self.entries.len() }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn navigation_truncates_forward_history() {
        let mut h = History::new();
        h.push("https://a.example"); h.push("https://b.example");
        assert_eq!(h.back().map(|e| e.url.as_str()), Some("https://a.example"));
        h.push("https://c.example");
        assert!(h.forward().is_none());
        assert_eq!(h.len(), 2);
    }
}
