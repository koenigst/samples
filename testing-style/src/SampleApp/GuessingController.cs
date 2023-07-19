using Microsoft.AspNetCore.Mvc;

namespace SampleApp;

[ApiController]
public sealed class GuessingController : ControllerBase
{
    private const string GuessRouteName = "guessRoute";
    private readonly IGuessingService _service;

    public GuessingController(IGuessingService service)
    {
        _service = service;
    }

    [HttpPost("guesses")]
    public IActionResult Start()
    {
        var id = _service.Start();
        return CreatedAtRoute(GuessRouteName, new RouteValueDictionary { { "id", id }, }, null);
    }

    [HttpPost("guesses/{id}", Name = GuessRouteName)]
    public IActionResult Guess(Guid id, [FromBody] GuessDto dto)
    {
        var result = _service.TryGuess(id, dto.Value);

        return result is { } guess
            ? Ok(guess)
            : NotFound();
    }
}
