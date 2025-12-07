public static class PhoneNumber
{
    public static (bool IsNewYork, bool IsFake, string LocalNumber) Analyze(string phoneNumber)
    {
        var groups = phoneNumber.Split('-');
        if (groups.Length != 3)
        {
            throw new ArgumentException("Invalid phone number format");
        }
        var IsNewYork = groups[0] == "212";
        var IsFake = groups[1].StartsWith("555");
        var LocalNumber = groups[2];

        return (IsNewYork, IsFake, LocalNumber);
    }

    public static bool IsFake((bool IsNewYork, bool IsFake, string LocalNumber) phoneNumberInfo) =>
        phoneNumberInfo.IsFake;
}
