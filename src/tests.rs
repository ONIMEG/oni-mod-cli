mod tests {
    use std::fs;
    use std::path::PathBuf;
    use crate::data::config::AppConfigInfo;
    use crate::project::csproj::CSProject;
    use crate::project::solution::SolutionInfo;
    #[test]
    fn create_solution() {
        let _ = fs::remove_dir_all("target/Test");
        let sln = SolutionInfo::new("hello", PathBuf::new().join("target/Test"));
        println!("{:?}", sln);
        assert_eq!(sln.create().is_ok(), true);
    }
    #[test]
    fn create_csproj(){
        let csproj = CSProject::new("TEST","SSS");
        let sln = SolutionInfo::new("hello", PathBuf::new().join("target/Test"));
        let result = csproj.create(&sln);
        println!("{:?}",&result);
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn config(){
        let mut config = AppConfigInfo::new();
        println!("{:?}",config.refresh_game_version())
    }
}
