def sum_two_smallest_numbers(numbers: list) -> list:
     numbers.sort() # surprises the user by changing data
     return sum(numbers[:2])

def min_max(lst):
  lst.sort() # surprises the user by changing data
  result = [lst[0], lst[-1]]
  return result

def calculate_years(principal, interest, tax, desired):
  counter = 0
  while principal < desired:
    principal = principal + ((principal * interest) * (1 - tax))
    counter += 1
  return counter

def likes(names):
    if len(names) == 0:
        print("no one likes this")
    if len(names) == 1:
        print(names[0] + " likes this")
    elif len(names) == 2:
        print(names[0] + " and " + names[1] + " like this")
    elif len(names) == 3:
        print(names[0] + ", " + names[1] + " and " + names[2] + " like this")
    elif len(names) >= 4:
        print(str(names[0]) + ", " + str(names[1]) + " and " + str(len(names) - 2) + " others like this")

if __name__ == "__main__":
    numList = [2,5,1,4,3]
    sum_smallest_2 = sum_two_smallest_numbers(numList)
    mix_max_result = min_max(numList)
    
    value = calculate_years(1000,0.01625,0.18,1000)

    likes([])
    likes(['Peter'])
    likes(['Jacob', 'Alex'])
    likes(['Alex', 'Jacob', 'Mark', 'Max'])
    likes(['Linna Yamazaki', 'Nigel', 'Priscilla S. Asagiri', 'Brian J. Mason', 'Sylia Stingray', 'Daley Wong', 'Anri', 'Macky Stingray', 'Leon McNichol', 'Quincy Rosenkreutz', 'Largo'])

    print(sum_smallest_2)
    print(mix_max_result)
    print(value)
