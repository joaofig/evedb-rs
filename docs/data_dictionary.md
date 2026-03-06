# Data Dictionary

This document provides a detailed description of the tables, columns, and units in the `evedb` SQLite database.

## Tables Overview

The database consists of the following main tables:

1.  [`vehicle`](#vehicle-table): Metadata for each unique vehicle.
2.  [`signal`](#signal-table): High-frequency sensor and GPS data for all trips.
3.  [`trajectory`](#trajectory-table): Aggregated trip-level data (derived from `signal`).
4.  [`node`](#node-table): Map-matched road network nodes (generated via `match` command).

---

## `vehicle` Table

Contains static information about the vehicles in the dataset.

| Column Name | Type | Description |
| :--- | :--- | :--- |
| `vehicle_id` | INTEGER | Primary Key. Unique identifier for the vehicle. |
| `vehicle_type` | TEXT | Type of vehicle (e.g., ICE, PHEV, EV, HEV). |
| `vehicle_class` | TEXT | Vehicle classification (e.g., Sedan, SUV). |
| `engine` | TEXT | Engine type or configuration. |
| `transmission` | TEXT | Transmission type. |
| `drive_wheels` | TEXT | Drive wheel configuration (e.g., FWD, AWD). |
| `weight` | INTEGER | Vehicle weight in kilograms (kg). |

---

## `signal` Table

The core table containing time-series data for vehicle signals.

| Column Name | Type | Description | Units |
| :--- | :--- | :--- | :--- |
| `signal_id` | INTEGER | Primary Key. | - |
| `day_num` | DOUBLE | Day number in the dataset. | Day |
| `vehicle_id` | INTEGER | Foreign Key to `vehicle`. | - |
| `trip_id` | INTEGER | Identifier for a specific trip. | - |
| `time_stamp` | INTEGER | Time elapsed since the start of the trip. | milliseconds (ms) |
| `latitude` | DOUBLE | Raw GPS latitude. | decimal degrees |
| `longitude` | DOUBLE | Raw GPS longitude. | decimal degrees |
| `speed` | DOUBLE | Vehicle speed. | km/h |
| `maf` | DOUBLE | Mass Air Flow. | g/sec |
| `rpm` | DOUBLE | Engine rotations per minute. | RPM |
| `abs_load` | DOUBLE | Absolute engine load. | % |
| `oat` | DOUBLE | Outside Air Temperature. | °C |
| `fuel_rate` | DOUBLE | Fuel consumption rate. | L/hr |
| `ac_power_kw` | DOUBLE | Air Conditioning power consumption (kW). | kW |
| `ac_power_w` | DOUBLE | Air Conditioning power consumption (Watts). | Watts |
| `heater_power_w` | DOUBLE | Heater power consumption. | Watts |
| `hv_bat_current` | DOUBLE | High Voltage battery current. | Amperes (A) |
| `hv_bat_soc` | DOUBLE | High Voltage battery State of Charge. | % |
| `hv_bat_volt` | DOUBLE | High Voltage battery voltage. | Volts (V) |
| `st_ftb_1` | DOUBLE | Short Term Fuel Trim Bank 1. | % |
| `st_ftb_2` | DOUBLE | Short Term Fuel Trim Bank 2. | % |
| `lt_ftb_1` | DOUBLE | Long Term Fuel Trim Bank 1. | % |
| `lt_ftb_2` | DOUBLE | Long Term Fuel Trim Bank 2. | % |
| `elevation` | DOUBLE | Raw elevation. | meters (m) |
| `elevation_smooth` | DOUBLE | Smoothed elevation. | meters (m) |
| `gradient` | DOUBLE | Road gradient/slope. | % (or ratio) |
| `energy_consumption` | DOUBLE | Energy consumption (eVED derived). | kWh (approx) |
| `match_latitude` | DOUBLE | Matched latitude (from eVED source). | decimal degrees |
| `match_longitude` | DOUBLE | Matched longitude (from eVED source). | decimal degrees |
| `match_type` | INTEGER | Type of map match. | - |
| `speed_limit_type` | INTEGER | Category of speed limit. | - |
| `speed_limit` | TEXT | Road speed limit. | km/h |
| `speed_limit_direct` | INTEGER | Speed limit directionality. | - |
| `intersection` | INTEGER | Boolean flag for intersection (0/1). | - |
| `bus_stop` | INTEGER | Boolean flag for bus stop (0/1). | - |
| `focus_points` | TEXT | Additional location metadata. | - |
| `h3_12` | INTEGER | H3 index at resolution 12 (i64). | - |

---

## `trajectory` Table

Summarized data for each unique trip (vehicle + trip ID).

| Column Name | Type | Description | Units |
| :--- | :--- | :--- | :--- |
| `traj_id` | INTEGER | Primary Key. | - |
| `vehicle_id` | INTEGER | Foreign Key to `vehicle`. | - |
| `trip_id` | INTEGER | Original trip identifier from source data. | - |
| `length_m` | DOUBLE | Total trajectory length. | meters (m) |
| `dt_ini` | TEXT | Start timestamp (ISO format). | - |
| `dt_end` | TEXT | End timestamp (ISO format). | - |
| `duration_s` | DOUBLE | Total trip duration. | seconds (s) |
| `h3_12_ini` | INTEGER | H3 resolution 12 index of the start point. | - |
| `h3_12_end` | INTEGER | H3 resolution 12 index of the end point. | - |

---

## `node` Table

Contains nodes from the road network that were map-matched to a trajectory.

| Column Name | Type | Description |
| :--- | :--- | :--- |
| `node_id` | INTEGER | Primary Key. |
| `traj_id` | INTEGER | Foreign Key to `trajectory`. |
| `latitude` | DOUBLE | Matched node latitude. |
| `longitude` | DOUBLE | Matched node longitude. |
| `h3_12` | INTEGER | H3 resolution 12 index of the node. |
| `match_error` | TEXT | Error message if map-matching failed for this trajectory. |
