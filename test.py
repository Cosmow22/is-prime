import requests
import json
from time import time


your_input = input("Votre nombre :")

t1 = time()
r = requests.get(f"http://localhost:3000/{your_input}")
t2 = time()
is_prime = json.loads(r.content.decode('utf-8'))["response"]

if is_prime :
    print(f"{your_input} is prime number.")
else :
    print(f"{your_input} is NOT prime number.")
print(f"Temps écoulé : {t2-t1}")
