use serde::Deserialize;

use crate::model::graph::{edge_id::EdgeId, vertex_id::VertexId};

use crate::model::units::{Length, Ratio, Velocity};

use super::road_class::RoadClass;

#[derive(Copy, Clone, Deserialize, Debug, Default)]
pub struct Edge {
    pub edge_id: EdgeId,
    pub src_vertex_id: VertexId,
    pub dst_vertex_id: VertexId,
    pub road_class: RoadClass,
    pub distance: Length,
    pub grade: Ratio,
}
