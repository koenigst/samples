# Solution Structure

This sample shows off a style of structuring C# solutions.
This style is based on the structure of repositories such as [.Net runtime][dotnetRuntime] and [Entity Framework Core][efCore].

## Directory Structure

* The solution file is at the root of the directory structure.
This might change if the directory structure contains more than one solution file.

* All implementation projects are under the `src` directory.
Example: [`src/Implementation/Implementation.csproj`][implementationProject]

* All test projects are under the `test` directory.
Example: [`test/Test/Test.csproj`][testProject]

* The directory structure should not be represented in the solution explorer as it is beneficial to have the implementation and test projects close to each other.

## Configuration Files

[SDK-style][sdkStyle] project files have built-in support for configuration sharing.
The following types of files can be used:

* [`.editorconfig`][editorconfig] contains the formatting and code analysis conventions.

* [`Directory.Build.props`][buildPropsTargets] contains all common configuration defaults.

* [`Directory.Build.targets`][buildPropsTargets] contains all the common derived configuration.
It also contains any shared build targets.

* [`Directory.Packages.props`][packagesProps] contains all the versions of the package references such that these are consistent across all projects.

If projects in a sub directory (such as `test`) have additional shared configuration those files can be merged together (see [`test/Directory.Build.props`][testBuildProps] for a sample).
Merging multiple levels of shared configuration files must be [done manually][sharedMerging] as the projects only include the file closest to their location.

[dotnetRuntime]: https://github.com/dotnet/runtime
[efCore]: https://github.com/dotnet/efcore
[implementationProject]: src/Implementation/Implementation.csproj
[testProject]: test/Test/Test.csproj
[sdkStyle]: https://learn.microsoft.com/en-us/dotnet/core/project-sdk/overview
[editorconfig]: https://learn.microsoft.com/en-us/dotnet/fundamentals/code-analysis/configuration-files#editorconfig
[buildPropsTargets]: https://learn.microsoft.com/en-us/visualstudio/msbuild/customize-by-directory#directorybuildprops-and-directorybuildtargets
[packagesProps]: https://learn.microsoft.com/en-us/nuget/consume-packages/central-package-management
[testBuildProps]: test/Directory.Build.props
[sharedMerging]: https://learn.microsoft.com/en-us/visualstudio/msbuild/customize-by-directory#use-case-multi-level-merging
