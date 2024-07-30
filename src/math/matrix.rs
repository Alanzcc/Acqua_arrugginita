use sdl2::pixels::Color;

pub struct Matrix {
    n_rows: usize,
    n_cols: usize,
    data: Vec<Color>,
}

impl Matrix {
    pub fn new(n_rows: usize, n_cols: usize, data: Vec<Color>) -> Self {
        Matrix {
            n_rows,
            n_cols,
            data,
        }
    }

    pub fn init_mat_zeros(&mut self) {
        self.data = vec![
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            };
            self.n_rows * self.n_cols
        ];
    }

    pub fn get_n_cols(&self) -> usize {
        return self.n_cols;
    }

    pub fn get_n_rows(&self) -> usize {
        return self.n_rows;
    }

    pub fn get_data(&self) -> Vec<Color> {
        return self.data.clone();
    }

    pub fn set_data(&mut self, x: usize, y: usize, intensity: Color) {
        self.data[x * self.n_cols + y] = intensity;
    }
}
