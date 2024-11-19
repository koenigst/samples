namespace Wpf.Interaction;

public interface ICommandContext
{
    event EventHandler? CanExecuteChanged;
    bool CanExecute(object command);
    ICommandExecution Begin(object command);
}

public interface ICommandExecution : IDisposable
{
    void ReportFailure(object command, Exception exception);
}
