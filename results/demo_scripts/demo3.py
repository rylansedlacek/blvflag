# Multiple errors - multiple fixes

# Run 1 error
def calculate_stats(values):
    mean = sum(values)/len(values
    squared_diffs = [(x - mean)**2 for x in values]
    variance = sum(squared_diffs)/len(values)
    print(f"Mean: {mean}, Variance: {variance}")

data = [10, 20, 30, 40, 50]
calculate_stats(data)

'''

'''


'''
# Run 2 - fix 1
# def calculate_stats(values):
#     mean = sum(values)/len(values)
#     squared_diffs = [(x - mean)**2 for x in values]
#     variance = sum(squared_diffs)/len(values)
#     print(f"Mean: {mean}, Variance: {variance}")
# 
# data = [10, 20, 30, 40, 50]
# calculate_stats(data)
'''

'''
# Run 3 - introduce new error
def calculate_stats(values):
mean = sum(values)/len(values)
squared_diffs = [(x - mean)**2 for x in values]
variance = sum(squared_diffs)/len(values)
print(f"Mean: {mean}, Variance: {variance}")

data = [10, 20, 30, 40, 50]
calculate_stats(data)
'''


'''
Run 4 - fix 2
def calculate_stats(values):
    mean = sum(values)/len(values)
    squared_diffs = [(x - mean)**2 for x in values]
    variance = sum(squared_diffs)/len(values)
    print(f"Mean: {mean}, Variance: {variance}")

data = [10, 20, 30, 40, 50]
calculate_stats(data)
'''

'''
# Run 5 - break one more time
def calculate_stats(values)
    mean = sum(values)/len(values)
    squared_diffs = [(x - mean)**2 for x in values]
    variance = sum(squared_diffs)/len(values)
    print(f"Mean: {mean}, Variance: {variance}")

data = [10, 20, 30, 40, 50]
calculate_stats(data)
'''