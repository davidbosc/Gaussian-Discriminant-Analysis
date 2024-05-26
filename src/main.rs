use std::{error::Error, fs::File, path::Path};
use std::ops::{Sub, AddAssign};

#[derive(Copy, Clone)]
struct DataStruct {
    x: f64,
    y: f64,
    class: i32
}
#[derive(Copy, Clone)]
struct Vector2f {
    x: f64,
    y: f64
}
#[derive(Copy, Clone)]
struct Matrix2x2f {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64
}

impl Vector2f {
    pub fn multiply_by_vector_transpose(&self, other: Vector2f) -> Matrix2x2f {
        let mut output: Matrix2x2f = Matrix2x2f::zero_matrix();

        output.x0 = self.x * other.x;
        output.x1 = self.x * other.y;
        output.y0 = self.y * other.x;
        output.y1 = self.y * other.y;
        
        return output;
    }

    pub fn zero() -> Vector2f {
        return Vector2f { x: 0.0, y: 0.0 };
    }

    pub fn scale(&self, scalar: f64) -> Vector2f {
        return Self {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }

    pub fn mul_with_matrix2x2f(&self, matrix: Matrix2x2f) -> Vector2f {
        return Self {
            x: self.x * matrix.x0 + self.y * matrix.y0,
            y: self.x * matrix.x1 + self.y * matrix.y1
        };
    }

    pub fn dot_product(&self, other: Vector2f) -> f64 {
        return self.x * other.x + self.y * other.y;
    }
}

impl Sub for Vector2f {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Vector2f {
        Self {x: self.x - _rhs.x, y: self.y - _rhs.y}
    }
}

impl AddAssign for Vector2f {
    fn add_assign(&mut self, _rhs: Self) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}

impl Matrix2x2f {
    pub fn zero_matrix() -> Matrix2x2f {
        return Matrix2x2f {x0: 0.0, x1: 0.0, y0: 0.0, y1: 0.0};
    }

    pub fn scalar_div(&self, _rhs: f64) -> Matrix2x2f {
        return Self {
            x0: self.x0 / _rhs,
            x1: self.x1 / _rhs,
            y0: self.y0 / _rhs,
            y1: self.y1 / _rhs
        };
    }

    pub fn determinate(&self) -> f64 {
        return self.x0 * self.y1 - self.x1 * self.y0;
    }

