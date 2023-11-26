import "./App.css";
import SettingsForm, { Settings } from "./SettingsForm";
import { useState } from "react";

function App() {
  const [settings, setSettings] = useState<Settings>({
    slackToken: "",
    threadUrl: "",
    voicevoxUrl: "",
    speakerStyleId: "",
  });
  return (
    <div className="App">
      <SettingsForm settings={settings} setSettings={setSettings} />
    </div>
  );
}

export default App;
