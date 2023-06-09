namespace SampleApp;

public sealed class GuessingServiceTest
{
    [Fact]
    public void Guess_NotStarted_Null()
    {
        var service = new GuessingService(new Random());
        var result = service.TryGuess(Guid.NewGuid(), 1);
        Assert.Null(result);
    }

    [Fact]
    public void Guess_WrongGuess_Fail()
    {
        var service = new GuessingService(new Random());
        var id = service.Start();
        var result = service.TryGuess(id, 1);
        Assert.Equal(GuessingResult.Fail, result);
    }

    [Fact]
    public void Guess_TooManyGuesses_Defeat()
    {
        var service = new GuessingService(new Random());
        var id = service.Start();
        service.TryGuess(id, 1);
        service.TryGuess(id, 2);
        var result = service.TryGuess(id, 3);
        Assert.Equal(GuessingResult.Defeat, result);
    }
}
