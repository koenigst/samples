using System.Globalization;
using System.Windows.Data;

namespace Wpf.Sample;

public sealed class IsNotNullConverter : IValueConverter
{
    public object Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (targetType != typeof(bool))
        {
            throw new NotSupportedException($"Conversion to {targetType} is not supported.");
        }

        return value is not null;
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        throw new NotSupportedException();
    }
}
