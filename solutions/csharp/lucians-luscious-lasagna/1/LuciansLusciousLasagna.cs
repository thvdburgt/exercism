class Lasagna
{
  public int ExpectedMinutesInOven() => 40;
  public int RemainingMinutesInOven(int minutesInOven) => 40 - minutesInOven;
  public int PreparationTimeInMinutes(int numberOfLayers) => numberOfLayers * 2;
  public int ElapsedTimeInMinutes(int numberOfLayers, int minutesInOven) =>
      PreparationTimeInMinutes(numberOfLayers) + minutesInOven;
}
