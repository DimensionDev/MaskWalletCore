using System;
using System.IO;
using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;
using CommunityToolkit.Mvvm.DependencyInjection;
using Dimension.MaskCore.Common;
using Dimension.MaskCore.Data.Repository;
using Dimension.MaskCore.UI.Shell;
using Microsoft.Extensions.DependencyInjection;
using Realms;

namespace Dimension.MaskCore;

internal class App : Application
{
    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
        EnsureWorkingDirectory();
        Ioc.Default.ConfigureServices(ConfigureServices());
    }

    private void EnsureWorkingDirectory()
    {
        if (!Directory.Exists(Consts.ConfigDirectory))
        {
            Directory.CreateDirectory(Consts.ConfigDirectory);
        }
    }

    private static IServiceProvider ConfigureServices()
    {
        var services = new ServiceCollection();
        services.AddSingleton<PersonaRepository>();
        services.AddSingleton<WalletRepository>();
        services.AddSingleton<Realm>(_ =>
            Realm.GetInstance(new RealmConfiguration(Path.Combine(Consts.ConfigDirectory, ".realm"))
                { ShouldDeleteIfMigrationNeeded = true }));
        return services.BuildServiceProvider();
    }

    public override void OnFrameworkInitializationCompleted()
    {
        switch (ApplicationLifetime)
        {
            case IClassicDesktopStyleApplicationLifetime classicDesktopStyleApplicationLifetime:
                classicDesktopStyleApplicationLifetime.MainWindow = new RootWindow();
                classicDesktopStyleApplicationLifetime.Exit += (_, _) =>
                {
                    Ioc.Default.GetRequiredService<Realm>().Dispose();
                };
                break;
            case ISingleViewApplicationLifetime singleViewApplicationLifetime:
                singleViewApplicationLifetime.MainView = new RootShell();
                break;
        }

        base.OnFrameworkInitializationCompleted();
    }
}