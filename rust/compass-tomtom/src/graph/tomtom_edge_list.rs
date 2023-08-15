use std::{collections::HashMap, fs::File, io::BufReader};

use compass_core::model::{
    graph::{edge_id::EdgeId, vertex_id::VertexId},
    property::edge::Edge,
};
use flate2::read::GzDecoder;
use log::debug;

use super::{tomtom_graph_config::TomTomGraphConfig, tomtom_graph_error::TomTomGraphError};
use kdam::Bar;
use kdam::BarExt;

pub struct TomTomEdgeList {
    pub edges: Vec<Edge>,
    pub adj: Vec<HashMap<EdgeId, VertexId>>,
    pub rev: Vec<HashMap<EdgeId, VertexId>>,
}

pub struct TomTomEdgeListConfig<'a> {
    pub config: &'a TomTomGraphConfig,
    pub n_edges: usize,
    pub n_vertices: usize,
}

impl<'a> TryFrom<TomTomEdgeListConfig<'a>> for TomTomEdgeList {
    type Error = TomTomGraphError;

    fn try_from(c: TomTomEdgeListConfig) -> Result<Self, Self::Error> {
        let min_node_connectivity: usize = 1;
        let mut edges: Vec<Edge> = vec![Edge::default(); c.n_edges];
        let mut adj: Vec<HashMap<EdgeId, VertexId>> =
            vec![HashMap::with_capacity(min_node_connectivity); c.n_vertices];
        let mut rev: Vec<HashMap<EdgeId, VertexId>> =
            vec![HashMap::with_capacity(min_node_connectivity); c.n_vertices];

        let edge_list_file = File::open(c.config.edge_list_csv.clone())
            .map_err(|e| TomTomGraphError::IOError { source: e })?;
        let mut edge_reader =
            csv::Reader::from_reader(Box::new(BufReader::new(GzDecoder::new(edge_list_file))));
        let edge_rows = edge_reader.deserialize();

        let mut pb = Bar::builder()
            .total(c.n_edges)
            .animation("fillup")
            .desc("edge list")
            .build()
            .map_err(|e| TomTomGraphError::ProgressBarBuildError(String::from("edge list"), e))?;

        for row in edge_rows {
            let edge: Edge = row.map_err(|e| TomTomGraphError::CsvError { source: e })?;
            edges[edge.edge_id.0 as usize] = edge;
            // the Edge provides us with all id information to build our adjacency lists as well

            match adj.get_mut(edge.src_vertex_id.0 as usize) {
                None => {
                    return Err(TomTomGraphError::AdjacencyVertexMissing(edge.src_vertex_id));
                }
                Some(out_links) => {
                    out_links.insert(edge.edge_id, edge.dst_vertex_id);
                }
            }
            match rev.get_mut(edge.dst_vertex_id.0 as usize) {
                None => {
                    return Err(TomTomGraphError::AdjacencyVertexMissing(edge.dst_vertex_id));
                }
                Some(in_links) => {
                    in_links.insert(edge.edge_id, edge.src_vertex_id);
                }
            }
            pb.update(1);
        }
        print!("\n");
        let result = TomTomEdgeList { edges, adj, rev };

        Ok(result)
    }
}
