# Testing Style

This sample shows off my style of writing [xUnit.net][xunit] based unit tests.
Most of the principles can also be applied to other testing frameworks.

In a lot of code samples some part of a name is in angle brackets `Prefix<ToBeReplaced>Suffix`.
The part in angle brackets is meant to be replaced while the other parts should be used verbatim.

## Naming

* Test classes are usually named after the unit under test with the suffix `Test`.
Examples of names for units under test are:

  * Simple input-output tests for classes: `<ClassUnderTest>Test`

  * Component behaviour tests: `<ComponentUnderTest>Test` ([sample][componentSample])

  * Web API tests: `<ApiAreaUnderTest>ApiTest` ([sample][apiSample])

  * End-to-end application tests: `<ApplicationAreaUnderTest>ApplicationTest`

  * Base classes for test classes: `<CommonTestClassDescriptor>TestBase`

* Test methods are named in a three part scheme `<Feature>_<Scenario>_<Result>`.

  * `<Feature>` describes which part of the behaviour of the unit under test is tested.
  This can also be a bug or requirement identifier if the test case is linked to one of those.

  * `<Scenario>` describes what happens during the test.
  This description should not be comprehensive but highlight what this test case does differently compared to the other test cases.

  * `<Result>` describes what outcome is to be expected again highlighting the difference to other test cases.

## Class Structure

* The body of a test method only consists of calls to `Given`, `When`, `Then` implementation methods ([sample][componentSample]).
This makes the test method read almost like a [gherkin][gherkin] description of the test case which helps with understanding what the test case does in detail.
It also promotes code reuse between the test cases and reduces the amount of lines of code that have to be changed during refactorings.
The naming of the implementation methods should be as follows:

  * `Given_<Target>_<InitialState>`

  * `When_<Target>_<PerformsAction>`

  * `Then_<Target>_<ExpectedState>`

  * `Helper_<MethodDescription>` for any methods that are not directly used in test methods.

* The state of the test should be stored in fields or properties on the test class.
The test method must not contain any local variables.
Local variables would massively reduce the readability of the test method.
Using fields and properties also helps with code reuse.

* All code in a test class except the test methods should be in a `#region`.
This allows the reader to focus on the part of the test class that is most interesting to them which is usually the test methods.
It also keeps similar members close to each other making code navigation simpler.
The recommended regions are:

  * `state` containing all the constants, fields and properties.

  * `lifecycle` containing the constructor and lifecycle methods such as `Dispose`, `InitializeAsync`, etc.

  * `implementation` containing all the `Given`, `When`, `Then`, `Helper` methods in this order and any nested types.

## Bootstrapping

I like using the applications dependency injection (DI) facilities in all tests except the simplest input-output tests.
This has the advantage that faking hard dependencies is relatively easy because most DI-frameworks allow for simple replacement of services.
It also simplifies writing reusable fakes.

The [injected tests][injectedTests] library supports writing these kinds of tests.
It makes common tasks for dependency configuration simple.
And it provides a common interface over multiple DI strategies which enables better test code reuse.

[xunit]: https://github.com/xunit/xunit
[componentSample]: test/SampleApp.Test/GuessingServiceTest.cs
[apiSample]: test/SampleApp.Test/GuessingApiTest.cs
[gherkin]: https://cucumber.io/docs/gherkin/
[injectedTests]: https://github.com/InjectedTests/InjectedTests
