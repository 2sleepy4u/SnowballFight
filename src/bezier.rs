use bevy::prelude::*;

use crate::rotate_point;

#[derive(Clone, Debug)]
pub struct BezierCurve {
    pub points: Vec<Vec3>
}

impl BezierCurve {
    pub fn add_point(mut self, point: Vec3) -> Self {
        self.points.push(point);
        self
    }

    pub fn from_vec(list: Vec<Vec3>) -> Self { 
        let mut bezier = BezierCurve::default();
        for point in list {
            bezier.points.push(point);
        }
        bezier 
    }
    
    pub fn compute(&self, t: f32) -> Vec3 {
        if self.points.len() < 2 {
            error!("Not enough points. Points found: {}", self.points.len());
            return Vec3::ZERO;
        }
        return BezierCurve::recursive_bezier(self.points.clone(), t);
    }

    pub fn start_from_point(mut self, start: Vec3) -> Self {
        self.points = self.points.iter().map(|p| *p + start).collect();
        return self
    }

    pub fn rotate_points(mut self, direction: Vec3) -> Self {
        self.points = self.points.iter().map(|p| rotate_point(*p, direction)).collect();
        self
    }

    fn recursive_bezier(points: Vec<Vec3>, t: f32) -> Vec3 {
        if points.len() == 1 {
            return points[0];
        }

        let a: Vec3 = BezierCurve::recursive_bezier(points[..points.len() - 1].to_vec(), t);
        let b: Vec3 = BezierCurve::recursive_bezier(points[1..].to_vec(), t);

        return a +( b - a ) * t; 
    }
}

impl Default for BezierCurve {
    fn default() -> Self { BezierCurve { points: Vec::new() } }
}


