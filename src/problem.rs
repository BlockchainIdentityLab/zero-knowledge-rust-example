pub struct PartictionProblem {
    pub partition_sizes: Vec<i64>,
    pub side_assignment: Vec<i64>,
}


impl PartictionProblem {
    pub fn new(partition_sizes: &[i64], side_assignment: &[i64]) -> Result<PartictionProblem, &'static str>  {
        if partition_sizes.len() != side_assignment.len() {
            return Err("Input vectors must be equal size to create witness");
        }
        Ok(PartictionProblem {
            partition_sizes: partition_sizes.to_vec(),
            side_assignment: side_assignment.to_vec()
        })
    }
    
}