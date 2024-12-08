with open("src/inputs/day07.txt") as f:
    equations = [
        (int(total), list(map(int, operands.strip().split())))
        for total, operands in (line.split(":") for line in f.read().strip().split("\n"))
    ]

def check(total, operations, operands):
    if not operands:
        return False

    if len(operands) == 1:
        return total == operands[0]

    x, y, *rest = operands
    return any(check(total, operations, [op(x, y), *rest]) for op in operations)

operations_a = [lambda x, y: x + y, lambda x, y: x * y]
operations_b = operations_a + [lambda x, y: int(f"{x}{y}")]

part_a = sum(total for total, operands in equations if check(total, operations_a, operands))
part_b = sum(total for total, operands in equations if check(total, operations_b, operands))

print(part_a, part_b)
