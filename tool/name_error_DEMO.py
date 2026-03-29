def calculate_total(items):
    total = 0
    for item in items:
        total += price  # price is undefined
    return total

items = [10, 20, 30]
print(calculate_total(items))