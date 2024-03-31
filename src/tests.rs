#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use crate::project::csproj::CSProject;
    use crate::project::solution::SolutionInfo;
    use crate::utils::{compare_hash};

    #[test]
    fn create_solution() {
        let _ = fs::remove_dir_all("target/Test");
        let sln = SolutionInfo::new("hello", PathBuf::new().join("target/Test"));
        println!("{:?}", sln);
        assert_eq!(sln.create(false).is_ok(), true);
    }
    #[test]
    fn create_csproj(){
        let csproj = CSProject::new("TEST","SSS");
        let sln = SolutionInfo::new("hello", PathBuf::new().join("target/Test"));
        let result = csproj.create(&sln, true);
        println!("{:?}",&result);
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn test_hash(){
        let re = compare_hash();
        println!("{:?}", re);
    }

}
