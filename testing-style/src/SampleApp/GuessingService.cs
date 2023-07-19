using System.Collections.Concurrent;

namespace SampleApp;

internal sealed class GuessingService : IGuessingService
{
    private const int MaxValue = 10;
    private const int MaxGuesses = 3;

    private readonly ConcurrentDictionary<Guid, GuessingState> _guesses = new();

    private readonly Random _random;

    public GuessingService(Random random)
    {
        _random = random;
    }

    public Guid Start()
    {
        while (true)
        {
            var id = Guid.NewGuid();
            var value = _random.Next(MaxValue);
            var state = new GuessingState(value, MaxGuesses);
            if (_guesses.TryAdd(id, state))
            {
                return id;
            }
        }
    }

    public GuessingResult? TryGuess(Guid id, int value)
    {
        if (!_guesses.TryGetValue(id, out var state))
        {
            return null;
        }

        if (state.Value == value)
        {
            _guesses.TryRemove(id, out _);
            return GuessingResult.Victory;
        }

        var remaining = state.Remaining - 1;

        if (remaining == 0)
        {
            _guesses.TryRemove(id, out _);
            return GuessingResult.Defeat;
        }

        var newState = state with { Remaining = remaining };

        return _guesses.TryUpdate(id, newState, state)
            ? GuessingResult.Fail
            : null;
    }
}
