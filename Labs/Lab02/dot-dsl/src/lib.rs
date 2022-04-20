pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;
            use maplit::hashmap;

            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Node {
                pub label: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: label.to_string(),
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        label: self.label.clone(),
                        attrs: attrs
                            .iter()
                            .map(|(a,b)| (a.to_string(), b.to_string()))
                            .collect(),
                    }
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }

            }
        }

        pub mod edge {
            use std::collections::HashMap;
            use maplit::hashmap;

            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Edge {
                pub node1: String,
                pub node2: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Edge {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        node1: self.node1.clone(),
                        node2: self.node2.clone(),
                        attrs: attrs
                            .iter()
                            .map(|(a,b)| (a.to_string(), b.to_string()))
                            .collect(),
                    }
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }
            }
        }
    }

    #[derive(Default)]
    pub struct Graph{
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()  /* it permits to avoid to write:
                              * Graph { nodes: vec![], edges: vec![], attrs: hashmap! {}, */
        }

        pub fn with_nodes(&self, nodes: &Vec<Node>) -> Self {
            Graph {
                nodes: nodes.clone(),
                edges: self.edges.clone(),
                attrs: self.attrs.clone(),
            }
        }

        pub fn with_edges(&self, edges: &Vec<Edge>) -> Self {
            Graph {
                nodes: self.nodes.clone(),
                edges: edges.clone(),
                attrs: self.attrs.clone(),
            }
        }

        pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
                attrs: attrs
                    .iter()
                    .map(|(a,b)| (a.to_string(), b.to_string()))
                    .collect(),
            }
        }

        pub fn get_node(&self, label: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.label == label )
        }

        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|value| value.as_str())
        }
    }
}
