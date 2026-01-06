import time

def timeit(func, *args, **kwargs):
    start = time.perf_counter()
    result = func(*args, **kwargs)
    end = time.perf_counter() - start
    print(f"{func.__name__} took {end:0.6f} seconds")
    return result

def stepcount(n, steps):
    if n == 0:
        return 1

    if n < 0:
        return 0

    return sum(stepcount(n - s, steps) for s in steps)

def memsteps(n, steps):
    return memsteps_cache(n, steps, {})

def memsteps_cache(n, steps, cache):
    if n == 0:
        return 1

    if n < 0:
        return 0

    if n in cache:
        return cache[n]

    total = sum(memsteps_cache(n - s, steps, cache) for s in steps)

    cache[n] = total
    return total