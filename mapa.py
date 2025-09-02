import geopandas as gpd
import matplotlib.pyplot as plt

mapa = gpd.read_file("ProvinciasArgentina.geojson")

# Colorear provincias con colores aleatorios
mapa.plot(column=None, cmap='tab20', edgecolor='black')

plt.show()