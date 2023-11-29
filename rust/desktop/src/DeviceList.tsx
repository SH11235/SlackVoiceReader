import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import styles from "./DeviceList.module.css";

interface DeviceListProps {
  setSelectedDevice: (deviceName: string) => void;
}

function DeviceList({ setSelectedDevice }: DeviceListProps) {
  const [deviceList, setDeviceList] = useState<string[]>([]);
  useEffect(() => {
    const loadDeviceList = async () => {
      try {
        const deviceList = await invoke<string[]>("device_list");
        setDeviceList(deviceList);
      } catch (err) {
        console.error("Failed to load device list:", err);
      }
    };

    loadDeviceList();
  }, []);

  return (
    <>
    <h2>Device List</h2>
      <ul className={styles.deviceList}>
        {deviceList.map((deviceName) => (
          <li key={deviceName} className={styles.deviceItem}>
            <input
              type="radio"
              name="device"
              id={deviceName}
              value={deviceName}
              onChange={() => setSelectedDevice(deviceName)}
              className={styles.radioInput}
            />
            <label htmlFor={deviceName}>{deviceName}</label>
          </li>
        ))}
      </ul>
    </>
  );
}

export default DeviceList;
