import time

# Registrar el tiempo inicial
start_time = time.time()

# Ejecutar el bucle 10,000 veces
for _ in range(10000):
   print("Hola, Mundo")


# Registrar el tiempo final
end_time = time.time()

# Calcular y mostrar la duración
execution_time = end_time - start_time
print(f"El tiempo de ejecución fue de {execution_time:.6f} segundos")