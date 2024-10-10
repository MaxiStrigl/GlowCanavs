
#[derive(Clone)]
pub struct Segment {
    points: Vec<(f64, f64)>,
    max_size: usize,
}

impl Segment {
    pub fn new(max_size: usize) -> Self {
        Segment {
            points: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, value: (f64, f64)) {
        if self.points.len() >= self.max_size {
            let items_to_remove = self.points.len() - self.max_size + 1;
            self.points.drain(0..items_to_remove);
        }
        self.points.push(value);
    }

    pub fn clear(&mut self) {
        self.points.clear();
    }

    pub fn len(&mut self) -> usize {
        self.points.len()
    }

    pub fn get_points(&mut self) -> &Vec<(f64, f64)> {
        &self.points
    }

    pub fn peek(&mut self) -> (f64, f64) {
        let index = self.len() - 1;

        *self.points.get(index).expect("Stack is empty")
    }
}
