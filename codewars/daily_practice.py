# number of odds under n, plus return the list for good measure
def return_odds(n: int) -> (int, list):
    out_list = [i for i in range(n) if i % 2 == 1]
    return_tuple = (len(out_list), out_list)
    print(return_tuple)
    return_tuple
    

if __name__ == "__main__":
    return_odds(10) # should return (5, [1,3,5,7,9])