
import csv
import random

fields = ["meter_id","active","issue_date","last_update"]

rows = []
date = 14
count = 15863
for i in range(1,14):
    date += 1

    month = random.randint(8,11)
    date_ = random.randint(1,30)
    for j in range(0,4):
        for k in range(0,4):
            flag = True
            n = random.randint(1,10)

            if n<=2:
                flag = False

            l = ["TNSWM-GCC{}".format(count),flag,"2021-05-{}".format(date),"2023-{}-{}".format(month, date_)]
            rows.append(l)
            count += 1


print(len(rows))

with open(r"meter_data.csv", 'w') as file:
    writer = csv.writer(file)

    writer.writerow(fields)

    writer.writerows(rows)