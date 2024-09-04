# Testing Style

This sample shows off my style of writing [xUnit.net][xunit] based unit tests.
Most of the principles can also be applied to other testing frameworks.

An explanation of my testing style can be found in [another repository][knowhowTestStructure].

[knowhowTestStructure]: https://github.com/koenigst/knowhow/blob/main/testing/structure.md

## Samples

* [Component test](test/SampleApp.Test/GuessingServiceTest.cs)

* [WebAPI test](test/SampleApp.Test/GuessingApiTest.cs)

## Test bootstrapping

The [injected tests][injectedTests] library is used extensively for bootstrapping tests using dependency injection.
It makes common tasks for dependency configuration simple.
It also provides a common interface over multiple DI strategies which enables better test code reuse.

[xunit]: https://github.com/xunit/xunit
[injectedTests]: https://github.com/InjectedTests/InjectedTests
