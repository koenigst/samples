namespace Wpf.Interaction;

public interface IAsyncCommand
{
    event EventHandler? CanExecuteChanged;
    bool CanExecute(object? parameter);
    ValueTask ExecuteAsync(object? parameter);
}
