# WPF

## Dependency Injection

This sample shows how a `Microsoft.Extensions.Hosting` service provider can be integrated into a WPF application.
It uses a [derived `Application`](src/Wpf.Hosting/HostingApplication.cs) to manage the `IHost`.
A [`WpfLifetime`](src/Wpf.Hosting/WpfLifetime.cs) is used to synchronise the host lifecycle with the WPF lifecycle.
