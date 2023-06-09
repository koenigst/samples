using static SampleApp.GuessingResult;

namespace SampleApp;

public sealed class GuessingServiceTest : IAsyncLifetime
{
    #region state

    private readonly ServiceProviderBootstrapper _bootstrapper = new ServiceProviderBootstrapper()
        .ConfigureServices(s => s.AddApp());

    private Guid _id = Guid.NewGuid();
    private GuessingResult? _result;

    private IGuessingService Service => _bootstrapper.GetRequiredService<IGuessingService>();

    #endregion

    #region lifecycle

    public Task InitializeAsync()
    {
        return Task.CompletedTask;
    }

    public async Task DisposeAsync()
    {
        await _bootstrapper.DisposeAsync();
    }

    #endregion

    [Fact]
    public void Guess_NotStarted_Null()
    {
        When_Service_Guess(1);
        Then_Result_IsNull();
    }

    [Fact]
    public void Guess_WrongGuess_Fail()
    {
        Given_Random_SeededWith(0);
        Given_Service_Started();
        When_Service_Guess(1);
        Then_Result_Is(Fail);
    }

    [Fact]
    public void Guess_TooManyGuesses_Defeat()
    {
        Given_Random_SeededWith(0);
        Given_Service_Started();
        When_Service_Guess(1);
        When_Service_Guess(2);
        When_Service_Guess(3);
        Then_Result_Is(Defeat);
    }

    [Fact]
    public void Guess_CorrectGuess_Victory()
    {
        Given_Random_SeededWith(5);
        Given_Service_Started();
        When_Service_Guess(3);
        Then_Result_Is(Victory);
    }

    #region given, when, then

    private void Given_Random_SeededWith(int seed)
    {
        _bootstrapper.ConfigureServices(s => s.Replace(ServiceDescriptor.Singleton(_ => new Random(seed))));
    }

    private void Given_Service_Started()
    {
        _id = Service.Start();
    }

    private void When_Service_Guess(int value)
    {
        _result = Service.TryGuess(_id, value);
    }

    private void Then_Result_IsNull()
    {
        Assert.Null(_result);
    }

    private void Then_Result_Is(GuessingResult expected)
    {
        Assert.Equal(expected, _result);
    }

    #endregion
}
