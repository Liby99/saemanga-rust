from typing import Dict, Any

def config(prod: bool) -> Dict[str, Any]:
  return {
    "addr": "localhost",
    "port": 8200 if prod else 8000
  }