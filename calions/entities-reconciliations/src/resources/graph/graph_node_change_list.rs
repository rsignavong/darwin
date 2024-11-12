use super::GraphNodeChange;
use derive_new::new;

#[derive(new)]
pub struct GraphNodeChangeList(Vec<GraphNodeChange>);

impl GraphNodeChangeList {
    pub fn for_each<F>(self, mut func: F)
    where
        F: FnMut(GraphNodeChange),
    {
        for change in self.0.into_iter() {
            func(change);
        }
    }
}
