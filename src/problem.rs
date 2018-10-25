pub struct PartictionProblem {
    pub partition_sizes: Vec<i32>,
    pub side_assignment: Vec<i32>,
}


impl PartictionProblem {
    pub fn new(partition_sizes: &[i32], side_assignment: &[i32]) -> Result<PartictionProblem, &'static str>  {
        if partition_sizes.len() != side_assignment.len() {
            return Err("Input vectors must be equal size to create witness");
        }
        Ok(PartictionProblem {
            partition_sizes: partition_sizes.to_vec(),
            side_assignment: side_assignment.to_vec()
        })
    }
    
}