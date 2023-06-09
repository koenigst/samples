using static SampleApp.GuessingResult;

namespace SampleApp;

public sealed class GuessingServiceTest
{
    #region state

    private readonly IGuessingService _service = new GuessingService(new Random());
    private Guid _id = Guid.NewGuid();
    private GuessingResult? _result;

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
        Given_Service_Started();
        When_Service_Guess(1);
        Then_Result_Is(Fail);
    }

    [Fact]
    public void Guess_TooManyGuesses_Defeat()
    {
        Given_Service_Started();
        When_Service_Guess(1);
        When_Service_Guess(2);
        When_Service_Guess(3);
        Then_Result_Is(Defeat);
    }

    #region given, when, then

    private void Given_Service_Started()
    {
        _id = _service.Start();
    }

    private void When_Service_Guess(int value)
    {
        _result = _service.TryGuess(_id, value);
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
