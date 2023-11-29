import { invoke } from "@tauri-apps/api/tauri";
import styles from "./RunButton.module.css";

interface RunButtonProps {
  selectedDevice: string;
}

function RunButton({ selectedDevice }: RunButtonProps) {
  const runButtonOnClick = async () => {
    try {
      await invoke("run_voice_reader", { device: selectedDevice });
    } catch (err) {
      console.error("Failed to run:", err);
    }
  };

  const stopButtonOnClick = async () => {
    try {
      await invoke("stop_voice_reader");
    } catch (err) {
      console.error("Failed to run:", err);
    }
  };
  return (
    <>
      <button onClick={runButtonOnClick} className={styles.runButton}>
        Run
      </button>
      <button onClick={stopButtonOnClick} className={styles.stopButton}>
        Stop
      </button>
    </>
  );
}

export default RunButton;
