#[test]
fn test() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[0, 0, 0, 0, 0, 0]);
    let z = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let result = x + y;
    //println!("The result of the operation is: {}", result);
    assert_eq!(z, result);
    //assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}

#[test]
fn test_new() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    println!("{:?}", x.row);
    println!("{:?}", x.col);
    println!("{:?}", x.data);

    assert_eq!(x.row, 2 as usize);
    assert_eq!(x.col, 3 as usize);
    assert_eq!(x.data[0], -2);
}

#[test]
fn test_new_empty() {
    let x:Matrix<i32> = Matrix::new_empty(2,3);
    assert_eq!(x.row, 2);
    assert_eq!(x.col, 3);
}

#[test]
fn test_data() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let shared_ref = x.data();
    assert_eq!(x.data[0], shared_ref[0]);
}

#[test]
fn test_mut_data() {
    let mut x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let data = x.mut_data();
    data[0] = 5;

    assert_eq!(data[0], 5);
}

#[test]
fn test_size() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = x.size();

    assert_eq!(y.0, 2);
    assert_eq!(y.1, 3);
}

 #[test]
fn test_add() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
    let z = x + y;
    assert_eq!(z.data, [-1,0,1,2,3,4]);
    //assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}

#[test]
fn test_sub() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
    let z = x - y;
    assert_eq!(z.data, [-3,-2,-1,0,1,2]);
}

#[test]
#[should_panic]
//#[ignore]
fn test_add_panic() {
    let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
    let _z = x + y;
}

#[test]
#[should_panic]
//#[ignore]
fn test_sub_panic() {
    let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
    let _z = x - y;
}

#[test]
fn test_fmt() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}

#[test]
fn test_mul() {
    let x = Matrix::new(3, 3, &[1,2,3,4,5,6,7,8,9]);
    let y = Matrix::new(3, 3, &[9,8,7,2,2,2,1,1,1]);
    let z = x * y;
    assert_eq!(z.data, [16,15,14,52,48,44,88,81,74]);
}

/*#[test]
fn test2() {
    let x = Matrix::new(2, 2, &[1, 2, 3, 4]);
    let y = Matrix::new(2, 2, &[4, 3, 2, 1]);
    let z = Matrix::new(2, 2, &[8, 5, 20, 13]);
    let result = x + y;
    //println!("The result of the operation is: {}", result);
    assert_eq!(z, result);
    //assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}*/

use std::{ops, fmt};
use std::ops::Add;

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        //let new_matrix = Matrix<T> { data: values, row: row, col: col };
        //return new_matrix;
        Matrix::<T> {
            row: row,
            col: col,
            data: values.to_vec(),

        }
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
    let mut empty_vec = Vec::new();
        /*Matrix::<T> {
            row: 0,
            col: 0,
            data: empty_vec,
        }*/
    let new_matrix = Matrix::<T> { row: row, col: col, data: empty_vec };
        return new_matrix;
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        return &self.data;
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        return &mut self.data;
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        return (self.row, self.col);
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

