def get_modules():
    nb_days = 12
    for i in range(1, nb_days + 1):
        try:
            yield __import__("day%d" % i)
        except ModuleNotFoundError:
            pass


def get_solutions(days):
    print("Actual solutions")
    for day in days:
        print("-", day.__name__)
        day.get_solutions()
    print()


if __name__ == "__main__":
    get_solutions(get_modules())