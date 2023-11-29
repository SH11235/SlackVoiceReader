import "./App.css";
import SettingsForm from "./SettingsForm";
import RunButton from "./RunButton";
import DeviceList from "./DeviceList";
import { useState } from "react";

function App() {
  const [selectedDevice, setSelectedDevice] = useState<string>("");
  return (
    <div className="App">
      <SettingsForm />
      <DeviceList setSelectedDevice={setSelectedDevice} />
      <RunButton selectedDevice={selectedDevice} />
    </div>
  );
}

export default App;
