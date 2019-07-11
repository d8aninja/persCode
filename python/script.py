from nose.tools import assert_true
import requests

def test_request_response():
    url = 'http://jsonplaceholder.typicode.com/todos'
    response = requests.get(url)
    assert_true(response.ok)

# my guess implementation
# def server_alive(url):
#     get_request = requests.get(url)
#     print(*[val for dic in get_request.json() for val in dic.values()], sep='\n')

# if __name__ == "__main__":
#     url = 'http://jsonplaceholder.typicode.com/todos'
#     # server_alive(url)
#     test_request_response(url)

