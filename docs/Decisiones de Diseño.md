# Decisiones de Diseño

## Responsibilidad del servidor

### Con verificación

#### Ventajas
 - Reconexión sin pérdida. El servidor al mantener el estado total de la partida, es capaz de proveerle su último estado válido a un jugador luego de perder la conexión TCP con el server.

#### Desventajas
 - Mayor complejidad... probablemente.

### Sin verificación

#### Ventajas
 - Menor complejidad.

#### Desventajas
 - Reconexión con pérdida. Al volver a la ronda no es posible obtener nuevas cartas hasta pasar a la siguiente mano.