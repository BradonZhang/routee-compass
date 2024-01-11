use routee_compass_core::model::{
    road_network::edge_id::EdgeId, traversal::traversal_model_error::TraversalModelError,
    unit::Grade,
};

use super::energy_model_service::EdgeHeading;

pub const ZERO_ENERGY: f64 = 1e-9;

/// look up the grade from the grade table
pub fn get_grade(
    grade_table: &Option<Box<[Grade]>>,
    edge_id: EdgeId,
) -> Result<Grade, TraversalModelError> {
    match grade_table {
        None => Ok(Grade::ZERO),
        Some(gt) => {
            let grade: &Grade = gt.get(edge_id.as_usize()).ok_or_else(|| {
                TraversalModelError::MissingIdInTabularCostFunction(
                    format!("{}", edge_id),
                    String::from("EdgeId"),
                    String::from("grade table"),
                )
            })?;
            Ok(*grade)
        }
    }
}

/// lookup up the edge heading from the headings table
pub fn get_headings(
    headings_table: &[EdgeHeading],
    edge_id: EdgeId,
) -> Result<EdgeHeading, TraversalModelError> {
    let heading: &EdgeHeading = headings_table.get(edge_id.as_usize()).ok_or_else(|| {
        TraversalModelError::MissingIdInTabularCostFunction(
            format!("{}", edge_id),
            String::from("EdgeId"),
            String::from("headings table"),
        )
    })?;
    Ok(*heading)
}

pub fn compute_headings_angle(a: EdgeHeading, b: EdgeHeading) -> i16 {
    (b.start_heading - a.end_heading + 180) % 360 - 180
}
