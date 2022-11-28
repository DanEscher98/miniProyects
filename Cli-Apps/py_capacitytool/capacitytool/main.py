# Filter Outlook calendar for Sprint Capacity
# Feel free to make any sugestion at:
# RevelesErnesto@JohnDeere.com

from datetime import date, datetime
import csv


def convertTimeDelta(duration):
    days, seconds = duration.days, duration.seconds
    hours = days * 24 + seconds // 3600
    minutes = (seconds % 3600) // 60
    seconds = seconds % 60
    return hours, minutes, seconds


def main():
    # Get user input, start and end date
    print("\nEnter START date.")
    yearStart = int(input("Year (YYYY): "))
    monthStart = int(input("Month: "))
    dayStart = int(input("Day: "))
    print("\n-----------------")
    print("\nEnter END date.")
    yearEnd = int(input("Year (YYYY): "))
    monthEnd = int(input("Month:"))
    dayEnd = int(input("Day: "))

    # Initialize values
    startDate = date(yearStart, monthStart, dayStart)
    endDate = date(yearEnd, monthEnd, dayEnd)
    categoryDict = {}


    # Iterate rows in csv accumulating object_event duration by category and if the are in time sample
    with open("outlook_calendar.csv") as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=",")
        for row in csv_reader:
            eventStartDate = row[1].split("/")
            eventStartTime = row[2].split(":")
            eventEndDate = row[3].split("/")
            eventEndTime = row[4].split(":")

            eventStartDateObject = date(
                int(eventStartDate[2]), int(eventStartDate[0]), int(eventStartDate[1])
            )
            eventEndDateObject = date(
                int(eventEndDate[2]), int(eventEndDate[0]), int(eventEndDate[1])
            )
            eventStartTimeObject = datetime.strptime(str(row[2]), "%I:%M:%S %p")
            eventEndTimeObject = datetime.strptime(str(row[4]), "%I:%M:%S %p")

            eventDuration = eventEndTimeObject - eventStartTimeObject
            category = row[5]

            # verify that
            if (str(row[5]) != "NA") and (str(row[5]) != ""):
                # check that event is in time sample
                if (startDate <= eventStartDateObject <= endDate) or (
                    startDate <= eventEndDateObject <= endDate
                ):
                    # get object_event duration
                    eventDuration = eventEndTimeObject - eventStartTimeObject
                    # classify object_events in dict
                    if category in categoryDict:
                        categoryDict[category] = categoryDict[category] + eventDuration
                    else:
                        categoryDict[category] = eventDuration

    sortedCategoryDict = sorted(categoryDict.keys(), key=lambda x: x.lower())
    print("\nTotal times:")

    # print results
    for key in sortedCategoryDict:
        print(" - " + str(key) + ": " + str(categoryDict[key]))


if __name__ == "__main__":
    main()
