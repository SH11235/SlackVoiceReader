import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

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
      <ul>
        {deviceList.map((deviceName) => (
          <li key={deviceName}>
            <input
              type="radio"
              name="device"
              id={deviceName}
              value={deviceName}
              onChange={() => setSelectedDevice(deviceName)}
            />
            <label htmlFor={deviceName}>{deviceName}</label>
          </li>
        ))}
      </ul>
    </>
  );
}

export default DeviceList;
