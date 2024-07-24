pub struct Matrix {
    n_rows: usize,
    n_cols: usize,
    data: Vec<i8>,
}

impl Matrix {
    pub fn init_mat_zeros(&mut self) {
        self.data = vec![0; self.n_rows * self.n_cols]
    }

    pub fn get_n_cols(&self) -> usize {
        return self.n_cols;
    }

    pub fn get_n_rows(&self) -> usize {
        return self.n_rows;
    }

    pub fn set_data(&mut self, x: usize, y: usize, intensity: i8) {
        self.data[x * self.n_cols + y] = intensity;
    }
}
