import requests
import json


your_input = input("Votre nombre :")
r = requests.get(f"http://localhost:3000/{your_input}")

is_prime = json.loads(r.content.decode('utf-8'))["response"]

if is_prime :
    print(f"{your_input} is prime number.")
else :
    print(f"{your_input} is NOT prime number.")
