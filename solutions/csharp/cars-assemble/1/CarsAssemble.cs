static class AssemblyLine
{
  static private int CARS_PER_HOUR_PER_SPEED_LEVEL = 221;

  public static double SuccessRate(int speed) {
    if (speed == 0) {
      return 0;
    } else if (speed >= 1 && speed <= 4) {
      return 1.0;
    } else if (speed >= 5 && speed <= 8) {
      return 0.9;
    } else if (speed == 9) {
      return 0.8;
    } else if (speed == 10) {
      return 0.77;
    } else {
      throw new ArgumentOutOfRangeException($"Unexpected speed: {speed}");
    }
  }

  public static double ProductionRatePerHour(int speed) => CARS_PER_HOUR_PER_SPEED_LEVEL * speed * SuccessRate(speed);

  public static int WorkingItemsPerMinute(int speed) => (int)Math.Floor(ProductionRatePerHour(speed) / 60);
}
