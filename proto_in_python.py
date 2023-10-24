import numpy as np
from random import *


def dice():
    return randint(1, 6)


def validate(box, val):
    if val < 0 or val >= box.size:
        return False
    if box[val - 1]:
        return False
    else:
        return True


def visualize(box):
    for i in range(9):
        if box[i]:
            print("| x ", end="")
            if i >= 9:
                print(" ", end="")
        else:
            print("|   ", end="")
            if i >= 9:
                print(" ", end="")

    print("|")

    for i in range(9):
        print("| " + str(i+1) + " ", end="")
    print("|")


def check_combinations(box, value):
    box_total = 0
    for i in range(0, min(value, 9)):
        if not box[i]:
            box_total += i+1
    if value <= box_total:
        return False
    else:
        return True


def calc_score(box):
    box_total = 0
    for i in range(0, 9):
        if not box[i]:
            box_total += i+1
    return box_total


#----------------------------------------------------------------------------------------------------------------------

box = np.full(9, False)
lost = False


while False in box and not lost:
    visualize(box)
    d1 = str(dice())
    d2 = str(dice())
    total = int(d1)+int(d2)
    print("You threw a " + d1 + " and a " + d2 + ".\n The total is: " + str(total))
    subtotal = 0

    while subtotal != total:
        if check_combinations(box, total-subtotal):
            lost = True
            break
        input1 = int(input("Please enter your number \n"))
        if not validate(box, input1):
            print("Not accepted, please try again")
        elif input1 > total-subtotal or input1 > 9:
            print("Values too high")
        else:
            subtotal += input1
            box[input1 - 1] = True
            if subtotal != total:
                visualize(box)

if lost:
    print("You Lost!")
else:
    print("You won!")
print("Your Score is: " + str(calc_score(box)))