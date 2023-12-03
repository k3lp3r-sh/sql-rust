import csv

# Define a mapping for category replacement
category_mapping = {'1': 'family', '2': 'bachelors', '3': 'single_resident'}

# Read the existing CSV file
with open('data.csv', 'r') as csvfile:
    reader = csv.DictReader(csvfile)
    rows = list(reader)

# Replace values in the "Category" column
for row in rows:
    row['Category'] = category_mapping.get(row['Category'], row['Category'])

# Write the updated data to a new CSV file
new_file_path = 'data.csv'
with open(new_file_path, 'w', newline='') as csvfile:
    fieldnames = ['House Number', 'Category', 'Occupied', 'Water Consumption']
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)

    # Write the CSV header
    writer.writeheader()

    # Write the updated rows
    writer.writerows(rows)

print(f"Updated CSV file generated successfully: {new_file_path}")