//let mut stack: Vec<usize> = vec![];
    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row`, panic.
    fn add(self, rhs: Self) -> Self::Output {

        if self.row != rhs.row {
            panic!("self and rhs are not equal!");
        } else {
        let mut size = self.row * self.col;
        //let mut index = 0;
        let mut stack: Vec<T> = vec![];
            for i in  0 .. size {
                let self_data = self.data[i];
                let rhs_data = rhs.data[i];
                let result = self_data + rhs_data;
                stack.push(result);

            }
            Matrix::<T> {
                row: self.row,
                col: self.col,
                data: stack,
            }
        }
}
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row {
            panic!("self and rhs are not equal!");
        } else {
        let mut size = self.row * self.col;
        //let mut index = 0;
        let mut stack: Vec<T> = vec![];
            for i in  0 .. size {
                let self_data = self.data[i];
                let rhs_data = rhs.data[i];
                let result = self_data - rhs_data;
                stack.push(result);

            }
            Matrix::<T> {
                row: self.row,
                col: self.col,
                data: stack,
            }
        }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T> + Copy + Default> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {

    let mut stack1: Vec<T> = vec![];
    let mut stack2: Vec<T> = vec![];
    let mut stack3: Vec<T> = vec![];
    let mut empty_stack: Vec<T> = vec![];
    let mut result_vec = vec![];

        if self.col != rhs.row {
            panic!("self and rhs are not equal!");
        } else {

            for i in 0 .. self.row {
                for j in 0 .. rhs.col {
                    //stack1[j + i * rhs.col] * stack1[]
                    //let mut var : T = Default::default;




                    let mut var : T = Default::default();
                    // initialize everthing in the stack to be zeroes
                    /*let mut done = false;
                    while !done {
                        let x = stack1.pop().unwrap();
                        if stack1.len() == 0 {
                            done = true;
                        }
                    }*/

                    //let result = stack2.iter().fold(0, |a, &b| a + b);

                    /*let mut done = false;

                    while !done {
                        if stack2.len() > 0 {
                            let mut pushed_var : T = empty_stack.pop().unwrap();
                            let z: T = stack2.pop().unwrap();
                            let dot_product = pushed_var + z;
                            stack3.push(dot_product);
                        } else if stack2.len() == 0 {
                            done = true;
                        }
                    }*/
                    /*for o in 0 .. stack1.len() {
                        //let mut result = 0;
                        let mut variable : T;
                        if stack1.len() > 0 {
                            //let mut pushed_var : T = stack1.pop().unwrap();
                            //let mut pushed_var2 : T = stack1.pop().unwrap();
                            let result = stack1.pop().unwrap() + stack1.pop().unwrap();
                            stack2.push(result);

                        }
                        //result = result + stack2[o];
                        //stack3.push(result);
                        //if stack2.len() > 0 {
                            //let mut pushed_var : T = stack1.pop().unwrap();
                            //let z: T;
                            //let dot_product = pushed_var + z;
                            //stack3.push(dot_product);
                    }*/
                    //}

                    for k in 0 .. rhs.row {
                        let self_index = self.data[k + i * rhs.row];
                        let rhs_index = rhs.data[j + k * rhs.col];
                        //let result : T = self_index * rhs_index;
                        //println!("self.data: [{}]\nrhs.data: [{}]", k + i * rhs.row, j + k * rhs.col);
                        var = var + self_index * rhs_index

                        //println!("Stack 1's content is stack1[{}]", j * self.row + i);
                        //println!("self.data: [{}]\nrhs.data: [{}]", k + i * rhs.row, j + k * rhs.col);
                        //let result = self_index + rhs_index;
                        //stack1.push(result);
                        // while loop
                        //let mut done = false;

                        /*while !done {
                            if stack1.len() > 0 {

                            let x: T = stack1.pop().unwrap();
                            //println!("The values popped are {}", stack1.pop());
                            stack2.push(x);
                            } else if stack1.len() == 0 {
                            //stack2.iter().fold(0, |a, &b| a + b);
                            //stack3.push()
                            done = true;
                        }

                        //for tokens in stack2
                            result_vec.push(var);
                    }*/

                    }
                    //stack1.push(stack1[j * self.row + i]);
                    result_vec.push(var);
                }
            }
            /*let matrix_size = self.row * rhs.col;
            for i in 0 .. matrix_size {
                println!("[{}]", stack1[i]);
            }*/
            Matrix::<T> {
                row: self.row,
                col: self.col,
                data: result_vec,
            } // end return struct
        } // end if else
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line.
    ///  No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space,
    /// except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    // ex. "-2 -1 0\n1 2 3\n"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut resultMatrix = "".to_string();
        let mut stringSpace = " ";
        let mut stringNewline = "\n";
        let mut columnIndex = 0;
        let mut k = 0;
        let mut forloop_iteration = 0;

        for i in 0..self.row {
                //resultMatrix = resultMatrix + &stringElement  + &stringSpace;
                //println!("Result Matrix is {}", resultMatrix);
                println!("Self data in row is {}", self.data[i]);
                forloop_iteration = forloop_iteration + 1;
            for j in 0..self.col {

                if j == self.col - 1 {
                println!("For loop iteration count is: {}", forloop_iteration);
                println!("Self data in column is {}", self.data[k]);
                let mut stringElement : String = self.data[k].to_string();
                resultMatrix = resultMatrix + &stringElement;
                println!("Result Matrix is {}", resultMatrix);
                //println!("Self data is {}", self.data)

            } else {
                println!("For loop iteration count is: {}", forloop_iteration);
                println!("Self data in column is {}", self.data[k]);
                let mut stringElement : String = self.data[k].to_string();
                resultMatrix = resultMatrix + &stringElement + &stringSpace;
                println!("Result Matrix is {}", resultMatrix);
            }

                //println!("Self data in column is {}", self.data[i]);
                //resultMatrix = resultMatrix + &stringElement + &stringSpace;
                //println!("Result Matrix is {}", resultMatrix);
                k = k + 1;
            }
            resultMatrix = resultMatrix + &stringNewline
        }
        return write!(f, "{}", resultMatrix);

    }
}
