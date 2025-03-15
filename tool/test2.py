def infinite_recursion(n):
    print(n)
    return infinite_recursion(n - 1) 

infinite_recursion(5)
