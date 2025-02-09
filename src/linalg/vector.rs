struct vector {
    data: Vec<f64>,
}

impl vector {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    pub fn zeros(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
        }
    }

    pub fn ones(size: usize) -> Self {
        Self {
            data: vec![1.0; size],
        }
    }
}
