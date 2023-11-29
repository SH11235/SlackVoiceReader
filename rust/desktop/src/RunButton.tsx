import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import styles from "./RunButton.module.css";

interface RunButtonProps {
  selectedDevice: string;
}

function RunButton({ selectedDevice }: RunButtonProps) {
  const [isRunning, setIsRunning] = useState(false);

  const runButtonOnClick = async () => {
    try {
      setIsRunning(true);
      await invoke("run_voice_reader", { device: selectedDevice });
    } catch (err) {
      console.error("Failed to run:", err);
    }
  };

  const stopButtonOnClick = async () => {
    try {
      setIsRunning(false);
      await invoke("stop_voice_reader");
    } catch (err) {
      console.error("Failed to run:", err);
    }
  };
  return (
    <>
      <button onClick={runButtonOnClick} className={styles.runButton} disabled={isRunning}>
        Run
      </button>
      <button onClick={stopButtonOnClick} className={styles.stopButton} disabled={!isRunning}>
        Stop
      </button>
    </>
  );
}

export default RunButton;
