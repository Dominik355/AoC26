import sys
import importlib
from utils.input import get_input

def main():
    day = sys.argv[1]
    part = sys.argv[2]

    module = importlib.import_module(f"solutions.day{day}")
    func = getattr(module, f"part_{part}")

    data = get_input(int(day)).splitlines()

    result = func(data)

    print(f"Result: {result}")

if __name__ == "__main__":
    main()