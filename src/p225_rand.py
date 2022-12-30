import random

stack = []
for i in range(1000):
    decider = random.randint(0, 2)
    if len(stack) == 0 or decider == 0:
        item = random.randint(0, 1e8)
        print(f'stack.push({item});')
        stack.append(item)
    elif decider == 1:
        print(f'assert_eq!(stack.top(), {stack[-1]});')
    else:
        print(f'assert_eq!(stack.pop(), {stack[-1]});')
        stack.pop()
