use crate::Matrix;
use crate::Point;
use crate::Vector;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix<f64, 4, 4> {
    let mut data = [[0.0; 4]; 4];
    data[0][3] = x;
    data[1][3] = y;
    data[2][3] = z;
    data[3][3] = 1.0;
    data[0][0] = 1.0;
    data[1][1] = 1.0;
    data[2][2] = 1.0;

    Matrix::from(data)
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<f64, 4, 4> {
    let mut data = [[0.0; 4]; 4];
    data[0][0] = x;
    data[1][1] = y;
    data[2][2] = z;
    data[3][3] = 1.0;

    Matrix::from(data)
}

pub fn rotation_x(angle: f64) -> Matrix<f64, 4, 4> {
    let mut data = [[0.0; 4]; 4];
    data[0][0] = 1.0;
    data[1][1] = angle.cos();
    data[1][2] = -angle.sin();
    data[2][1] = angle.sin();
    data[2][2] = angle.cos();
    data[3][3] = 1.0;

    Matrix::from(data)
}

pub fn rotation_y(angle: f64) -> Matrix<f64, 4, 4> {
    let mut data = [[0.0; 4]; 4];
    data[1][1] = 1.0;
    data[0][0] = angle.cos();
    data[2][0] = -angle.sin();
    data[0][2] = angle.sin();
    data[2][2] = angle.cos();
    data[3][3] = 1.0;

    Matrix::from(data)
}

pub fn rotation_z(angle: f64) -> Matrix<f64, 4, 4> {
    let mut data = [[0.0; 4]; 4];
    data[0][0] = angle.cos();
    data[0][1] = -angle.sin();
    data[1][0] = angle.sin();
    data[1][1] = angle.cos();
    data[2][2] = 1.0;
    data[3][3] = 1.0;

    Matrix::from(data)
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<f64, 4, 4> {
    let mut data = [[0.0; 4]; 4];
    data[0][0] = 1.0;
    data[1][1] = 1.0;
    data[2][2] = 1.0;
    data[3][3] = 1.0;
    data[0][1] = xy;
    data[0][2] = xz;
    data[1][0] = yx;
    data[1][2] = yz;
    data[2][0] = zx;
    data[2][1] = zy;

    Matrix::from(data)
}

pub fn view_transform(
    from: &Point<f64, 4>,
    to: &Point<f64, 4>,
    up: &Vector<f64, 4>,
) -> Matrix<f64, 4, 4> {
    let forward = Vector::normalize(*to - *from);
    let upn = up.normalize();
    let left = forward | upn;
    let true_up = left | forward;

    let orientation = Matrix::from([
        [
            left[0],
            left[1],
            left[2],
            0.0,
        ],
        [
            true_up[0],
            true_up[1],
            true_up[2],
            0.0,
        ],
        [
            forward[0],
            -(forward[1]),
            -(forward[2]),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    orientation
        * translation(
            -from[0],
            -from[1],
            -(from[2]),
        )
}
