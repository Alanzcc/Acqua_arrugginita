pub struct Matrix2D {
    n_rows: usize,
    n_cols: usize,
    data: Vec<i8>,
}

impl Matrix2D {
    fn init_mat_zeros(&mut self) {
        self.data = vec![0; self.n_rows * self.n_cols]
    }
}

fn set_pixel(mut screen: Matrix2D, mut x: i32, mut y: i32, intensity: i8) {
    let c: i32 = screen.n_cols.try_into().unwrap();
    let r: i32 = screen.n_rows.try_into().unwrap();
    if x < 0 {
        x = 0;
    }
    if y < 0 {
        y = 0;
    }
    if x >= c {
        x = c - 1;
    }
    if y >= r {
        y = r - 1;
    }
    screen.data[x as usize * screen.n_cols + y as usize] = intensity;
}

fn main() {
    let mut mat = Matrix2D {
        n_rows: 4,
        n_cols: 4,
        data: vec![],
    };
    mat.init_mat_zeros();

    let x = 2;
    let y = 2;
    let a = 2;
    set_pixel(mat, x, y, a);
}
