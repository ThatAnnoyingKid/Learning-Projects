import math
import random
lower = 0
higher = 0
guesses = 5
userGuess = 0
randNum = 0

while True:
    lower = int(input("What is the lower number? "))
    if lower < 0:
        print("Your number is too low")
        
    else:
        break
    
while True:
    higher = int(input("What is the higher number? "))
    if higher < lower:
        print("Your number is too low")
        
    else:
        break

randNum = random.randint(lower, higher)

print("The range is between " + str(lower) + " and " + str(higher))
while guesses > 0:
    print("You have " + str(guesses) + " guesses left")
    userGuess = int(input("What is your guess? "))
    guesses -= 1
    if userGuess < randNum:
        print("Your guess is too low")
        
    elif userGuess > randNum:
        print("Your guess is too high")
        
    else:
        print("You guessed the number")
        break
    
    if guesses == 0:
        print("You did not guess the number")
        print("The number was " + str(randNum))
        break