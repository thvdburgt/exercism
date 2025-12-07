static class AssemblyLine
{
  static private int CarsPerHourPerSpeedLevel = 221;

    public static double SuccessRate(int speed) =>
        speed switch
        {
            0 => 0.0,
            >= 1 and <= 4 => 1.0,
            >= 5 and <= 8 => 0.9,
            9 => 0.8,
            10 => 0.77,
            _ => throw new ArgumentOutOfRangeException($"Unexpected speed: {speed}")
        };

  public static double ProductionRatePerHour(int speed) =>
      CarsPerHourPerSpeedLevel * speed * SuccessRate(speed);

  public static int WorkingItemsPerMinute(int speed) =>
      (int) Math.Floor(ProductionRatePerHour(speed) / 60);
}
