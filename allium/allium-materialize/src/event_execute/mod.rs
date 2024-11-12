mod error;
mod node;

use crate::event_execute::node::Node;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    vec::IntoIter,
};

pub use error::EventExecuteError;

pub struct EventExecute;

impl EventExecute {
    pub fn recursive_filter(
        predicates: &[&str],
        nodes: &[&str],
    ) -> Result<Vec<String>, EventExecuteError> {
        let list = EventExecute::build_list(predicates, nodes)?;

        Ok(Vec::from_iter(list))
    }

    fn build_list(
        predicates: &[&str],
        nodes: &[&str],
    ) -> Result<HashSet<String>, EventExecuteError> {
        // deserialize all nodes
        let id_nodes: IntoIter<(String, Vec<String>)> = nodes
            .iter()
            .map(|node| serde_json::from_str::<Node>(node).map(|node| (node.id, node.next)))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        let nodes: HashMap<String, Vec<String>> = HashMap::from_iter(id_nodes);

        let mut execute_ids: HashSet<String> = HashSet::new();
        let initial_nodes: Vec<String> = predicates.iter().map(|p| p.to_string()).collect();

        EventExecute::add_recursively(&mut execute_ids, &initial_nodes, true, &nodes);

        Ok(execute_ids)
    }

    fn add_recursively(
        execute_ids: &mut HashSet<String>,
        current_nodes: &[String],
        is_predicate_nodes: bool,
        nodes: &HashMap<String, Vec<String>>,
    ) {
        for current_node in current_nodes.iter() {
            if let Some(next_nodes) = nodes.get(current_node) {
                if !execute_ids.insert(current_node.clone()) && !is_predicate_nodes {
                    continue;
                }
                EventExecute::add_recursively(execute_ids, next_nodes, false, nodes);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // select
    //   recursive_filter_event_executes(
    //     list [ 'a', 'b', 'c' ],
    //     list [ '{"id": "a", "next": ["b", "d"] }' :: jsonb,
    //     '{"id": "b", "next": ["c", "e"] }' :: jsonb,
    //     '{"id": "c", "next": ["f", "g"] }' :: jsonb,
    //     '{"id": "d", "next": ["b"] }' :: jsonb,
    //     '{"id": "e", "next": [] }' :: jsonb,
    //     '{"id": "f", "next": [] }' :: jsonb,
    //     '{"id": "g", "next": [] }' :: jsonb ]
    //   );

    #[test]
    fn single_root() {
        let predicates = &["1"];
        let nodes = &[r#"{
            "id": "1",
            "next": []
        }"#];

        let res = EventExecute::recursive_filter(predicates, nodes).unwrap();

        assert_eq!(res, vec!["1"]);
    }

    #[test]
    fn simple_recursive() {
        let predicates = &["1", "4"];
        let nodes = &[
            r#"{
            "id": "1",
            "next": ["2"]
        }"#,
            r#"{
            "id": "2",
            "next": ["3"]
        }"#,
            r#"{
            "id": "3",
            "next": []
        }"#,
            r#"{
            "id": "4",
            "next": []
        }"#,
        ];

        let mut res = EventExecute::recursive_filter(predicates, nodes).unwrap();
        res.sort_unstable();

        assert_eq!(res, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn complex_one() {
        let predicates = &["1"];
        let nodes = &[
            r#"{
            "id": "1",
            "next": ["2", "3"]
        }"#,
            r#"{
            "id": "2",
            "next": ["4", "5"]
        }"#,
            r#"{
            "id": "3",
            "next": ["6", "7"]
        }"#,
            r#"{
            "id": "4",
            "next": []
        }"#,
            r#"{
            "id": "5",
            "next": []
        }"#,
            r#"{
            "id": "6",
            "next": []
        }"#,
            r#"{
            "id": "7",
            "next": []
        }"#,
        ];

        let mut res = EventExecute::recursive_filter(predicates, nodes).unwrap();
        res.sort_unstable();

        assert_eq!(res, vec!["1", "2", "3", "4", "5", "6", "7"]);
    }

    #[test]
    fn complex_with_duplicate() {
        let predicates = &["1", "2", "3"];
        let nodes = &[
            r#"{
            "id": "1",
            "next": ["2", "4"]
        }"#,
            r#"{
            "id": "2",
            "next": ["3", "5"]
        }"#,
            r#"{
            "id": "3",
            "next": ["6", "7"]
        }"#,
            r#"{
            "id": "4",
            "next": ["2"]
        }"#,
            r#"{
            "id": "5",
            "next": []
        }"#,
            r#"{
            "id": "6",
            "next": []
        }"#,
            r#"{
            "id": "7",
            "next": []
        }"#,
        ];

        let mut res = EventExecute::recursive_filter(predicates, nodes).unwrap();
        res.sort_unstable();

        assert_eq!(res, vec!["1", "2", "3", "4", "5", "6", "7"]);
    }

    #[test]
    fn infinite_loop() {
        let predicates = &["1", "2", "3"];
        let nodes = &[
            r#"{
            "id": "1",
            "next": ["2", "4"]
        }"#,
            r#"{
            "id": "2",
            "next": ["3", "5", "1"]
        }"#,
            r#"{
            "id": "3",
            "next": ["6", "7"]
        }"#,
            r#"{
            "id": "4",
            "next": ["2"]
        }"#,
            r#"{
            "id": "5",
            "next": []
        }"#,
            r#"{
            "id": "6",
            "next": []
        }"#,
            r#"{
            "id": "7",
            "next": []
        }"#,
        ];

        let mut res = EventExecute::recursive_filter(predicates, nodes).unwrap();
        res.sort_unstable();

        assert_eq!(res, vec!["1", "2", "3", "4", "5", "6", "7"]);
    }

    #[test]
    fn missing_node_1() {
        let predicates = &["1"];
        let nodes = &[];

        let res = EventExecute::recursive_filter(predicates, nodes).unwrap();

        let expected: Vec<String> = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    fn missing_node_2() {
        let predicates = &["1"];
        let nodes = &[r#"{
            "id": "1",
            "next": ["2"]
        }"#];

        let res = EventExecute::recursive_filter(predicates, nodes).unwrap();

        assert_eq!(res, vec!["1"]);
    }

    #[test]
    fn missing_node_3() {
        let predicates = &["1", "3"];
        let nodes = &[r#"{
            "id": "1",
            "next": ["2"]
        }"#];

        let res = EventExecute::recursive_filter(predicates, nodes).unwrap();

        assert_eq!(res, vec!["1"]);
    }
}
