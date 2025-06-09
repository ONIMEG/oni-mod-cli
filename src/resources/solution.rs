pub const CONTENT: &str = r#"
Microsoft Visual Studio Solution File, Format Version 12.00
# Visual Studio Version 17
VisualStudioVersion = 17.2.32616.157
MinimumVisualStudioVersion = 10.0.40219.1
Project("{2150E333-8FDC-42A3-9474-1A3956D46DE8}") = "Build", "Build", "{DD48872E-7B35-44EF-A5D3-6F1472F322CE}"
	ProjectSection(SolutionItems) = preProject
		Directory.Build.props = Directory.Build.props
		Directory.Build.props.default = Directory.Build.props.default
		Directory.Build.targets = Directory.Build.targets
	EndProjectSection
EndProject
Global
	GlobalSection(SolutionConfigurationPlatforms) = preSolution
		Debug|Mergedown = Debug|Mergedown
		Debug|Vanilla = Debug|Vanilla
		Release|Mergedown = Release|Mergedown
		Release|Vanilla = Release|Vanilla
	EndGlobalSection
	GlobalSection(ProjectConfigurationPlatforms) = postSolution
	EndGlobalSection
	GlobalSection(SolutionProperties) = preSolution
		HideSolutionNode = FALSE
	EndGlobalSection
	GlobalSection(ExtensibilityGlobals) = postSolution
		SolutionGuid = {$[guid]}
	EndGlobalSection
EndGlobal
"#;