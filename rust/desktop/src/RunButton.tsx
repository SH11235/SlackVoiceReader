import { invoke } from "@tauri-apps/api/tauri";

function RunButton() {
  const runButtonOnClick = async () => {
    try {
      await invoke("run_voice_reader");
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
      <button onClick={runButtonOnClick}>Run</button>;
      <button onClick={stopButtonOnClick}>Stop</button>;
    </>
  );
}

export default RunButton;
