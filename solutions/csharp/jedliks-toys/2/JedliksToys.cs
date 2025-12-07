class RemoteControlCar
{
    private uint _metersDriven = 0;
    private uint _batteryPercentage = 100;
    
    public static RemoteControlCar Buy() => new();

    public string DistanceDisplay() => $"Driven {_metersDriven} meters";

    public string BatteryDisplay() =>
        _batteryPercentage > 0 ? $"Battery at {_batteryPercentage}%" : "Battery empty";

    public void Drive()
    {
        if (_batteryPercentage > 0) {
            _metersDriven += 20;
            _batteryPercentage -= 1;
        }
    }
}
