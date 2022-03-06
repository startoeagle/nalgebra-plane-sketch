use na::{Dynamic, OMatrix, Vector3, U3};
use nalgebra as na;
use random_number::rand::random;

type Points = OMatrix<f32, U3, Dynamic>;

fn main() {
    const N: usize = 3_000_000;
    let points = Points::from_fn(N, |_, _| random::<f32>());

    let plane = Vector3::from_fn(|_, _| random::<f32>());
    let d = 1.0;

    let mut projections = points.clone();
    projections.column_iter_mut().for_each(|mut point| {
        let tmp = plane.dot(&point) - d;
        let len = tmp / plane.norm();
        let tmp2 = plane.normalize() * len;
        point -= tmp2;
    });

    projections
        .column_iter()
        .for_each(|point| assert!(point.dot(&plane) - d < 0.0001));

    let data = &projections * &projections.transpose();
    let rhs: Vector3<f32> = projections.column_sum();
    let plane_new = data.lu().solve(&rhs);
    println!("Plane normals\n{:?} {:?}", plane_new, plane);
}