    pub fn inverse(&self) -> Matrix2x2f {
        let det: f64 = self.determinate();
        let temp: f64 = self.x0;

        return Self {
            x0: self.y1 / det,
            x1: -self.x1 / det,
            y0: -self.y0 / det,
            y1: temp / det
        };
    }
}

impl AddAssign for Matrix2x2f {
    fn add_assign(&mut self, _rhs: Self) {
        self.x0 += _rhs.x0;
        self.x1 += _rhs.x1;
        self.y0 += _rhs.y0;
        self.y1 += _rhs.y1;
    }
}

fn read_data(input: &Path) -> Result<Vec<DataStruct>, Box<dyn Error>> {
    let file: File = File::open(input)?;

    let mut reader: csv::Reader<File> = csv::Reader::from_reader(file);

    let mut temp: Vec<DataStruct> = Vec::<DataStruct>::new();
    for result in reader.records() {
        let record: csv::StringRecord = result?;
        println!("{:?}", record);
        
        let mut fields: csv::StringRecordIter<'_> = record.iter();
        let x: f64 = fields.next().ok_or("Missing x")?.trim().parse()?;
        let y: f64 = fields.next().ok_or("Missing y")?.trim().parse()?;
        let class: i32 = fields.next().ok_or("Missing class")?.trim().parse()?;

        let data_record: DataStruct = DataStruct {x, y, class};
        temp.push(data_record);
    }
    Ok(temp)
}

fn calculate_phi(input: Vec<DataStruct>) -> f64 {
    let m: f64 = input.iter().count() as f64;
    return input.iter().filter(|&y| y.class==1).count() as f64 / m;
}

fn calculate_sigma(input: Vec<DataStruct>, mu0: Vector2f, mu1: Vector2f) -> Matrix2x2f {
    let m: f64 = input.iter().count() as f64;
    let mut output: Matrix2x2f = Matrix2x2f {x0: 0.0, x1: 0.0, y0: 0.0, y1: 0.0};

    for data in input.iter() {
        let xi: Vector2f = Vector2f { x: data.x, y: data.y };
        let yi: i32 = data.class;
        let mu: Vector2f = if yi == 1 { mu1 }  else { mu0 };

        let xi_minus_mu: Vector2f = xi - mu;
        let sqr: Matrix2x2f = xi_minus_mu.multiply_by_vector_transpose(xi_minus_mu);
        output += sqr;
    }

    return output.scalar_div(m);
}

fn calculate_mu(input: Vec<DataStruct>, class: i32) -> Vector2f {
    let m: f64 = input.iter().count() as f64;
    let y_count: f64 = input.iter().filter(|&y| y.class==class).count() as f64;
    let mut vector_sum: Vector2f = Vector2f { x: 0.0, y: 0.0 };

    for data in input.iter() {
        let xi: Vector2f = Vector2f { x: data.x, y: data.y };
        let yi: i32 = data.class;
        if yi == class {
            vector_sum += xi;
        };
    }

    return vector_sum.scale(1.0/y_count).scale(1.0/m);
}

fn calculate_px_py(x: Vector2f, mu: Vector2f, sigma: Matrix2x2f) -> f64 {
    let n: f64 = 1.0;
    let pi: f64 = 3.14;
    let two_pi: f64 = 2.0*pi;
    let x_less_mu: Vector2f = x-mu;

    let ratio: f64 = 1.0 / (two_pi.powf(n/2.0)*(sigma.determinate().sqrt()));
    let exponent: f64 = f64::exp((x_less_mu).scale(-0.5).mul_with_matrix2x2f(sigma.inverse()).dot_product(x_less_mu));

    return ratio * exponent;
}

fn calculate_py(y: i32, phi: f64) -> f64 {
    return if y == 1 { phi } else { 1.0 - phi };
}

fn main() -> Result<(), Box<dyn Error>> {
    print!("Loading Data: Start...");
    
    let file_path: &Path = Path::new("data/iris.csv");
    let iris_data: Vec<DataStruct> = read_data(file_path)?;

    // for data in iris_data.iter() {
    //     println!("DataStruct: {}, {}, {}", data.x, data.y, data.class);
    // }

    println!("Done");

    println!("---------------------------------------------------------------");
    
    let phi: f64 = calculate_phi(iris_data.clone());
    let mu0: Vector2f = calculate_mu(iris_data.clone(), 0);
    let mu1: Vector2f = calculate_mu(iris_data.clone(), 1);
    let sigma: Matrix2x2f = calculate_sigma(iris_data.clone(), mu0, mu1);
    
    println!("phi = {}", phi);
    println!("mu0 = ({}, {})", mu0.x, mu0.y);
    println!("mu1 = ({}, {})", mu1.x, mu1.y);
    println!("sigma = [({}, {}), ({}, {})]", sigma.x0, sigma.x1, sigma.y0, sigma.y1);
    
    println!("---------------------------------------------------------------");

    let x0: Vector2f = Vector2f { x: 4.0, y: 4.0 };
    let x1: Vector2f = Vector2f { x: 6.5, y: 2.25 };

    let px0_0: f64 = calculate_px_py(x0, mu0, sigma)*calculate_py(0, phi);
    let px0_1: f64 = calculate_px_py(x0, mu1, sigma)*calculate_py(1, phi);

    let px1_0: f64 = calculate_px_py(x1, mu0, sigma)*calculate_py(0, phi);
    let px1_1: f64 = calculate_px_py(x1, mu1, sigma)*calculate_py(1, phi);

    println!("px0_0 = {}", px0_0);
    println!("px0_1 = {}", px0_1);
    println!("px1_0 = {}", px1_0);
    println!("px1_1 = {}", px1_1);

    Ok(())
}