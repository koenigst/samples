using System.Net;
using System.Net.Http.Json;
using static SampleApp.GuessingResult;

namespace SampleApp;

public sealed class GuessingApiTest : IAsyncLifetime
{
    #region state

    private readonly WebApplicationBootstrapper<SampleAppHost> _bootstrapper = new();

    private Guid _id = Guid.NewGuid();
    private HttpStatusCode? _statusCode;
    private Uri? _locationHeader;
    private GuessingResult? _guessResult;

    private HttpClient Client => _bootstrapper.Client;
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
    public async Task Start_NotStarted_CreatedAtId()
    {
        Given_Service_ReturnsId();
        await When_Client_PostStart();
        Then_Result_Created();
        Then_Result_HasIdInLocation();
    }

    [Fact]
    public async Task Guess_NotStarted_NotFound()
    {
        await When_Client_PostGuess(1);
        Then_Result_IsNotFound();
    }

    [Fact]
    public async Task Guess_WrongGuess_Fail()
    {
        Given_Random_SeededWith(0);
        Given_Service_Started();
        await When_Client_PostGuess(1);
        Then_Result_Is(Fail);
    }

    [Fact]
    public async Task Guess_CorrectGuess_Victory()
    {
        Given_Random_SeededWith(5);
        Given_Service_Started();
        await When_Client_PostGuess(3);
        Then_Result_Is(Victory);
    }

    #region given, when, then

    private void Given_Service_ReturnsId()
    {
        _bootstrapper.ConfigureServices(s => s.Replace(ServiceDescriptor.Singleton(Helper_CreateMockGuessingService)));
    }

    private void Given_Random_SeededWith(int seed)
    {
        _bootstrapper.ConfigureServices(s => s.Replace(ServiceDescriptor.Singleton(_ => new Random(seed))));
    }

    private void Given_Service_Started()
    {
        _id = Service.Start();
    }

    private async Task When_Client_PostGuess(int value)
    {
        var builder = new UriBuilder { Path = $"guesses/{_id}", };
        var dto = new GuessDto { Value = value, };

        using var response = await Client.PostAsJsonAsync(builder.Uri, dto);

        _statusCode = response.StatusCode;
        if (response.IsSuccessStatusCode)
        {
            _guessResult = await response.Content.ReadFromJsonAsync<GuessingResult?>();
        }
    }

    private async Task When_Client_PostStart()
    {
        var builder = new UriBuilder { Path = "guesses", };

        using var response = await Client.PostAsync(builder.Uri, null);

        _statusCode = response.StatusCode;
        _locationHeader = response.Headers.Location;
    }

    private void Then_Result_Created()
    {
        Assert.Equal(HttpStatusCode.Created, _statusCode);
    }

    private void Then_Result_IsNotFound()
    {
        Assert.Equal(HttpStatusCode.NotFound, _statusCode);
    }

    private void Then_Result_HasIdInLocation()
    {
        Assert.NotNull(_locationHeader);
        Assert.EndsWith(_id.ToString(), _locationHeader.LocalPath);
    }

    private void Then_Result_Is(GuessingResult expected)
    {
        Assert.Equal(expected, _guessResult);
    }

    private IGuessingService Helper_CreateMockGuessingService(IServiceProvider services)
    {
        var mock = new Mock<IGuessingService>();
        mock.Setup(m => m.Start()).Returns(() => _id);
        return mock.Object;
    }

    #endregion
}
