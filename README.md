# 🚀 CPU Stats Monitor

A set of Rust tools for monitoring CPU usage and system temperatures, designed to be used with i3blocks/i3status.

## ✨ Features

- **CPU Monitor**: Displays real-time CPU usage with an ASCII sparkline 📊
- **Temperature Monitor**: Shows the average temperature of system sensors with an ASCII sparkline 🌡️
- Support for warning and critical thresholds ⚠️
- Data history stored in `/tmp` 📁
- Integration with i3blocks/i3status 🔄

## 🛠️ Requirements

- Rust (latest stable version)
- `sensors` (for temperature monitoring)
- i3blocks or i3status (optional)

## 📦 Installation

1. Clone the repository:
```bash
git clone https://github.com/your-username/cpu-temp.git
cd cpu-temp
```

2. Build the project:
```bash
cargo build --release
```

## 🚀 Usage

### CPU Monitor

```bash
./target/release/cpu [options]
```

Options:
- `-w, --warning <WARN>`: Warning threshold in percentage (default: 70.0) ⚠️
- `-c, --critical <CRIT>`: Critical threshold in percentage (default: 90.0) 🚨
- `-n, --count <N>`: Sparkline length (default: 20) 📊

### Temperature Monitor

```bash
./target/release/temperature [options]
```

Options:
- `-w, --warning <WARN>`: Warning threshold in degrees (default: 70.0) ⚠️
- `-c, --critical <CRIT>`: Critical threshold in degrees (default: 90.0) 🚨
- `--chip <CHIP>`: Specify the sensor chip 🔧
- `-n, --count <N>`: Sparkline length (default: 5) 📊

## 🖥️ i3blocks Configuration

Example configuration for `~/.config/i3blocks/config`:

```ini
[cpu]
command=/path/to/cpu -n 20
interval=1

[temperature]
command=/path/to/temperature --chip coretemp-isa-0000 -n 10
interval=10
```

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details. 🎉
