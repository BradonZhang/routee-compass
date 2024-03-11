use super::speed_traversal_engine::SpeedTraversalEngine;
use crate::model::road_network::edge_id::EdgeId;
use crate::model::state::state_feature::StateFeature;
use crate::model::state::state_model::StateModel;
use crate::model::traversal::traversal_model::TraversalModel;
use crate::model::unit::{Distance, Time, BASE_DISTANCE_UNIT};
use crate::model::{
    property::{edge::Edge, vertex::Vertex},
    traversal::{state::state_variable::StateVar, traversal_model_error::TraversalModelError},
    unit::Speed,
};
use crate::util::geo::haversine;
use std::sync::Arc;

pub struct SpeedTraversalModel {
    engine: Arc<SpeedTraversalEngine>,
}

impl SpeedTraversalModel {
    pub fn new(engine: Arc<SpeedTraversalEngine>) -> SpeedTraversalModel {
        SpeedTraversalModel { engine }
    }
}

impl TraversalModel for SpeedTraversalModel {
    fn traverse_edge(
        &self,
        trajectory: (&Vertex, &Edge, &Vertex),
        state: &mut Vec<StateVar>,
        state_model: &StateModel,
    ) -> Result<(), TraversalModelError> {
        let (_, edge, _) = trajectory;
        let distance = BASE_DISTANCE_UNIT.convert(&edge.distance, &self.engine.distance_unit);
        let speed = get_speed(&self.engine.speed_table, edge.edge_id)?;
        let edge_time = Time::create(
            speed,
            self.engine.speed_unit,
            distance,
            self.engine.distance_unit,
            self.engine.time_unit,
        )?;

        state_model.add_time(state, "time", &edge_time, &self.engine.time_unit)?;
        Ok(())
    }

    fn access_edge(
        &self,
        _trajectory: (&Vertex, &Edge, &Vertex, &Edge, &Vertex),
        _state: &mut Vec<StateVar>,
        _state_model: &StateModel,
    ) -> Result<(), TraversalModelError> {
        Ok(())
    }

    fn estimate_traversal(
        &self,
        od: (&Vertex, &Vertex),
        state: &mut Vec<StateVar>,
        state_model: &StateModel,
    ) -> Result<(), TraversalModelError> {
        let (src, dst) = od;
        let distance =
            haversine::coord_distance(&src.coordinate, &dst.coordinate, self.engine.distance_unit)
                .map_err(TraversalModelError::NumericError)?;

        if distance == Distance::ZERO {
            return Ok(());
        }

        let estimated_time = Time::create(
            self.engine.max_speed,
            self.engine.speed_unit,
            distance,
            self.engine.distance_unit,
            self.engine.time_unit,
        )?;
        state_model.add_time(state, "time", &estimated_time, &self.engine.time_unit)?;

        Ok(())
    }
    /// no additional state features are needed
    fn state_features(&self) -> Vec<(String, StateFeature)> {
        vec![]
    }
}

/// look up a speed from the speed table
pub fn get_speed(speed_table: &[Speed], edge_id: EdgeId) -> Result<Speed, TraversalModelError> {
    let speed: &Speed = speed_table.get(edge_id.as_usize()).ok_or_else(|| {
        TraversalModelError::MissingIdInTabularCostFunction(
            format!("{}", edge_id),
            String::from("EdgeId"),
            String::from("speed table"),
        )
    })?;
    Ok(*speed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::unit::{Distance, DistanceUnit, SpeedUnit, TimeUnit};
    use crate::model::{
        property::{edge::Edge, vertex::Vertex},
        road_network::{edge_id::EdgeId, vertex_id::VertexId},
    };
    use crate::util::geo::coord::InternalCoord;
    use geo::coord;
    use std::path::PathBuf;

    fn mock_vertex() -> Vertex {
        Vertex {
            vertex_id: VertexId(0),
            coordinate: InternalCoord(coord! {x: -86.67, y: 36.12}),
        }
    }
    fn mock_edge(edge_id: usize) -> Edge {
        Edge {
            edge_id: EdgeId(edge_id),
            src_vertex_id: VertexId(0),
            dst_vertex_id: VertexId(1),
            distance: Distance::new(100.0),
        }
    }
    fn filepath() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("model")
            .join("traversal")
            .join("default")
            .join("test")
            .join("velocities.txt")
    }

    fn approx_eq(a: f64, b: f64, error: f64) {
        let result = match (a, b) {
            (c, d) if c < d => d - c < error,
            (c, d) if c > d => c - d < error,
            (_, _) => true,
        };
        assert!(
            result,
            "{} ~= {} is not true within an error of {}",
            a, b, error
        )
    }

    #[test]
    fn test_edge_cost_lookup_with_seconds_time_unit() {
        let file = filepath();
        let engine = SpeedTraversalEngine::new(
            &file,
            SpeedUnit::KilometersPerHour,
            None,
            Some(TimeUnit::Seconds),
        )
        .unwrap();
        let state_model = Arc::new(
            StateModel::empty()
                .extend(vec![
                    (
                        String::from("distance"),
                        StateFeature::Distance {
                            distance_unit: DistanceUnit::Kilometers,
                            initial: Distance::new(0.0),
                        },
                    ),
                    (
                        String::from("time"),
                        StateFeature::Time {
                            time_unit: TimeUnit::Seconds,
                            initial: Time::new(0.0),
                        },
                    ),
                ])
                .unwrap(),
        );
        let model: SpeedTraversalModel = SpeedTraversalModel::new(Arc::new(engine));
        let mut state = state_model.initial_state().unwrap();
        let v = mock_vertex();
        let e1 = mock_edge(0);
        // 100 meters @ 10kph should take 36 seconds ((0.1/10) * 3600)
        model
            .traverse_edge((&v, &e1, &v), &mut state, &state_model)
            .unwrap();

        let expected = 36.0;
        // approx_eq(result.total_cost.into(), expected, 0.001);
        // approx_eq(result.updated_state[1].into(), expected, 0.001);
        approx_eq(state[1].into(), expected, 0.001);
    }

    #[test]
    fn test_edge_cost_lookup_with_milliseconds_time_unit() {
        let file = filepath();
        let engine = SpeedTraversalEngine::new(
            &file,
            SpeedUnit::KilometersPerHour,
            None,
            Some(TimeUnit::Milliseconds),
        )
        .unwrap();
        let state_model = Arc::new(
            StateModel::empty()
                .extend(vec![
                    (
                        String::from("distance"),
                        StateFeature::Distance {
                            distance_unit: DistanceUnit::Kilometers,
                            initial: Distance::new(0.0),
                        },
                    ),
                    (
                        String::from("time"),
                        StateFeature::Time {
                            time_unit: TimeUnit::Milliseconds,
                            initial: Time::new(0.0),
                        },
                    ),
                ])
                .unwrap(),
        );
        let model = SpeedTraversalModel::new(Arc::new(engine));
        let mut state = state_model.initial_state().unwrap();
        let v = mock_vertex();
        let e1 = mock_edge(0);
        // 100 meters @ 10kph should take 36,000 milliseconds ((0.1/10) * 3600000)
        model
            .traverse_edge((&v, &e1, &v), &mut state, &state_model)
            .unwrap();
        let expected = 36000.0;
        // approx_eq(result.total_cost.into(), expected, 0.001);
        // approx_eq(result.updated_state[1].into(), expected, 0.001);
        approx_eq(state[1].into(), expected, 0.001);
    }
}
