import csv
import random

fields = ["staff_id", "Staff_name", "Staff_contact"]


rows = []
date = 14
count = 15863
count_ = 2071
for i in range(1,10):
    date += 1

    month = random.randint(8,11)
    date_ = random.randint(1,30)
    for j in range(0,4):
        for k in range(0,4):
            occupants = random.randint(1,8)
            
            consumption = ((1 + round(random.random(), 5)) * 110) * occupants * 30

            amount = consumption * 9 / 1000

            flag = True
            n = random.randint(1,10)

            if n<=2:
                flag = False
            

            l = ["B0{}-{}0{}".format(i,j,k), occupants ,consumption, "SGCCID-{}".format(count_), amount, "2023-11-{}".format(random.randint(1,5))]
            rows.append(l)
            count += 1
            count_ += 1
for i in range(1,10):
    date += 1

    month = random.randint(8,11)
    date_ = random.randint(1,30)
    for j in range(0,4):
        for k in range(0,4):
            occupants = random.randint(1,8)
            
            consumption = ((1 + round(random.random(), 5)) * 110) * occupants * 30

            amount = consumption * 9 / 1000

            flag = True
            n = random.randint(1,10)

            if n<=2:
                flag = False
            

            l = ["B{}-{}0{}".format(i,j,k), occupants ,consumption, "SGCCID-{}".format(count_), amount, "2023-11-{}".format(random.randint(1,5))]
            rows.append(l)
            count += 1
            count_ += 1


print(len(rows))

with open(r"staffData.csv", 'w') as file:
    writer = csv.writer(file)

    writer.writerow(fields)

    writer.writerows(rows)