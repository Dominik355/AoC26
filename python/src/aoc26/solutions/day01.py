from dis import Instruction

from duplicity.dup_time import prevtime

from aoc26.utils.input import get_input
from aoc26.utils.decorators import solution


@solution
def part_1_modulo(inpt: list[str]):
    position = 50
    zero_counter = 0
    for line in inpt:
        clicks = int(line[1:])

        if line[0] == 'L':
            position = (position - clicks) % 100
        elif line[0] == 'R':
            position = (position + clicks) % 100

        if position == 0:
            zero_counter += 1


    print("Code:", zero_counter)

def part_1_sequence(inpt: list[str]):
    position = 50
    zero_counter = 0
    moves = {'L': -1, 'R': 1}
    for line in inpt:
        move = moves[line[0]]
        for clicks in range(0, int(line[1:])):

        if line[0] == 'L':
            position = (position - clicks) % 100
        elif line[0] == 'R':
            position = (position + clicks) % 100

        if position == 0:
            zero_counter += 1


    print("Code:", zero_counter)

# + number of times any click causes the dial to point at 0
@solution
def part_2(inpt: list[str]):
    position = 50
    zero_counter = 0
    signs = {'L': -1, 'R': 1}
    for line in inpt:
        print("")
        clicks = int(line[1:])
        sign = signs[line[0]]

        prev = position
        position += clicks * sign
        print("prev:", prev)
        print("move:", clicks * sign)
        print("position:", position)


        prev_lo = prev // 100
        curr_lo = position // 100
        prev_hi = (prev - 1) // 100
        curr_hi = (position - 1) // 100
        print("prev_lo:", prev_lo)
        print("curr_lo:", curr_lo)
        print("prev_hi:", prev_hi)
        print("curr_hi:", curr_hi)

        zero_counter += abs(prev_lo - curr_lo) + abs(prev_hi - curr_hi)
    print(zero_counter / 2)

# inpt = get_input(1).splitlines()
inpt = [
    "L68",
    "L30",
    "R48",
    "L5",
    "R60",
    "L55",
    "L1",
    "L99",
    "R14",
    "L82",
]
# part_1_modulo(inpt[::])
part_2(inpt)