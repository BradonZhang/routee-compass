use super::graph_config::GraphConfig;
use crate::{
    model::{
        graphv2::graph_error::GraphError,
        graphv2::{edge_id::EdgeId, vertex_id::VertexId},
        property::edge::Edge,
    },
    util::fs::read_utils,
};
use kdam::Bar;
use kdam::BarExt;
use std::collections::{HashMap, HashSet};

pub struct EdgeLoader {
    pub edges: Vec<Edge>,
    pub adj: Vec<HashMap<EdgeId, VertexId>>,
    pub rev: Vec<HashMap<EdgeId, VertexId>>,
}

pub struct EdgeLoaderConfig<'a> {
    pub config: &'a GraphConfig,
    pub n_edges: usize,
    pub n_vertices: usize,
}

impl<'a> TryFrom<EdgeLoaderConfig<'a>> for EdgeLoader {
    type Error = GraphError;

    fn try_from(c: EdgeLoaderConfig) -> Result<Self, Self::Error> {
        let min_node_connectivity: usize = 1;
        let mut adj: Vec<HashMap<EdgeId, VertexId>> =
            vec![HashMap::with_capacity(min_node_connectivity); c.n_vertices];
        let mut rev: Vec<HashMap<EdgeId, VertexId>> =
            vec![HashMap::with_capacity(min_node_connectivity); c.n_vertices];

        let mut pb = Bar::builder()
            .total(c.n_edges)
            .animation("fillup")
            .desc("edge list")
            .build()
            .map_err(|e| GraphError::ProgressBarBuildError(String::from("edge list"), e))?;

        let mut missing_vertices: HashSet<VertexId> = HashSet::new();
        let cb = Box::new(|edge: &Edge| {
            // the Edge provides us with all id information to build our adjacency lists as well
            match adj.get_mut(edge.src_vertex_id.0 as usize) {
                None => {
                    missing_vertices.insert(edge.src_vertex_id);
                }
                Some(out_links) => {
                    out_links.insert(edge.edge_id, edge.dst_vertex_id);
                }
            }
            match rev.get_mut(edge.dst_vertex_id.0 as usize) {
                None => {
                    missing_vertices.insert(edge.dst_vertex_id);
                }
                Some(in_links) => {
                    in_links.insert(edge.edge_id, edge.src_vertex_id);
                }
            }
            pb.update(1);
        });

        let edges =
            read_utils::vec_from_csv(&c.config.edge_list_csv, true, Some(c.n_edges), Some(cb))?;

        print!("\n");
        let result = EdgeLoader { edges, adj, rev };

        Ok(result)
    }
}
