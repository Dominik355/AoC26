from utils.decorators import solution
import re

# it can actually be done with regexes ... hell, I spent time on my rust solution with splitting
@solution
def part_1(inpt: list[str]) -> int:
    return common_solution(inpt, r"^(\d+)\1$")

@solution
def part_2(inpt: list[str]) -> int:
    return common_solution(inpt, r"^(\d+)\1+$")

def common_solution(inpt: list[str], regex) -> int:
    pattern = re.compile(regex)
    ans = 0
    for i in inpt[0].strip().split(","):
        x, y = i.split("-")
        for j in range(int(x), int(y) + 1):
            if pattern.findall(str(j)): ans += j

    return ans
