## Pulseline

A simple single-string system monitor for the terminal with a per-core CPU histogram, CPU, RAM and battery percentages.

When you run `pulseline` it will sleep for 500ms, collect CPU usage metrics, then prints a single string to `stdout` and exits.

Execution time will be ~520-550ms so it can be run every second on say a `tmux` status line.

You can add a newline with the `-n` flag.

Example output from an Apple M1 Max:

```
██▃▂▁      29ℂ 72ℝ 100♥
```

This output breaks down into these parts:

```
██▃▂▁      29ℂ 72ℝ 100♥
^^^^^^^^^^ ^^  ^^  ^  ^
|          |   |   |  + Charege indicator (solid = charging, hollow = discharging)
|          |   |   + Battery percentage
|          |   + RAM percentage
|          + CPU percentage, normalized to 100% for all cores
| Per-core CPU usage histogram (10 cores = 10 bars)
```
