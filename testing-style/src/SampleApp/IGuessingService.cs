namespace SampleApp;

public interface IGuessingService
{
    Guid Start();
    GuessingResult? TryGuess(Guid id, int value);
}
