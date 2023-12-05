import csv

fields = ["apartment_id","meter_id"]

rows = []

count = 15863
for i in range(1,10):
    for j in range(0,4):
        for k in range(0,4):
            l = ["B0{}-{}0{}".format(i,j,k),"TNSWM-GCC{}".format(count)]
            rows.append(l)
            count += 1

for i in range(10,14):
    for j in range(0,4):
        for k in range(0,4):
            l = ["B{}-{}0{}".format(i,j,k),"TNSWM-GCC{}".format(count)]
            rows.append(l)
            count += 1

print(len(rows))

with open(r"apartment_meter.csv", 'w') as file:
    writer = csv.writer(file)

    writer.writerow(fields)

    writer.writerows(rows)