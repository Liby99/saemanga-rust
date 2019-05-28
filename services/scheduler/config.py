from typing import Dict

def config(prod: bool) -> Dict[str, str]:
  if prod:
    return {
      "addr": "localhost",
      "port": 8193
    }
  else:
    return {
      "addr": "localhost",
      "port": 8000
    }