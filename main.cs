using System;
using System.IO; // Not strictly needed for this Console version, but good practice
using System.Text;
using System.Threading;
using System.Diagnostics; // For Stopwatch

class Program
{
    static void Main(string[] args)
    {
        // 1. Set Console Window Title (Customizable Window Function)
        Console.Title = "Rust Ultimate Overlord C# v0.70b";

        string[] asciiArt = {
            @"  _____           _     _   _______          _       _ ",
            @" |  __ \         | |   | | |__   |        | |     | |",
            @" | |) |   _ | |  | |    | | ___   ___ | | ___ | |",
            @" |  _  / | | / __| __| | |    | |/ _ \ / _ | |/ _ | |",
            @" | | \ \ || _ \ |_  | || | () | () | | () | |",
            @" ||  __,|/_| |||_/ _/|_|__/|_|",
            @"                   Rust Ultimate Overlord v0.70b        "
        };

        // Tuple array for colors (R, G, B)
        (int R, int G, int B)[] colors = {
            (255, 0, 0),
            (255, 127, 0),
            (255, 255, 0),
            (0, 255, 0),
            (0, 0, 255),
            (75, 0, 130),
            (148, 0, 211)
        };

        // Ensure console supports ANSI escape codes (modern Windows Terminal does)
        // For older cmd.exe, this might not render colors correctly.
        // No direct equivalent to force it beyond what the terminal supports.
        // However, Windows 10+ cmd.exe has improved ANSI support.

        for (int i = 0; i < asciiArt.Length; i++)
        {
            var color = colors[i % colors.Length];
            // ANSI escape code for 24-bit color
            Console.Write($"\x1b[38;2;{color.R};{color.G};{color.B}m{asciiArt[i]}");
            Console.WriteLine(); // Newline
        }
        Console.WriteLine("\x1b[0m"); // Reset color
        Console.Out.Flush(); // Equivalent to io::stdout().flush().unwrap();

        Thread.Sleep(TimeSpan.FromSeconds(25));

        // Clear screen: \x1b[2J clears entire screen, \x1b[H moves cursor to home
        // Console.Clear() is the simpler C# way
        Console.Clear();
        // Console.Write("\x1b[2J\x1b[H"); // Alternative using ANSI
        // Console.Out.Flush();

        int barLength = 50;
        Stopwatch stopwatch = Stopwatch.StartNew();
        TimeSpan totalDuration = TimeSpan.FromSeconds(45);

        while (true)
        {
            TimeSpan elapsed = stopwatch.Elapsed;
            double progress;

            if (elapsed >= totalDuration)
            {
                progress = 100.0;
            }
            else
            {
                progress = (elapsed.TotalSeconds / totalDuration.TotalSeconds) * 100.0;
            }

            int filledChars = (int)(progress / 100.0 * barLength);
            StringBuilder bar = new StringBuilder();

            for (int i = 0; i < barLength; i++)
            {
                if (i < filledChars)
                {
                    // Ensure barLength - 1 is not zero if barLength is 1, though unlikely for 50
                    double ratio = (barLength > 1) ? (double)i / (barLength - 1) : 1.0;
                    byte r = (byte)(255.0 * (1.0 - ratio));
                    byte g = (byte)(255.0 * ratio);
                    byte b = 0;
                    bar.Append($"\x1b[38;2;{r};{g};{b}m█");
                }
                else
                {
                    bar.Append("\x1b[0m "); // Reset color before space
                }
            }

            bar.Append($"\x1b[0m {progress:F1}%"); // Reset color and add percentage
            Console.Write($"\r{bar.ToString()}"); // \r to return to start of line
            Console.Out.Flush();

            if (progress >= 100.0)
            {
                break;
            }

            Thread.Sleep(TimeSpan.FromMilliseconds(50));
        }
        stopwatch.Stop();

        Console.WriteLine(); // Move to next line after progress bar finishes
        Console.Clear();
        // Console.Write("\x1b[2J\x1b[H");
        // Console.Out.Flush();

        // ANSI escape code for green text
        Console.WriteLine("\x1b[32mActivation successful! Closing console in 25 seconds...\x1b[0m");
        Console.Out.Flush();

        Thread.Sleep(TimeSpan.FromSeconds(25));
    }
}
