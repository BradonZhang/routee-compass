use std::fmt::Display;

use crate::{
    algorithm::search::edge_traversal::EdgeTraversal, model::graphv2::vertex_id::VertexId,
};

#[derive(Clone, Debug)]
pub struct SearchTreeBranch {
    pub terminal_vertex: VertexId,
    pub edge_traversal: EdgeTraversal,
}

impl Display for SearchTreeBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-[edge:{} acost:{} tcost:{} state:{:?}]-> ({})",
            self.edge_traversal.edge_id,
            self.edge_traversal.access_cost,
            self.edge_traversal.traversal_cost,
            self.edge_traversal.result_state,
            self.terminal_vertex
        )
    }
}
