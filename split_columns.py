import numpy as np

# File path to the text file
file_path = "one.txt"

# Load the data from the text file
data = np.loadtxt(file_path, dtype=int)

# Separate the columns into two lists
column1 = data[:, 0].tolist()
column2 = data[:, 1].tolist()

# Print the lists
print("Column 1:", column1)
print("Column 2:", column2)