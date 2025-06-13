# CPU Stats Monitor

Un conjunto de herramientas en Rust para monitorear el uso de CPU y temperaturas del sistema, diseñado para ser utilizado con i3blocks/i3status.

## Características

- **Monitor de CPU**: Muestra el uso de CPU en tiempo real con un gráfico ASCII (sparkline)
- **Monitor de Temperatura**: Muestra la temperatura promedio de los sensores del sistema con un gráfico ASCII
- Soporte para umbrales de advertencia y críticos
- Historial de datos almacenado en `/tmp`
- Integración con i3blocks/i3status

## Requisitos

- Rust (última versión estable)
- `sensors` (para el monitor de temperatura)
- i3blocks o i3status (opcional)

## Instalación

1. Clona el repositorio:
```bash
git clone https://github.com/tu-usuario/cpu-temp.git
cd cpu-temp
```

2. Compila el proyecto:
```bash
cargo build --release
```

## Uso

### Monitor de CPU

```bash
./target/release/cpu [opciones]
```

Opciones:
- `-w, --warning <WARN>`: Umbral de advertencia en porcentaje (default: 70.0)
- `-c, --critical <CRIT>`: Umbral crítico en porcentaje (default: 90.0)
- `-n, --count <N>`: Longitud del sparkline (default: 20)

### Monitor de Temperatura

```bash
./target/release/temperature [opciones]
```

Opciones:
- `-w, --warning <WARN>`: Umbral de advertencia en grados (default: 70.0)
- `-c, --critical <CRIT>`: Umbral crítico en grados (default: 90.0)
- `--chip <CHIP>`: Especificar el chip del sensor
- `-n, --count <N>`: Longitud del sparkline (default: 5)

## Configuración para i3blocks

Ejemplo de configuración para `~/.config/i3blocks/config`:

```ini
[cpu]
command=/ruta/a/cpu -n 20
interval=1

[temperature]
command=/ruta/a/temperature --chip coretemp-isa-0000 -n 10
interval=10
```

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo LICENSE para más detalles.
