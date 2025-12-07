static class LogLine
{
    private static string[] LOG_LEVELS = ["INFO", "WARNING", "ERROR"];

    
    public static string? Message(string logLine)
    {
        foreach (var level in LOG_LEVELS) {
            var prefix = $"[{level}]: ";
            if (logLine.StartsWith(prefix)) {
                return logLine.Substring(prefix.Length).Trim();
            }
        }
        
        return null;
    }

    public static string? LogLevel(string logLine)
    {
        foreach (var level in LOG_LEVELS) {
            var prefix = $"[{level}]: ";
            if (logLine.StartsWith(prefix)) {
                return level.ToLower();
            }
        }
        
        return null;
    }

    public static string Reformat(string logLine)
    {
        var message = Message(logLine);
        if (message is null) {
            return null;
        }
        var level = LogLevel(logLine);
        if (level is null) {
            return null;
        }
        return $"{message} ({level})";
    }
}
