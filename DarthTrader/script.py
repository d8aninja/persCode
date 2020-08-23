import requests as req
# import tdma_api
# - Failed to load: libTDAmeritradeAPI.so
# - OSError: dlopen(libTDAmeritradeAPI.so, 6): image not found
# - Use tdma_api.clib.init(path) to load manually

# needs to be refreshed
access_token = "woHSjI3R2332ILIS0FuD5+CH2dt395m4cmn4hwtJqj3NhwIiqPnSfH+VpXY/J6+1q8lca6eE0eSAkZ9Jo/EeVQ3Vw3fozOETJKLq+Qb6+RhBIKCV5bEk3XJz0z9LvNf2kqfjP9xB8vrOtnv7sH39n2XHVaLCG3DtYHWDvELMh8JfmMeYrY1uLsxzAAzwOZXQxfrP4DKoogCABd2n8mSOs2dq3QJN28n4kEsXUy0gjA7+yO2UXB+Pj9WhHGIF03/mbv8sRxktVSNkiKxzDZdhAdiR14oBbn8qXTvwl6NuTTXvUrdLsLrLV3A3gN4eaVvXBrppFnmFwown9bvcbMT70SP02BviggUUGHI77a3BSx7HR9PbuuLCWwbMuuvSP18SbsrL02wZHmdQGOrhdFt2Jj8yWW/AwHawYarQSQPPXqT466JwdJuY+se5MvQEFV86Ai8R0dex7AosjGGFdn7s8MN6uzC0QXj7QalwXM9ku9dRTWrVAbkz/9NS6aziyys0JPgGnlUM8v2qXdrWx5nJbSl1ecXxqt8L9D9/iZPU3e0li4HfeVzL1OZpajRooym9/22xpVy7ATx7ltbK/1e65XezW2hyp/D8JVy1j100MQuG4LYrgoVi/JHHvlQvCnrLsv/Av0GypLu63tq/ZZVh4+lqYh62D4DrR0RYOK/zQ6q6MEONitFwpfLlj0AvEjZnEumHJQR5uKWjc/pgkFFyNDhxXSKsLWoeUPVheKfhxs9I4SuIUwzs5iXWaQAONLG2FmTQLWU6h0QwvHl01RaUB8bhOSL7X9jCAcPksb7DNclykepNbTyLzGmaX31gpGv8T5jqInX6EnrZ2/OIKvkx19Uz9SsHYvwSGr2cspkHi3/jk7cini2+4FkUK1MztxkdNosqbdS5Y49ulyKlXkhCsDbD4fGul0gkB2Irgf2sig5KHfLkYkcLCcyBfOQZE/SEYwynP9yrnVUSSdhaeuSJiMuwfDne6Ohb21vckmZcXnJ8GT1a6YGMNsgukbP6THREaI/gAEW3pO5FmmgO21mfBd9H04YYKQSLn8ROnixlvdmvMSmT6kGy2z3iOvIDDjjOvrPIJz6/UyKeJ54ya1EmOapHNpPMotKUezZkSp/gYBp1gYEuqQ7kgCQwpqziI1gxDoBVAF4YRm9+DGPoSj6NHTtkknwMIIFSdzBAool8K+4t2ygEHiTnLLyjk6jx8y6nMGVu59YbZYnsqQ32ESPrcZ8GBi0RGtY2212FD3x19z9sWBHDJACbC00B75E",

uri = "https://api.tdameritrade.com/v1/marketdata/quotes?apikey=VQ9DLHMUNMSBMDDN9EXABXQS7GWSC0GA&symbol=OIS%2COIH%2CDAL"

# not used
scope = ["PlaceTrades", "AccountAccess", "MoveMoney"]

# noteworthy
# "expires_in": 1800,
# "token_type": "Bearer"

# manual af way, via https://stackoverflow.com/questions/67631/how-to-import-a-module-given-the-full-path, and
# Requirement already satisfied: tdma_api in /Users/jvg/.pyenv/versions/3.8.2/envs/DarthTrader/lib/python3.8/site-packages (0.2)
import importlib.util
spec = importlib.util.spec_from_file_location("tdma_api", "/Users/jvg/.pyenv/versions/3.8.2/envs/DarthTrader/lib/python3.8/site-packages/tdma_api/clib.py")
tdma_api = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tdma_api)
# tdma_api.SomeClass()