#[cfg(test)]
mod test_solutions {
    use crate::solution_part_1;

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution_part_1("testData.txt"), 71);
    }
}