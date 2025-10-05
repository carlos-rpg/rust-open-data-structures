pub struct AdjacencyMatrix {
    storage: Vec<bool>,
    side: usize,
}

impl AdjacencyMatrix {
    pub fn initialize(side: usize) -> Self {
        Self { storage: vec![false; side * side], side }
    }

    pub fn has_edge(&self, i: usize, j: usize) -> bool {
        self.get(i, j)
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        *self.get_mut(i, j) = true;
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        *self.get_mut(i, j) = false;
    }

    pub fn out_edges(&self, i: usize) -> Vec<usize> {
        (0..self.side)
            .filter(|&j| self.has_edge(i, j))
            .collect()
    }

    pub fn in_edges(&self, j: usize) -> Vec<usize> {
        (0..self.side)
            .filter(|&i| self.has_edge(i, j))
            .collect()
    }

    fn get(&self, i: usize, j: usize) -> bool {
        if self.is_out_of_bounds(i, j) {
            panic!("Out of bounds access: i, j = {}, {}", i, j)
        }
        self.storage[self.index(i, j)]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut bool {
        if self.is_out_of_bounds(i, j) {
            panic!("Out of bounds access: i, j = {}, {}", i, j)
        }
        let storage_idx = self.index(i, j);
        &mut self.storage[storage_idx]
    }

    fn is_out_of_bounds(&self, i: usize, j: usize) -> bool {
        i >= self.side || j >= self.side    
    }

    fn index(&self, i: usize, j: usize) -> usize {
        i * self.side + j
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
