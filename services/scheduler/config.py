from typing import Dict, Any

def config(prod: bool) -> Dict[str, Any]:
  if prod:
    return { "addr": "localhost", "port": 8200 }
  else:
    return { "addr": "localhost", "port": 8000 }