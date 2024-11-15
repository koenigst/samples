namespace Wpf.Interaction;

public interface ICommandContext
{
    event EventHandler? CanExecuteChanged;
    bool CanExecute(object command);
    IDisposable Begin(object command);
}
