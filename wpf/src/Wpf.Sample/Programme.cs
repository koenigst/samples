using System.Windows;

namespace Wpf.Sample;

internal sealed class Programme
{
    [STAThread]
    public static void Main()
    {
        var splashScreen = new SplashScreen("splash.png");
        splashScreen.Show(true);

        using var app = new App();
        app.InitializeComponent();
        app.Run();
    }
}
