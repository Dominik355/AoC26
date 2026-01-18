from utils.decorators import solution

@solution
def part_1(inpt: list[str]):
    position, zero_counter = 50, 0
    for line in inpt:
        # python's modulo with negative behaves as rem_euclid in rust
        position = (position + (-1 if line[0] == 'L' else 1) * int(line[1::])) % 100
        zero_counter += position == 0

    return zero_counter

@solution
def part_2(inpt: list[str]):
    total, pos = 0, 50

    for line in inpt:
        clicks = int(line[1::])
        if line[0] == 'L':
            total += clicks // 100 # full rounds
            if pos != 0 and clicks % 100 >= pos:
                total += 1
            pos = (pos - clicks) % 100
        else:
            pos += clicks
            total += pos // 100
            pos %= 100

    return total